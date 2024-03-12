use anchor_lang::prelude::*;

#[derive(InitSpace, Copy, Clone, Debug, AnchorSerialize, AnchorDeserialize, PartialEq)]
pub enum Cell {
    Empty,
    Block,
    Recharge,
}
impl Default for Cell {
    fn default() -> Self {
        Cell::Empty
    }
}

#[derive(InitSpace, Copy, Clone, Debug, AnchorSerialize, AnchorDeserialize)]
pub struct Grid {
    pub cells: [[Cell; 10]; 10],
}

#[derive(InitSpace, Clone, Copy, Debug, AnchorSerialize, AnchorDeserialize)]
pub struct Player {
    pub x_coord: u8,
    pub y_coord: u8,
    pub address: Pubkey,
    pub energy: u8,
}

#[account]
#[derive(InitSpace)]
pub struct DummyAccount {
    pub cell: Grid,
    #[max_len(2)]
    pub player: Vec<Player>,
    pub owner: Pubkey,
    pub bump: u8,
}