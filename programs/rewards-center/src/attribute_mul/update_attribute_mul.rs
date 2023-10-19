use crate::AttributeMul;
use crate::utils::resize_account;
use anchor_lang::prelude::*;
use crate::errors::ErrorCode;
#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct UpdateAttributeMulIx {
    attribute_multiply: Vec<Vec<String>>,
    multiply_data: Vec<Vec<f32>>,
    identifier: String,
    authority: Pubkey,
}

#[derive(Accounts)]
#[instruction(ix: UpdateAttributeMulIx)]
pub struct UpdateAttributeMulCtx<'info> {

    #[account(mut, constraint = ix.authority.key() == attribute_mul.authority)]
    attribute_mul: Account<'info, AttributeMul>,

    #[account(mut)]
    payer: Signer<'info>,
    system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<UpdateAttributeMulCtx>,
    ix: UpdateAttributeMulIx
) -> Result<()> {

    if ix.attribute_multiply.len() != ix.multiply_data.len() {
        return Err(error!(ErrorCode::NotSameAttritubeData));
    }
    
    let attribute_mul = &mut ctx.accounts.attribute_mul;

    let mut new_attribute_mul = AttributeMul {
        attribute_multiply: ix.attribute_multiply,
        multiply_amount: vec![],
        multiply_prob: vec![],
        bump: attribute_mul.bump,
        authority: ix.authority,
    };
    
    for data in ix.multiply_data.iter() {

        if data.len() != 2 {
            return Err(error!(ErrorCode::NotSameProbAmount));
        }
        new_attribute_mul.multiply_amount.push(data[0] as f32);
        new_attribute_mul.multiply_prob.push(data[1] as u32);
    }


    let new_space = new_attribute_mul.try_to_vec()?.len() + 8;
    attribute_mul.set_inner(new_attribute_mul);

    resize_account(
        &attribute_mul.to_account_info(),
        new_space,
        &ctx.accounts.payer.to_account_info(),
        &ctx.accounts.system_program.to_account_info(),
    )?;
    Ok(())
}