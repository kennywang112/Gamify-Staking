use anchor_lang::prelude::*;

pub const ATTRIBUTE_MUL_DEFAULT_SIZE: usize = 32 + 32 + 24 + 8 + 32 + 32;
pub const ATTRIBUTE_MUL_PREFIX: &str = "attribute-mul";

#[account]
pub struct AttributeMul {
    pub attribute_multiply: Vec<Vec<String>>,
    pub multiply_amount: Vec<f32>,
    pub multiply_prob: Vec<u32>,
    pub bump: u8,
    pub authority: Pubkey,
}

