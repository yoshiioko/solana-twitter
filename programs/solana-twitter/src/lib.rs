use anchor_lang::prelude::*;

declare_id!("6jRCggDP41Qe5Rc5ECCgbQP5tvDEqxf2hSyWFdW71tzc");

#[program]
pub mod solana_twitter {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
