use crate::utils::resize_account;
use crate::Discount;
use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct UpdateDiscountIx {
    discount_str: String,
    authority: Pubkey,
    identifier: String,
}

#[derive(Accounts)]
#[instruction(ix: UpdateDiscountIx)]
pub struct UpdateDiscountCtx<'info> {
    #[account(mut, constraint = discount_data.authority == payer.key())]
    discount_data: Account<'info, Discount>,

    #[account(mut)]
    payer: Signer<'info>,
    system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<UpdateDiscountCtx>, 
    ix: UpdateDiscountIx
) -> Result<()> {

    let identifier = ix.identifier;
    let new_discount_data = Discount {
        // bump,
        discount_str: ix.discount_str,
        authority: ix.authority,
        identifier,
        promotion_collection_mul: ctx.accounts.discount_data.promotion_collection_mul
    };
    let discount_data = &mut ctx.accounts.discount_data;
    let new_space = new_discount_data.try_to_vec()?.len() + 8;
    discount_data.set_inner(new_discount_data);

    resize_account(
        &discount_data.to_account_info(),
        new_space,
        &ctx.accounts.payer.to_account_info(),
        &ctx.accounts.system_program.to_account_info(),
    )?;

    Ok(())
}
