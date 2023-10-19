use anchor_lang::prelude::*;
use crate::ConfigEntry;
use crate::assert_authority;
use crate::resize_account;

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct UpdateConfigEntryIx {
    value: String,
    extends: Vec<Pubkey>,
    append: bool,
}

#[derive(Accounts)]
#[instruction(ix: UpdateConfigEntryIx)]
pub struct UpdateConfigEntryCtx<'info> {
    #[account(mut)]
    config_entry: Account<'info, ConfigEntry>,
    #[account(mut)]
    authority: Signer<'info>,
    system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<UpdateConfigEntryCtx>, ix: UpdateConfigEntryIx) -> Result<()> {
    assert_authority(
        &ctx.accounts.config_entry.prefix,
        &ctx.accounts.config_entry.value.as_bytes(),
        &ix.value.as_bytes(),
        ctx.accounts.authority.key(),
        &mut ctx.remaining_accounts.iter(),
    )?;
    let config_entry = &mut ctx.accounts.config_entry;

    let new_config_entry = ConfigEntry {
        bump: config_entry.bump,
        prefix: config_entry.prefix.to_vec(),
        key: config_entry.key.to_vec(),
        value: if ix.append { config_entry.value.to_string() + &ix.value } else { ix.value },
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
