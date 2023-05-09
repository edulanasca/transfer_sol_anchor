use std::str::FromStr;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::native_token::LAMPORTS_PER_SOL;
use crate::errors::UndeadArenaError;

#[account]
pub struct Vault {}

impl Id for Vault {
    fn id() -> Pubkey {
        Pubkey::from_str("Az9bZzW2197hmk1fjMWBQAjNjJWYqWPXL85dJjvtS4RE").unwrap()
    }
}

pub fn start_match(ctx: Context<StartMatch>, bet: u64) -> Result<()> {

    let ix = anchor_lang::solana_program::system_instruction::transfer(
        &ctx.accounts.player.key(),
        &ctx.accounts.vault.key(),
        bet
    );

    anchor_lang::solana_program::program::invoke(&ix,
    &[ctx.accounts.player.to_account_info(), ctx.accounts.vault.to_account_info()]
    )?;

    Ok(())
}

#[derive(Accounts)]
pub struct StartMatch<'info> {
    /// CHECK: Vault address
    #[account(mut, address = Vault::id() @ UndeadArenaError::IncorrectVault)]
    vault: AccountInfo<'info>,
    #[account(mut)]
    player: Signer<'info>,
    system_program: Program<'info, System>
}