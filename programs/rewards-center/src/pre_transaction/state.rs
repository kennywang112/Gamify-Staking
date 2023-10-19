use anchor_lang::prelude::*;

pub const PROOF_DEFAULT_SIZE: usize = 32 + 24 + 24 + 32;
pub const PROOF_PREFIX: &str = "proof";
#[account]
pub struct Proof {
    pub authority: Pubkey,
    pub identifier: String,
    pub time: i64,
}
