use crate::AttributeMul;
use crate::utils::resize_account;
use crate::StakePool;
use crate::ATTRIBUTE_MUL_DEFAULT_SIZE;
use crate::ATTRIBUTE_MUL_PREFIX;
use anchor_lang::prelude::*;
use crate::errors::ErrorCode;

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct InitAttributeMulIx {
    attribute_multiply: Vec<Vec<String>>,
    multiply_data: Vec<Vec<f32>>,
    identifier: String,
    authority: Pubkey,
}

#[derive(Accounts)]
#[instruction(ix: InitAttributeMulIx)]
pub struct InitAttributeMulCtx<'info> {

    #[account(
        init,
        payer = payer,
        space = ATTRIBUTE_MUL_DEFAULT_SIZE,
        seeds = [ATTRIBUTE_MUL_PREFIX.as_bytes(), ix.identifier.as_ref()],
        bump
    )]
    attribute_mul: Account<'info, AttributeMul>,

    #[account(mut, constraint = payer.key() == stake_pool.authority)]
    stake_pool: Box<Account<'info, StakePool>>,

    #[account(mut)]
    payer: Signer<'info>,
    system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<InitAttributeMulCtx>,
    ix: InitAttributeMulIx
) -> Result<()> {
    // attribute = prob & amount
    if ix.attribute_multiply.len() != ix.multiply_data.len(){
        return Err(error!(ErrorCode::NotSameAttritubeData));
    }

    let bump: u8 = *ctx.bumps.get("attribute_mul").unwrap();

    let mut new_attribute_mul = AttributeMul {
        attribute_multiply: ix.attribute_multiply,
        multiply_amount: vec![],
        multiply_prob: vec![],
        bump: bump,
        authority: ix.authority,
    };

    for data in ix.multiply_data.iter() {
        // prob & amount = 2
        if data.len() != 2 {
            return Err(error!(ErrorCode::NotSameProbAmount));
        }
        new_attribute_mul.multiply_amount.push(data[0] as f32);
        new_attribute_mul.multiply_prob.push(data[1] as u32);
    }

    let attribute_mul = &mut ctx.accounts.attribute_mul;
    let new_space = new_attribute_mul.try_to_vec()?.len() + 8;

    resize_account(
        &attribute_mul.to_account_info(), 
        new_space, 
        &ctx.accounts.payer.to_account_info(), 
        &ctx.accounts.system_program.to_account_info(),
    )?;

    attribute_mul.set_inner(new_attribute_mul);

    Ok(())
}