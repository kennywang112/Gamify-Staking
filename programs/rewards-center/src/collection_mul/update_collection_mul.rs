use crate::CollectionMul;
use crate::utils::resize_account;
use anchor_lang::prelude::*;
use crate::errors::ErrorCode;

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct UpdateCollectionMulIx {
    collections_multiply: Vec<Pubkey>,
    multiply_data: Vec<Vec<f32>>,
    identifier: String,
    authority: Pubkey,
}

#[derive(Accounts)]
pub struct UpdateCollectionMulCtx<'info> {

    #[account(mut, constraint = collection_mul.authority == authority.key())]
    collection_mul: Box<Account<'info, CollectionMul>>,

    authority: Signer<'info>,

    #[account(mut)]
    payer: Signer<'info>,
    system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<UpdateCollectionMulCtx>,
    ix: UpdateCollectionMulIx
) -> Result<()> {

    if ix.collections_multiply.len() != ix.multiply_data.len() {
        return Err(error!(ErrorCode::NotSameCollectionData));
    }

    let collection_mul = &mut ctx.accounts.collection_mul;

    let mut new_collection_mul = CollectionMul {
        collections_multiply: ix.collections_multiply,
        multiply_amount: vec![],
        multiply_prob: vec![],
        bump: collection_mul.bump,
        authority: ix.authority,
    };

    for data in ix.multiply_data.iter() {

        if data.len() != 2 {
            return Err(error!(ErrorCode::NotSameProbAmount));
        }
        new_collection_mul.multiply_amount.push(data[0] as f32);
        new_collection_mul.multiply_prob.push(data[1] as u32);
    }


    let new_space = new_collection_mul.try_to_vec()?.len() + 8;
    collection_mul.set_inner(new_collection_mul);

    resize_account(
        &collection_mul.to_account_info(), 
        new_space, 
        &ctx.accounts.payer.to_account_info(), 
        &ctx.accounts.system_program.to_account_info(),
    )?;

    Ok(())
}