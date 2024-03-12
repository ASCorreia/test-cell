use crate::*;

#[derive(Accounts)]
pub struct AddPlayer<'info> {
    #[account(
        mut, 
        seeds = [b"Dummy"], 
        bump = account.bump
    )]
    account: Account<'info, DummyAccount>,
    user: Signer<'info>,
}

impl<'info> AddPlayer<'info> {
    pub fn add_player(&mut self) -> Result<()> {
        self.account.player.push(Player { 
            x_coord: 0, 
            y_coord: 0, 
            address: self.user.key(),
            energy: 100 }
        );
        msg!("Player succesfully added! The total number of players is {:?}", self.account.player.len());
        
        Ok(())
    }

    pub fn find_player(&mut self) -> Result<()> {
        let player = self.account.player.iter().find(|player| player.address == self.user.key());

        match player {
            Some(val) => msg!("Player found with address {:?}", val.address),
            None => msg!("Player with address {:?} was not found!", self.user.key()),
        }

        Ok(())
    }
}