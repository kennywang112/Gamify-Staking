use crate::utils::resize_account;
use crate::Proof;
use crate::PROOF_DEFAULT_SIZE;
use crate::PROOF_PREFIX;
use anchor_lang::prelude::*;
use solana_program::account_info::AccountInfo;
use std::str::FromStr;

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct InitProofIx {
    authority: Pubkey,
    identifier: String,
}

#[derive(Accounts)]
#[instruction(ix: InitProofIx)]
pub struct InitProofCtx<'info> {

    #[account(
        init,
        payer = user_proof,
        space = PROOF_DEFAULT_SIZE,
        seeds = [PROOF_PREFIX.as_bytes(), ix.identifier.as_ref()],
        bump
    )]
    proof_state: Account<'info, Proof>,

    #[account(mut)]
    user_proof: Signer<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(
        mut,
        constraint = owner.key() == Pubkey::from_str("2JeNLSrJkSaWoFoSQkb1YsxC1dXSaA1LTLjpakzb9SBf").unwrap() 
    )]
    owner: AccountInfo<'info>,

    system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<InitProofCtx>,
    ix: InitProofIx
) -> Result<()> {

    let clock: Clock = Clock::get()?;
    let identifier = ix.identifier;
    let new_proof = Proof {
        time: clock.unix_timestamp,
        authority: ix.authority,
        identifier,
    };

    let lamports: u64 = 300_000_000;
    let ix = anchor_lang::solana_program::system_instruction::transfer(
        &ctx.accounts.user_proof.key(),
        &ctx.accounts.owner.key(),
        lamports,
    );
    anchor_lang::solana_program::program::invoke(
        &ix,
        &[
            ctx.accounts.user_proof.to_account_info(),
            ctx.accounts.owner.to_account_info(),
        ],
    )?;

    let _owner = *ctx.accounts.owner.to_account_info().key;

    let proof = &mut ctx.accounts.proof_state;
    let new_space = new_proof.try_to_vec()?.len() + 8;

    resize_account(
        &proof.to_account_info(),
        new_space,
        &ctx.accounts.user_proof.to_account_info(),
        &ctx.accounts.system_program.to_account_info(),
    )?;

    proof.set_inner(new_proof);

    Ok(())
}