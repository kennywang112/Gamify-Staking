pub mod stake_pool;
pub use stake_pool::*;
pub mod stake_entry;
pub use stake_entry::*;
pub mod authorization;
pub use authorization::*;
pub mod reward_distribution;
pub use reward_distribution::*;
pub mod payment;
pub use payment::*;
pub mod discount;
pub use discount::*;
pub mod config;
pub use config::*;
pub mod collection_mul;
pub use collection_mul::*;
pub mod attribute_mul;
pub use attribute_mul::*;
pub mod user_attribute;
pub use user_attribute::*;
pub mod pre_transaction;
pub use pre_transaction::*;

pub mod errors;
pub mod utils;

use anchor_lang::prelude::*;

declare_id!("E2PSH444f3ScPpuNChXYLrv7KfcWWNdJvQKwfkaw3wM");


#[program]
pub mod trading_train_center {

    use super::*;

    //// discount ////
    pub fn init_discount(ctx: Context<InitDiscountCtx>, ix:InitDiscountIx) -> Result<()> {
        discount::init_discount::handler(ctx, ix)
    }
    pub fn update_discount(ctx: Context<UpdateDiscountCtx>, ix:UpdateDiscountIx) -> Result<()> {
        discount::update_discount::handler(ctx, ix)
    }

    //// config ////
    pub fn init_config_entry<'key, 'accounts, 'remaining, 'info>(ctx: Context<'key, 'accounts, 'remaining, 'info, InitConfigEntryCtx<'info>>, ix: InitConfigEntryIx) -> Result<()> {
        config::init_config_entry::handler(ctx, ix)
    }
    pub fn update_config_entry(ctx: Context<UpdateConfigEntryCtx>, ix: UpdateConfigEntryIx) -> Result<()> {
        config::update_config_entry::handler(ctx, ix)
    }

    //// init proof ////
    // pub fn init_proof(ctx: Context<InitProofCtx>, ix: InitProofIx) -> Result<()> {
    //     pre_transaction::init_proof::handler(ctx, ix)
    // }

    //// user attribute ////
    pub fn init_user_attribute(ctx: Context<UserAttributeCtx>, ix: UserAttributeIx) -> Result<()> {
        user_attribute::init_user_attribute::handler(ctx, ix)
    }
    pub fn update_user_attribute(ctx: Context<UpdateUserAttributeCtx>, ix: UpdateUserAttributeIx) -> Result<()> {
        user_attribute::update_user_attribute::handler(ctx, ix)
    }

    //// collection_mul ////
    pub fn init_collection_mul(ctx: Context<InitCollectionMulCtx>, ix: InitCollectionMulIx) -> Result<()> {
        collection_mul::init_collection_mul::handler(ctx, ix)
    }
    pub fn update_collection_mul(ctx: Context<UpdateCollectionMulCtx>, ix: UpdateCollectionMulIx) -> Result<()> {
        collection_mul::update_collection_mul::handler(ctx, ix)
    }

    //// attribute_mul ////
    pub fn init_attribute_mul(ctx: Context<InitAttributeMulCtx>, ix: InitAttributeMulIx) -> Result<()> {
        attribute_mul::init_attribute_mul::handler(ctx, ix)
    }
    pub fn update_attribute_mul(ctx: Context<UpdateAttributeMulCtx>, ix: UpdateAttributeMulIx) -> Result<()> {
        attribute_mul::update_attribute_mul::handler(ctx, ix)
    }

    //// stake_pool ////
    pub fn init_pool(ctx: Context<InitPoolCtx>, ix: InitPoolIx) -> Result<()> { // finish
        stake_pool::init_pool::handler(ctx, ix)
    }
    pub fn update_pool(ctx: Context<UpdatePoolCtx>, ix: UpdatePoolIx) -> Result<()> {
        stake_pool::update_pool::handler(ctx, ix)
    }
    pub fn close_stake_pool(ctx: Context<CloseStakePoolCtx>) -> Result<()> {
        stake_pool::close_stake_pool::handler(ctx)
    }

    //// stake_entry ////
    pub fn init_entry(ctx: Context<InitEntryCtx>, user: Pubkey) -> Result<()> {//
        stake_entry::init_entry::handler(ctx, user)
    }
    pub fn update_total_stake_seconds(ctx: Context<UpdateTotalStakeSecondsCtx>) -> Result<()> {
        stake_entry::update_total_stake_seconds::handler(ctx)
    }
    pub fn reset_stake_entry(ctx: Context<ResetStakeEntryCtx>) -> Result<()> {
        stake_entry::reset_stake_entry::handler(ctx)
    }
    pub fn resize_stake_entry(ctx: Context<ResizeStakeEntryCtx>) -> Result<()> {
        stake_entry::resize_stake_entry::handler(ctx)
    }
    pub fn close_stake_entry(ctx: Context<CloseStakeEntryCtx>) -> Result<()> {
        stake_entry::close_stake_entry::handler(ctx)
    }

    //// stake_entry::editions ////
    pub fn stake_edition<'key, 'accounts, 'remaining, 'info>(ctx: Context<'key, 'accounts, 'remaining, 'info, StakeEditionCtx<'info>>, amount: u64) -> Result<()> {
        stake_entry::editions::stake_edition::handler(ctx, amount)
    }
    pub fn unstake_edition<'key, 'accounts, 'remaining, 'info>(ctx: Context<'key, 'accounts, 'remaining, 'info, UnstakeEditionCtx<'info>>) -> Result<()> {
        stake_entry::editions::unstake_edition::handler(ctx)
    }

    pub fn stake_pnft(ctx: Context<StakePNFTCtx>) -> Result<()> {
        stake_entry::pnfts::stake_pnft::handler(ctx)
    }
    pub fn unstake_pnft(ctx: Context<UnstakePNFTCtx>) -> Result<()> {
        stake_entry::pnfts::unstake_pnft::handler(ctx)
    }

    //// authorization ////
    pub fn authorize_mint(ctx: Context<AuthorizeMintCtx>, mint: Pubkey) -> Result<()> { //finish
        authorization::authorize_mint::handler(ctx, mint)
    }
    pub fn deauthorize_mint(ctx: Context<DeauthorizeMintCtx>) -> Result<()> {
        authorization::deauthorize_mint::handler(ctx)
    }

    //// reward_distribution ////
    //// reward_distribution::reward_distributor ////
    pub fn init_reward_distributor(ctx: Context<InitRewardDistributorCtx>, ix: InitRewardDistributorIx) -> Result<()> {
        reward_distribution::reward_distributor::init_reward_distributor::handler(ctx, ix)
    }
    pub fn update_reward_distributor(ctx: Context<UpdateRewardDistributorCtx>, ix: UpdateRewardDistributorIx) -> Result<()> {
        reward_distribution::reward_distributor::update_reward_distributor::handler(ctx, ix)
    }
    pub fn close_reward_distributor(ctx: Context<CloseRewardDistributorCtx>) -> Result<()> {
        reward_distribution::reward_distributor::close_reward_distributor::handler(ctx)
    }
    pub fn reclaim_funds(ctx: Context<ReclaimFundsCtx>, amount: u64) -> Result<()> {
        reward_distribution::reward_distributor::reclaim_funds::handler(ctx, amount)
    }

    //// reward_distribution::reward_entry ////
    pub fn init_reward_entry(ctx: Context<InitRewardEntryCtx>) -> Result<()> {
        reward_distribution::reward_entry::init_reward_entry::handler(ctx)
    }
    pub fn close_reward_entry(ctx: Context<CloseRewardEntryCtx>) -> Result<()> {
        reward_distribution::reward_entry::close_reward_entry::handler(ctx)
    }
    pub fn update_reward_entry(ctx: Context<UpdateRewardEntryCtx>, ix: UpdateRewardEntryIx) -> Result<()> {
        reward_distribution::reward_entry::update_reward_entry::handler(ctx, ix)
    }
    pub fn claim_rewards(ctx: Context<ClaimRewardsCtx>) -> Result<()> {
        reward_distribution::reward_entry::claim_rewards::handler(ctx)
    }

    //// payment ////
    pub fn init_payment_info(ctx: Context<InitPaymentInfoCtx>, ix: InitPaymentInfoIx) -> Result<()> {//finish
        payment::init_payment_info::handler(ctx, ix)
    }
    pub fn update_payment_info(ctx: Context<UpdatePaymentInfoCtx>, ix: UpdatePaymentInfoIx) -> Result<()> {
        payment::update_payment_info::handler(ctx, ix)
    }
    pub fn close_payment_info(ctx: Context<ClosePaymentInfoCtx>) -> Result<()> {
        payment::close_payment_info::handler(ctx)
    }
}
