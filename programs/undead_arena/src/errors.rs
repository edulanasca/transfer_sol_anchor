use anchor_lang::prelude::*;

#[error_code]
pub enum UndeadArenaError {
    #[msg("The address specified is incorrect")]
    IncorrectVault,
}