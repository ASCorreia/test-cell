use crate::*;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, seeds = [b"Dummy"], bump, payer = user, space = 8 + DummyAccount::INIT_SPACE)]
    pub account: Account<'info, DummyAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> Initialize<'info> {
    pub fn initialize(&mut self) -> Result<()> {
        self.account.cell = Grid {
            cells: [[Cell::Empty; 10]; 10],
        };

        self.account.cell.cells[0][1] = Cell::Block;
        self.account.cell.cells[4][2] = Cell::Block;
        self.account.cell.cells[5][5] = Cell::Block;
        self.account.cell.cells[2][2] = Cell::Block;
        self.account.cell.cells[0][5] = Cell::Block;
        self.account.cell.cells[5][1] = Cell::Block;

        let player_vec: Vec<Player> = vec![];

        self.account.player = player_vec;

        self.account.owner = self.user.key();

        msg!("User Cell Account created");
        
        Ok(())
    }
}