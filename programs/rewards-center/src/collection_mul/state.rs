use anchor_lang::prelude::*;

pub const COLLECTION_MUL_DEFAULT_SIZE: usize = 32 + 32 + 24 + 8 + 32 + 32;
pub const COLLECTION_MUL_PREFIX: &str = "collection-mul";

#[account]
pub struct CollectionMul {
    pub collections_multiply: Vec<Pubkey>,
    pub multiply_amount: Vec<f32>,
    pub multiply_prob: Vec<u32>,
    pub bump: u8,
    pub authority: Pubkey,
}

