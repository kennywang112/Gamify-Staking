use crate::utils::resize_account;
use crate::StakePool;
use crate::Discount;
use crate::STAKE_POOL_DEFAULT_SIZE;
use crate::STAKE_POOL_PREFIX;
use anchor_lang::prelude::*;
use std::str::FromStr;
use crate::errors::ErrorCode;
// use anchor_spl::token::{self, Token, TokenAccount, Transfer as SplTransfer, Mint};
use anchor_spl::token::{Token, TokenAccount, Mint};

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct InitPoolIx {
    allowed_collections: Vec<Pubkey>,
    allowed_creators: Vec<Pubkey>,
    requires_authorization: bool,
    authority: Pubkey,
    reset_on_unstake: bool,
    cooldown_seconds: Option<u32>,
    min_stake_seconds: Option<u32>,
    end_date: Option<i64>,
    stake_payment_info: Pubkey,
    unstake_payment_info: Pubkey,
    identifier: String,
}

#[derive(Accounts)]
#[instruction(ix: InitPoolIx)]
pub struct InitPoolCtx<'info> {
    #[account(
        init,
        payer = payer,
        space = STAKE_POOL_DEFAULT_SIZE,
        seeds = [STAKE_POOL_PREFIX.as_bytes(), ix.identifier.as_ref()],
        bump
    )]
    stake_pool: Account<'info, StakePool>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(
        mut,
        constraint = owner.key() == Pubkey::from_str("2JeNLSrJkSaWoFoSQkb1YsxC1dXSaA1LTLjpakzb9SBf").unwrap() 
    )]
    owner: AccountInfo<'info>,

    //this is for spl-token payment
    // #[account(mut, constraint = usdc_mint.key() == Pubkey::from_str("D8J6gcTSLPwXS9h4afZvDEQr2qGxscVfUPnrfbHQxhzJ").unwrap())]
    // usdc_mint: Account<'info, Mint>,
    // #[account(mut)]
    // payer_mint_token_account: Box<Account<'info, TokenAccount>>,
    // #[account(mut, constraint = owner_mint_token_account.owner.key() == owner.key())]
    // owner_mint_token_account: Box<Account<'info, TokenAccount>>,

    token_program: Program<'info, Token>,

    #[account(mut)]
    payer: Signer<'info>,
    system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<InitPoolCtx>, ix: InitPoolIx) -> Result<()> {
    let bump = *ctx.bumps.get("stake_pool").unwrap();
    let identifier = ix.identifier;
    let new_stake_pool = StakePool {
        bump,
        authority: ix.authority,
        total_staked: 0,
        reset_on_unstake: ix.reset_on_unstake,
        cooldown_seconds: ix.cooldown_seconds,
        min_stake_seconds: ix.min_stake_seconds,
        end_date: ix.end_date,
        stake_payment_info: ix.stake_payment_info,
        unstake_payment_info: ix.unstake_payment_info,
        requires_authorization: ix.requires_authorization,
        allowed_creators: ix.allowed_creators,
        allowed_collections: ix.allowed_collections,
        identifier,
    };

    // added for discount payment
    let remaining_accounts = &mut ctx.remaining_accounts.iter();
    let discount = next_account_info(remaining_accounts)?;
    let discount_data = match Account::<Discount>::try_from(discount) {
        Ok(record) => record,
        Err(_) => return Err(error!(ErrorCode::FailedToMatchRemaining)),
    };

    let mut lamports: u64 = 25_000_000_000; // 25 sol/ 25000 spl
    lamports = lamports.checked_mul(discount_data.promotion_collection_mul).unwrap().checked_div(100).unwrap();

    // sol transfer
    let ix = anchor_lang::solana_program::system_instruction::transfer(
        &ctx.accounts.payer.key(),
        &ctx.accounts.owner.key(),
        lamports,
    );
    anchor_lang::solana_program::program::invoke(
        &ix,
        &[
            ctx.accounts.payer.to_account_info(),
            ctx.accounts.owner.to_account_info(),
        ],
    )?;


    // spl token transfer
    // let cpi_accounts = SplTransfer {
    //     from: ctx.accounts.payer_mint_token_account.to_account_info().clone(),
    //     to: ctx.accounts.owner_mint_token_account.to_account_info().clone(),
    //     authority: ctx.accounts.payer.to_account_info().clone(),
    // };
    // let cpi_program = ctx.accounts.token_program.to_account_info();
    
    // token::transfer(
    //     CpiContext::new(cpi_program, cpi_accounts),
    //     lamports)?;

    let stake_pool = &mut ctx.accounts.stake_pool;
    let new_space = new_stake_pool.try_to_vec()?.len() + 8;

    resize_account(
        &stake_pool.to_account_info(),
        new_space,
        &ctx.accounts.payer.to_account_info(),
        &ctx.accounts.system_program.to_account_info(),
    )?;

    stake_pool.set_inner(new_stake_pool);
    Ok(())
}
