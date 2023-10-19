use anchor_lang::prelude::*;

pub const USER_ATTRIBUTE_DEFAULT_SIZE: usize = 32 + 32 + 24 + 32;
pub const USER_ATTRIBUTE_PREFIX: &str = "user-attribute";

#[account]
pub struct UserAttribute {
    pub collection: Pubkey,
    pub attribute: Vec<Vec<String>>,
    pub identifier: String,
    pub authority: Pubkey,
}

