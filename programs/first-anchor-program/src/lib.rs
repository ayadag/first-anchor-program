use anchor_lang::prelude::*;

declare_id!("DeW1FJiWH26Fvf56tR4r9XeAonfpfiYgK68myCP7DoJo");

#[program]
pub mod first_anchor_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
