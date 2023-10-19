use anchor_lang::prelude::*;

pub const DISCOUNT_DEFAULT_SIZE: usize = 16 + 32 + 24 + 24;
pub const DISCOUNT_PREFIX: &str = "discount-prefix";
#[account]
pub struct Discount {
    // pub bump: u8,
    pub discount_str: String,
    pub authority: Pubkey,
    pub identifier: String,
    pub promotion_collection_mul: u64
}
