use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

pub mod contexts;
pub mod state;

pub use contexts::*;
pub use state::*;

#[program]
pub mod test_cell {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.initialize()?;
        ctx.accounts.account.bump = *ctx.bumps.get("account").unwrap();

        msg!("User Grid Account successfully created!");
        
        Ok(())
    }

    pub fn move_player_x(ctx: Context<MovePlayer>) -> Result<()> {
        ctx.accounts.move_player_x()?;

        msg!("Player Move Operation Completed!");
        
        Ok(())
    }

    pub fn move_player_y(ctx: Context<MovePlayer>) -> Result<()> {
        ctx.accounts.move_player_y()?;

        msg!("Player Move Operation Completed!");

        Ok(())
    }

    pub fn add_player(ctx: Context<AddPlayer>) -> Result<()> {
        ctx.accounts.add_player()?;

        msg!("Add Player Operation Completed!");

        Ok(())
    }

    pub fn find_player(ctx: Context<AddPlayer>) -> Result<()> {
        ctx.accounts.find_player()?;

        msg!("Find Player Operation Completed!");

        Ok(())
    }

    pub fn check_surrondings(ctx: Context<MovePlayer>) -> Result<()> {
        ctx.accounts.check_surrondings()?;

        msg!("Check Surroundings Operation Completed!");

        Ok(())
    }
}
