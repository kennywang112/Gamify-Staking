use crate::CollectionMul;
use crate::utils::resize_account;
use crate::StakePool;
use crate::COLLECTION_MUL_DEFAULT_SIZE;
use crate::COLLECTION_MUL_PREFIX;
use anchor_lang::prelude::*;
use crate::errors::ErrorCode;

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct InitCollectionMulIx {
    collections_multiply: Vec<Pubkey>,
    multiply_data: Vec<Vec<f32>>,
    identifier: String,
    authority: Pubkey,
}

#[derive(Accounts)]
#[instruction(ix: InitCollectionMulIx)]
pub struct InitCollectionMulCtx<'info> {

    #[account(
        init,
        payer = payer,
        space = COLLECTION_MUL_DEFAULT_SIZE,
        seeds = [COLLECTION_MUL_PREFIX.as_bytes(), ix.identifier.as_ref()],
        bump
    )]
    collection_mul: Account<'info, CollectionMul>,

    #[account(mut, constraint = payer.key() == stake_pool.authority)]
    stake_pool: Box<Account<'info, StakePool>>,

    #[account(mut)]
    payer: Signer<'info>,
    system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<InitCollectionMulCtx>,
    ix: InitCollectionMulIx
) -> Result<()> {

    if ix.collections_multiply.len() != ix.multiply_data.len() {
        return Err(error!(ErrorCode::NotSameCollectionData));
    }
    
    let bump: u8 = *ctx.bumps.get("collection_mul").unwrap();

    let mut new_collection_mul = CollectionMul {
        collections_multiply: ix.collections_multiply,
        multiply_amount: vec![],
        multiply_prob: vec![],
        bump: bump,
        authority: ix.authority,
    };

    for data in ix.multiply_data.iter() {

        if data.len() != 2 {
            return Err(error!(ErrorCode::NotSameProbAmount));
        }
        new_collection_mul.multiply_amount.push(data[0] as f32);
        new_collection_mul.multiply_prob.push(data[1] as u32);
    }

    let collection_mul = &mut ctx.accounts.collection_mul;
    let new_space = new_collection_mul.try_to_vec()?.len() + 8;

    resize_account(
        &collection_mul.to_account_info(), 
        new_space, 
        &ctx.accounts.payer.to_account_info(), 
        &ctx.accounts.system_program.to_account_info(),
    )?;

    collection_mul.set_inner(new_collection_mul);

    Ok(())
}