pub mod instructions;
pub mod errors;

use anchor_lang::prelude::*;
use instructions::*;

declare_id!("uAreWLnxjRHP3pcgy1kCtJNs9hMW7HiCzVt9r31nqVc");

#[program]
pub mod undead_arena {
    use super::*;

    pub fn start_match(ctx: Context<StartMatch>, bet: u64) -> Result<()> {
        instructions::start_match(ctx, bet)
    }
}
