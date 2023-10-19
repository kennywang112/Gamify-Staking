use anchor_lang::prelude::*;
use crate::errors::ErrorCode;
use std::collections::{HashMap, HashSet};
use crate::reward_distribution::{RewardDistributor, RewardEntry, REWARD_DISTRIBUTOR_SEED};
use crate::{StakeEntry, StakePool, CollectionMul, AttributeMul, UserAttribute};
use anchor_spl::token::{self, Mint, Token, TokenAccount};
use mpl_token_metadata::state::Metadata;
use std::cmp::min;
use crate::handle_payment_info;

#[derive(Accounts)]
pub struct ClaimRewardsCtx<'info> {
    #[account(mut)]
    reward_entry: Box<Account<'info, RewardEntry>>,
    #[account(mut, constraint = reward_distributor.stake_pool == stake_pool.key())]
    reward_distributor: Box<Account<'info, RewardDistributor>>,

    #[account(constraint = stake_entry.key() == reward_entry.stake_entry @ ErrorCode::InvalidStakeEntry)]
    stake_entry: Box<Account<'info, StakeEntry>>,
    #[account(constraint = stake_pool.key() == stake_entry.pool)]
    stake_pool: Box<Account<'info, StakePool>>,

    #[account(mut, constraint = reward_mint.key() == reward_distributor.reward_mint @ ErrorCode::InvalidRewardMint)]
    reward_mint: Box<Account<'info, Mint>>,
    #[account(mut, constraint = user_reward_mint_token_account.owner == stake_entry.last_staker &&  user_reward_mint_token_account.mint == reward_distributor.reward_mint @ ErrorCode::InvalidUserRewardMintTokenAccount)]
    user_reward_mint_token_account: Box<Account<'info, TokenAccount>>,

    #[account(mut, constraint = reward_distributor_token_account.mint == reward_mint.key() && reward_distributor_token_account.owner == reward_distributor.key() @ ErrorCode::InvalidTokenAccount)]
    reward_distributor_token_account: Box<Account<'info, TokenAccount>>,

    #[account(mut, constraint = user.key() == stake_entry.last_staker || user.key() == reward_distributor.authority @ ErrorCode::InvalidAuthority)]
    user: Signer<'info>,
    token_program: Program<'info, Token>,
    system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<ClaimRewardsCtx>
) -> Result<()> {

    let reward_entry = &mut ctx.accounts.reward_entry;
    let reward_distributor = &mut ctx.accounts.reward_distributor;
    let stake_pool = reward_distributor.stake_pool;
    let stake_entry = &mut ctx.accounts.stake_entry;
    let identifier_seed = reward_distributor.identifier.to_le_bytes();
    let reward_distributor_seed = &[REWARD_DISTRIBUTOR_SEED.as_bytes(), stake_pool.as_ref(), identifier_seed.as_ref(), &[reward_distributor.bump]];
    let reward_distributor_signer = &[&reward_distributor_seed[..]];

    let reward_amount = reward_distributor.reward_amount;
    let reward_duration_seconds = reward_distributor.reward_duration_seconds;

    let reward_seconds_received = reward_entry.reward_seconds_received;
    if reward_seconds_received <= stake_entry.total_stake_seconds {

        let mut reward_seconds = stake_entry.total_stake_seconds;
        if let Some(max_reward_seconds) = reward_distributor.max_reward_seconds_received {
            reward_seconds = min(reward_seconds, max_reward_seconds)
        };
        if reward_seconds_received >= reward_seconds {
            msg!("Max reward seconds claimed");
            return Ok(());
        }

        //add for collection
        // remove the origin param stake_mint_metadata to fit the origin program, using remaining account
        let remaining_accounts: &mut std::slice::Iter<'_, AccountInfo<'_>> = &mut ctx.remaining_accounts.iter();
        let mint_metadata_data = next_account_info(remaining_accounts)?;
        let stake_mint_metadata = Metadata::deserialize(&mut mint_metadata_data.try_borrow_mut_data().unwrap().as_ref()).expect("Damn, failed to deserialize metadata");
        
        let collection_key = stake_mint_metadata.collection.unwrap().key;
        
        if stake_mint_metadata.mint.key() != stake_entry.stake_mint.key() {
            return  Err(error!(ErrorCode::NotSameMint));
        }
        
        //add for probability using clock
        let clock: Clock = Clock::get()?;
        let random_collection = vec![13, 25, 75, 43, 78, 92, 49, 47, 57, 13, 17, 29, 71, 79, 89, 54, 76, 44, 14, 88];
        let random_attribute = vec![88, 45, 67, 33, 23, 22, 18, 78, 89, 23, 46, 37, 55, 91, 43, 29, 35, 12, 19, 43];

        let collection_multiply: &AccountInfo<'_> = next_account_info(remaining_accounts)?;
        let collection_multiply_data = Account::<CollectionMul>::try_from(collection_multiply)
        .map(|record| Some(record))
        .unwrap_or(None);

        let attribute_multiply_pub: &AccountInfo<'_> = next_account_info(remaining_accounts)?;
        let attribute_multiply_data = Account::<AttributeMul>::try_from(attribute_multiply_pub)
        .map(|record| Some(record))
        .unwrap_or(None);

        let user_attribute_multiply_pub: &AccountInfo<'_> = next_account_info(remaining_accounts)?;
        let user_attribute_multiply_data = Account::<UserAttribute>::try_from(user_attribute_multiply_pub)
        .map(|record| Some(record))
        .unwrap_or(None);

        let mut collection_transfer_mul: f32 = 1.0 * 1000.0;
        let mut collection_random_storage: HashMap<_, _> = HashMap::new();

        if let Some(collections_multiply) = collection_multiply_data {

            if collections_multiply.collections_multiply.len() > 0 {

                for step in 0..collections_multiply.collections_multiply.len() {
    
                    // set random value for each collection
                    let collection_setting: &mut (i64, u32) = collection_random_storage
                        .entry(collections_multiply.collections_multiply[step])
                        .or_insert((clock.unix_timestamp.checked_mul(random_collection[step]).unwrap().checked_rem(100).unwrap() + 1, 0));
    
                    // get the random value for each collection
                    let random: i64 = collection_setting.0;
    
                    collection_setting.1 += collections_multiply.multiply_prob[step]; // this is a cumulate for probability
    
                    if collections_multiply.collections_multiply[step] == collection_key && random <= collection_setting.1 as i64 {
                        collection_transfer_mul *= collections_multiply.multiply_amount[step];
                        break;
                    }
    
                }
            }
        }

        // 保留三位小數
        let mut attribute_transfer_mul: f32 = 1.0 * 1000.0;
        let mut attribute_random_storage: HashMap<_, _> = HashMap::new();
        // let attribute_multiply = attribute_multiply_data.unwrap();//.attribute_multiply;

        // if the owners inited attribute have values in it
        if let Some(attribute_multiply) = attribute_multiply_data {

            if attribute_multiply.attribute_multiply.len() > 0 {

                // set a hashset to insert the users attribute, in order to match the attribute from below
                let mut attribute_set: HashSet<Vec<String>> = HashSet::new();

                let user_attribute = user_attribute_multiply_data.unwrap();
                for index in 0..user_attribute.attribute.len() {
                    // insert all users attribute to attribute_set
                    attribute_set.insert(user_attribute.attribute[index].clone());
                }

                // for each attribute multiply
                for step in 0..attribute_multiply.attribute_multiply.len() {

                    // attribute collection address
                    let attribute_collection_address: String = attribute_multiply.attribute_multiply[step][0].clone();

                    // stake collection address
                    let stake_collection_address: String = user_attribute.collection.to_string();

                    // [trait_type, trait_value]
                    let current_attribute: Vec<String> = attribute_multiply.attribute_multiply[step][1..].to_vec();

                    // 同時符合 collection address 相同 及 nft 擁有此 attribute
                    if attribute_collection_address == stake_collection_address && attribute_set.contains(&current_attribute) {

                        let attribute_setting: &mut (i64, u32) = attribute_random_storage
                            .entry(attribute_multiply.attribute_multiply[step].clone())
                            .or_insert((clock.unix_timestamp.checked_mul(random_attribute[step]).unwrap().checked_rem(100).unwrap() + 1, 0));

                        let random: i64 = attribute_setting.0;

                        attribute_setting.1 += attribute_multiply.multiply_prob[step];

                        if attribute_set.contains(&current_attribute) && random <= attribute_setting.1 as i64 {

                            attribute_transfer_mul *= attribute_multiply.multiply_amount[step];

                            // 若觸發條件，則將此 attribute 之 probability 減少 200，避免再次觸發
                            attribute_setting.1 -= 200;

                        }
                    }
                }
            }
        }
        
        let mut reward_amount_to_receive = reward_seconds
            .checked_sub(reward_seconds_received)
            .unwrap()
            .checked_div(reward_duration_seconds)
            .unwrap()
            .checked_mul(reward_amount as u128)
            .unwrap()
            .checked_mul(reward_entry.multiplier as u128)
            .unwrap()
            .checked_div((10_u128).checked_pow(reward_distributor.multiplier_decimals as u32).unwrap())
            .unwrap();

        let reward_amount_to_receive_with_mul = reward_amount_to_receive
            .checked_mul(collection_transfer_mul as u128)
            .unwrap()
            .checked_div(1000)
            .unwrap()
            .checked_mul(attribute_transfer_mul as u128)
            .unwrap()
            .checked_div(1000)
            .unwrap();

        // mint to the user
        // if reward_amount_to_receive_with_mul > ctx.accounts.reward_distributor_token_account.amount as u128 {
        //     reward_amount_to_receive = ctx.accounts.reward_distributor_token_account.amount as u128;
        // }
        if reward_amount_to_receive > ctx.accounts.reward_distributor_token_account.amount as u128 {
            reward_amount_to_receive = ctx.accounts.reward_distributor_token_account.amount as u128;
            // return Err(error!(ErrorCode::RewardNotEnough));
        }

        let cpi_accounts = token::Transfer {
            from: ctx.accounts.reward_distributor_token_account.to_account_info(),
            to: ctx.accounts.user_reward_mint_token_account.to_account_info(),
            authority: reward_distributor.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_context = CpiContext::new(cpi_program, cpi_accounts).with_signer(reward_distributor_signer);

        token::transfer(cpi_context, reward_amount_to_receive_with_mul
            .try_into().expect("Too many rewards to receive"))?;

        let reward_time_to_receive = if reward_entry.multiplier != 0 {
            reward_amount_to_receive
                .checked_mul((10_u128).checked_pow(reward_distributor.multiplier_decimals as u32).unwrap())
                .unwrap()
                .checked_div(reward_entry.multiplier as u128)
                .unwrap()
                .checked_div(reward_amount as u128)
                .unwrap()
                .checked_mul(reward_duration_seconds)
                .unwrap()
        } else {
            0_u128
        };

        reward_distributor.rewards_issued = reward_distributor.rewards_issued.checked_add(reward_amount_to_receive).unwrap();
        reward_entry.reward_seconds_received = reward_entry.reward_seconds_received.checked_add(reward_time_to_receive).unwrap();

        // handle payment
        // let remaining_accounts = &mut ctx.remaining_accounts.iter();
        handle_payment_info(remaining_accounts)?;

    }

    Ok(())
}