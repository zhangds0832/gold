use anchor_lang::prelude::*;

declare_id!("8Vygu4A1v3dXyMhA6U7H1jp7zKRcptU4zXto8VHtNXjJ");

#[program]
pub mod gold {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
