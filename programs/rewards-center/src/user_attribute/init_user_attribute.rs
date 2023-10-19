use crate::UserAttribute;
use crate::utils::resize_account;
use crate::USER_ATTRIBUTE_DEFAULT_SIZE;
use crate::USER_ATTRIBUTE_PREFIX;
use anchor_lang::prelude::*;
use anchor_spl::token::Mint;

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct UserAttributeIx {
    collection: Pubkey,
    attribute: Vec<Vec<String>>,
    identifier: String,
    authority: Pubkey,
}

#[derive(Accounts)]
#[instruction(ix: UserAttributeIx)]
pub struct UserAttributeCtx<'info> {

    #[account(
        init,
        payer = payer,
        space = USER_ATTRIBUTE_DEFAULT_SIZE,
        seeds = [
            USER_ATTRIBUTE_PREFIX.as_bytes(),
            ix.identifier.as_ref(),
            user_mint.key().as_ref()
            ],
        bump
    )]
    pub user_attribute: Account<'info, UserAttribute>,

    pub user_mint: Account<'info, Mint>,

    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<UserAttributeCtx>,
    ix: UserAttributeIx
) -> Result<()> {

    // let attribute_strings: Vec<Vec<String>> = ix.attribute.iter().map(|v| v.iter().map(|s| s.to_string()).collect()).collect();

    let new_user_attribute = UserAttribute {
        collection: ix.collection,
        attribute: ix.attribute,
        identifier: ix.identifier,
        authority: ix.authority
    };

    let user_attribute = &mut ctx.accounts.user_attribute;
    let new_space = new_user_attribute.try_to_vec()?.len() + 8;

    resize_account(
        &user_attribute.to_account_info(), 
        new_space, 
        &ctx.accounts.payer.to_account_info(), 
        &ctx.accounts.system_program.to_account_info(),
    )?;

    user_attribute.set_inner(new_user_attribute);

    Ok(())
}