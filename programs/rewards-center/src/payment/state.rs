use crate::errors::ErrorCode;
use anchor_lang::prelude::*;
use anchor_lang::Result;
use anchor_spl::token::{self ,TokenAccount, Transfer};
use solana_program::program::invoke;
use solana_program::system_instruction::transfer;
use solana_program::system_program;
use std::cmp::Eq;
use std::slice::Iter;
use std::str::FromStr;

pub const BASIS_POINTS_DIVISOR: u64 = 10_000;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Eq, PartialEq)]
pub struct PaymentShare {
    pub address: Pubkey,
    pub basis_points: u16,
}

// this gets resized on add and remove of shares
pub const DEFAULT_PAYMENT_INFO_SIZE: usize = 8 + std::mem::size_of::<PaymentInfo>();
pub const PAYMENT_INFO_PREFIX: &str = "payment-info";
#[account]
pub struct PaymentInfo {
    pub bump: u8,
    pub authority: Pubkey,
    pub identifier: String,
    pub payment_amount: u64,
    pub payment_mint: Pubkey,
    pub payment_shares: Vec<PaymentShare>,
}

#[derive(Clone, Copy)]
pub enum Action {
    Stake = 0,
    Unstake,
    ClaimRewards,
    ClaimRewardReceipt,
    BoostStakeEntry,
}

pub fn assert_payment_info(stake_pool: Pubkey, action: Action, payment_info: Pubkey) -> Result<()> {
    let default_allowed_payment_infos = match action {
        _ => [
            "DQo6cj9Ak2rrxBSvs7njKfxzhWPgxrauf1Q9D5mqn8Nk".to_string()//haco
        ]
        .to_vec(),
    };
    let allowed_payment_infos = match (stake_pool.key().to_string().as_str(), action) {
        ("2sV8TZaJCUbXbHrgngHsYrxHSHTJmLdFS8BPqhhuThmi", Action::Stake) => ["DQo6cj9Ak2rrxBSvs7njKfxzhWPgxrauf1Q9D5mqn8Nk".to_string()].to_vec(),
        ("2sV8TZaJCUbXbHrgngHsYrxHSHTJmLdFS8BPqhhuThmi", Action::Unstake) => ["DQo6cj9Ak2rrxBSvs7njKfxzhWPgxrauf1Q9D5mqn8Nk".to_string()].to_vec(),

        _ => default_allowed_payment_infos,
    };
    if !allowed_payment_infos.contains(&payment_info.to_string()) {
        return Err(error!(ErrorCode::InvalidPaymentInfo));
    }
    Ok(())
}

pub fn handle_payment_info<'info>(
    remaining_accounts: &mut Iter<AccountInfo<'info>>
) -> Result<()> {
    // let payment_info = Pubkey::from_str("DQo6cj9Ak2rrxBSvs7njKfxzhWPgxrauf1Q9D5mqn8Nk").unwrap();

    // check payment info
    let payment_info_account_info = next_account_info(remaining_accounts)?;
    // assert_eq!(payment_info, payment_info_account_info.key());
    let payment_info_account = Account::<PaymentInfo>::try_from(payment_info_account_info)?;
    // check amount
    if payment_info_account.payment_amount == 0 {
        return Ok(());
    }
    handle_payment(
        payment_info_account.payment_amount,
        payment_info_account.payment_mint,
        &payment_info_account.payment_shares,
        remaining_accounts,
    )
}

pub fn handle_payment<'info>(
    payment_amount: u64, 
    payment_mint: Pubkey, 
    payment_shares: &Vec<PaymentShare>, 
    remaining_accounts: &mut Iter<AccountInfo<'info>>
) -> Result<()> {
    let payer = next_account_info(remaining_accounts)?;
    let transfer_program: &AccountInfo = if payment_mint == Pubkey::default() {
        let transfer_program = next_account_info(remaining_accounts)?;
        if !system_program::check_id(&transfer_program.key()) {

            return Err(error!(ErrorCode::InvalidTransferProgram));
        }
        transfer_program
    } else {

        let transfer_program = next_account_info(remaining_accounts)?;
        if transfer_program.key() != token::ID {
            return Err(error!(ErrorCode::InvalidTransferProgram));
        }
        transfer_program
    };

    // payer token account if needed
    let mut payer_token_account: Option<Account<TokenAccount>> = None;
    if payment_mint != Pubkey::default() {

        let payer_token_account_info = next_account_info(remaining_accounts)?;
        let payer_token_account_data = Account::<TokenAccount>::try_from(payer_token_account_info)?;
        if payer_token_account_data.owner != payer.key() || payer_token_account_data.mint != payment_mint.key() {
            return Err(error!(ErrorCode::InvalidPayerTokenAccount));
        }
        payer_token_account = Some(payer_token_account_data);
    }

    let collectors = &payment_shares;
    let share_amounts: Vec<u64> = collectors
        .iter()
        .map(|s| payment_amount.checked_mul(u64::try_from(s.basis_points).expect("Could not cast u8 to u64")).unwrap())
        .collect();
    let share_amounts_sum: u64 = share_amounts.iter().sum();

    // remainder is distributed to first collectors
    let mut remainder = payment_amount.checked_sub(share_amounts_sum.checked_div(BASIS_POINTS_DIVISOR).expect("Div error")).expect("Sub error");
    for payment_share in payment_shares {

        if payment_share.basis_points != 0 {

            let remainder_amount = u64::from(remainder > 0);
            let payment_share_amount = payment_amount
                .checked_mul(u64::try_from(payment_share.basis_points).expect("Could not cast u8 to u64"))
                .unwrap()
                .checked_div(BASIS_POINTS_DIVISOR)
                .expect("Div error")
                .checked_add(remainder_amount) // add remainder amount
                .expect("Add error");
            remainder = remainder.checked_sub(remainder_amount).expect("Sub error");

            let payment_share_account_info = next_account_info(remaining_accounts)?;

            if payment_mint == Pubkey::default() {

                // native sol
                if payment_share_account_info.key() != payment_share.address {
                    return Err(error!(ErrorCode::InvalidPaymentShare));
                }

                if payment_share_amount > 0 {
                    invoke(
                        &transfer(&payer.key(), &payment_share_account_info.key(), payment_share_amount),
                        &[payer.to_account_info(), payment_share_account_info.to_account_info(), transfer_program.to_account_info()],
                    )?;
                }

            } else {

                // any spl token
                let payment_share_token_account = Account::<TokenAccount>::try_from(payment_share_account_info)?;
                if payment_share_token_account.owner != payment_share.address || payment_share_token_account.mint != payment_mint.key() {
                    return Err(error!(ErrorCode::InvalidTokenAccount));
                }

                if payment_share_amount > 0 {
                    let cpi_accounts = Transfer {
                        from: payer_token_account.clone().expect("Invalid payer token account").to_account_info(),
                        to: payment_share_account_info.to_account_info(),
                        authority: payer.to_account_info(),
                    };
                    let cpi_context = CpiContext::new(transfer_program.to_account_info(), cpi_accounts);
                    token::transfer(cpi_context, payment_share_amount)?;
                }
            }
        }
    }

    Ok(())
}

