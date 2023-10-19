use crate::UserAttribute;
use anchor_lang::prelude::*;
use anchor_spl::token::Mint;
use crate::utils::resize_account;

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct UpdateUserAttributeIx {
    collection: Pubkey,
    attribute: Vec<Vec<String>>,
    identifier: String,
    authority: Pubkey,
}

#[derive(Accounts)]
#[instruction(ix: UpdateUserAttributeIx)]
pub struct UpdateUserAttributeCtx<'info> {

    #[account(mut, constraint = user_attribute.authority == ix.authority.key())]
    pub user_attribute: Account<'info, UserAttribute>,

    pub user_mint: Account<'info, Mint>,

    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<UpdateUserAttributeCtx>,
    ix: UpdateUserAttributeIx
) -> Result<()> {

    let new_user_attribute = UserAttribute {
        collection: ix.collection,
        attribute: ix.attribute,
        identifier: ix.identifier,
        authority: ix.authority
    };

    let user_attribute = &mut ctx.accounts.user_attribute;
    let new_space = new_user_attribute.try_to_vec()?.len() + 8;
    user_attribute.set_inner(new_user_attribute);

    resize_account(
        &user_attribute.to_account_info(), 
        new_space, 
        &ctx.accounts.payer.to_account_info(), 
        &ctx.accounts.system_program.to_account_info(),
    )?;

    Ok(())
}