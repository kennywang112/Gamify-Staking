use crate::utils::resize_account;
use crate::Discount;
use crate::DISCOUNT_DEFAULT_SIZE;
use crate::DISCOUNT_PREFIX;
use anchor_lang::prelude::*;
use anchor_spl::metadata::MetadataAccount;
use std::str::FromStr;
use anchor_spl::token::{Mint, TokenAccount};
// use mpl_token_metadata::state::Metadata;
use crate::errors::ErrorCode;

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct InitDiscountIx {
    discount_str: String,
    authority: Pubkey,
    identifier: String,
}

#[derive(Accounts)]
#[instruction(ix: InitDiscountIx)]
pub struct InitDiscountCtx<'info> {

    #[account(
        init,
        payer = payer,
        space = DISCOUNT_DEFAULT_SIZE,
        seeds = [DISCOUNT_PREFIX.as_bytes(), ix.identifier.as_ref()],
        bump
    )]
    discount_data: Account<'info, Discount>,

    /// The Mint of the NFT
    #[account(
        constraint = nft_mint.supply == 1 @ ErrorCode::MINTSUPPLYNOTONE,
    )]
    // determine if the nft belongs to the user
    pub nft_mint: Option<Box<Account<'info, Mint>>>,
    #[account(
        mut,
        // constraint = nft_token_account.mint == nft_mint.key().clone(),
        constraint = nft_token_account.amount == 1,
        constraint = nft_token_account.owner == payer.key()
    )]
    /// strict collection nft owner
    pub nft_token_account: Option<Box<Account<'info, TokenAccount>>>,
    #[account(
        mut,
        seeds = [
            b"metadata", 
            mpl_token_metadata::ID.as_ref(), 
            // nft_mint.unwrap().as_ref().key().as_ref()
            nft_mint.as_ref().unwrap().key().clone().as_ref()
        ],
        seeds::program = mpl_token_metadata::ID,
        bump,
        constraint = metadata_account.collection.as_ref().unwrap().verified
    )]
    /// strict the nft belongs to the collection
    pub metadata_account: Option<Account<'info, MetadataAccount>>,

    #[account(mut)]
    payer: Signer<'info>,

    system_program: Program<'info, System>,
    
}

pub fn handler(
    ctx: Context<InitDiscountCtx>, 
    ix: InitDiscountIx,
) -> Result<()> {

    let mut promotion_collection_mul :u64 = 100;

    // for the Option param
    if let Some(nft_token_account) = ctx.accounts.nft_token_account.clone() {
        
        if let Some(nft_mint) = ctx.accounts.nft_mint.clone() {
            
            if nft_mint.key() != nft_token_account.mint.key() {
                return Err(error!(ErrorCode::NOTTHESAMEMINT));
            }
        }

        // strict the collection and change the multiply
        if ctx.accounts.metadata_account.as_mut().unwrap().collection.as_ref().unwrap().key == Pubkey::from_str("8E8BHMvZiKq7q9xn1dw8rbZr7Vf2uPUdshaNU5mmFeZ8").unwrap() {
            
            promotion_collection_mul = 70

        }
        if ctx.accounts.metadata_account.as_mut().unwrap().collection.as_ref().unwrap().key == Pubkey::from_str("DGtq4HmAb9HtN1Y3n4jHBLUEHJtwxqSGWTuFkvas3Bw6").unwrap() {
            
            promotion_collection_mul = 60

        }

    }
    
    if ix.discount_str == String::from("Admiral2023") || ix.discount_str == String::from("Rentiirebel2023") || ix.discount_str == String::from("TradingTrain2023") {
        
        promotion_collection_mul = 80
    }


    let identifier = ix.identifier;
    let new_discount_data = Discount {
        // bump,
        discount_str: ix.discount_str,
        authority: ix.authority,
        identifier,
        promotion_collection_mul: promotion_collection_mul
    };

    let discount_data = &mut ctx.accounts.discount_data;
    let new_space = new_discount_data.try_to_vec()?.len() + 8;

    resize_account(
        &discount_data.to_account_info(),
        new_space,
        &ctx.accounts.payer.to_account_info(),
        &ctx.accounts.system_program.to_account_info(),
    )?;

    discount_data.set_inner(new_discount_data);
    Ok(())
}
