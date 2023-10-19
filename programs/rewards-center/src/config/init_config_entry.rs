use anchor_lang::prelude::*;
use crate::ConfigEntry;
use crate::assert_authority;
use crate::resize_account;
use crate::CONFIG_ENTRY_SIZE;
use crate::CONFIG_ENTRY_SEED_PREFIX;

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct InitConfigEntryIx {
    prefix: Vec<u8>,
    key: Vec<u8>,
    value: String,
    extends: Vec<Pubkey>,
}

#[derive(Accounts)]
#[instruction(ix: InitConfigEntryIx)]
pub struct InitConfigEntryCtx<'info> {
    #[account(
        init,
        payer = authority,
        space = CONFIG_ENTRY_SIZE,
        seeds = [CONFIG_ENTRY_SEED_PREFIX.as_bytes(), ix.prefix.as_ref(), ix.key.as_ref()],
        bump
    )]
    config_entry: Account<'info, ConfigEntry>,

    #[account(mut)]
    authority: Signer<'info>,
    system_program: Program<'info, System>,
}

pub fn handler<'key, 'accounts, 'remaining, 'info>(ctx: Context<'key, 'accounts, 'remaining, 'info, InitConfigEntryCtx<'info>>, ix: InitConfigEntryIx) -> Result<()> {
    assert_authority(
        &ix.prefix,
        &String::from("").as_bytes(),
        &ix.value.as_bytes(),
        ctx.accounts.authority.key(),
        &mut ctx.remaining_accounts.iter(),
    )?;
    let config_entry = &mut ctx.accounts.config_entry;

    let new_config_entry = ConfigEntry {
        bump: *ctx.bumps.get("config_entry").unwrap(),
        prefix: ix.prefix,
        key: ix.key,
        value: ix.value,
        extends: ix.extends,
    };
    let new_space = new_config_entry.try_to_vec()?.len() + 8;
    config_entry.set_inner(new_config_entry);

    resize_account(
        &config_entry.to_account_info(),
        new_space,
        &ctx.accounts.authority.to_account_info(),
        &ctx.accounts.system_program.to_account_info(),
    )?;

    Ok(())
}
