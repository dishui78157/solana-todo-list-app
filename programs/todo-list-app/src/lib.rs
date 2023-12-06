use anchor_lang::prelude::*;

declare_id!("9YgWakAxc5Keakf9fjRc83ZqEJzFVpUVaE6VtcnL95ML");

#[program]
pub mod todo_list_app {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
