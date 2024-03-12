use crate::*;

#[derive(Accounts)]
pub struct MovePlayer<'info> {
    #[account(
        mut, 
        seeds = [b"Dummy"], 
        bump = account.bump
    )]
    account: Account<'info, DummyAccount>,
    user: Signer<'info>,
}

impl<'info> MovePlayer<'info> {
    pub fn move_player_y(&mut self) -> Result<()> {
        let cells = self.account.cell.cells.clone();
        let player = self.account.player.iter_mut().find(|player| player.address == self.user.key());
        match player {
            Some(player) => {
                let new_x = player.x_coord;
                let new_y = player.y_coord + 1;

                msg!("Player trying to moved from x: {:?}, y: {:?}", player.x_coord, player.y_coord);

                match cells[new_x as usize][new_y as usize] {
                    Cell::Empty => {
                        player.y_coord = new_y;
                        msg!("Player successfully moved to x: {:?}, y: {:?}", player.x_coord, player.y_coord);
                        return Ok(())
                    }
                    Cell::Recharge => {
                        player.y_coord = new_y;
                    msg!("Player successfully moved to x: {:?}, y: {:?}", player.x_coord, player.y_coord);
                    return Ok(())
                    }
                    _ => {
                        msg!("Player could not move!! Road is blocked in x: {:?}, y: {:?}", new_x, new_y);
                        return Ok(())
                    }
                }
            }
            None => {
                msg!("Player not found");
                return Ok(())
            }
        }
    }

    pub fn move_player_x(&mut self) -> Result<()> {
        let cells = self.account.cell.cells.clone();
        let player = self.account.player.iter_mut().find(|player| player.address == self.user.key());
        match player {
            Some(player) => {
                let new_x = player.x_coord + 1;
                let new_y = player.y_coord;

                msg!("Player trying to moved from x: {:?}, y: {:?}", player.x_coord, player.y_coord);

                match cells[new_x as usize][new_y as usize] {
                    Cell::Empty => {
                        player.x_coord = new_x;
                        msg!("Player successfully moved to x: {:?}, y: {:?}", player.x_coord, player.y_coord);
                        return Ok(())
                    }
                    Cell::Recharge => {
                        player.x_coord = new_x;
                    msg!("Player successfully moved to x: {:?}, y: {:?}", player.x_coord, player.y_coord);
                    return Ok(())
                    }
                    _ => {
                        msg!("Player could not move!! Road is blocked in x: {:?}, y: {:?}", new_x, new_y);
                        return Ok(())
                    }
                }
            }
            None => {
                msg!("Player not found");
                return Ok(())
            }
        }
    }

    pub fn check_surrondings(&mut self) -> Result<()> {
        let player = self.account.player.iter().find(|player| player.address == self.user.key());
        match player {
            Some(player) => {
                msg!("Player {:?} found", player.address);

                let (check_x, check_y) = (player.x_coord, player.y_coord);
                let cells_to_check = [(self.check_add(check_x + 1), check_y), (self.check_add(check_x + 1), self.check_add(check_y + 1)), 
                (check_x.saturating_sub(1), check_y), (check_x.saturating_sub(1), check_y.saturating_sub(1)),
                (check_x , self.check_add(check_y + 1)), (check_x , check_y.saturating_sub(1)), 
                (self.check_add(check_x + 1), check_y.saturating_sub(1)), (check_x.saturating_sub(1), self.check_add(check_y + 1))];

                let mut counter: u8 = 0;

                let mut last_player_attacked = Pubkey::default();

                for index in 0..cells_to_check.len() {
                    let victim = self.account.player.iter_mut().find(|player| (player.x_coord, player.y_coord) == cells_to_check[index]);
                    match victim {
                        Some(player) => {
                            if player.address != self.user.key() && player.address != last_player_attacked {
                                counter += 1;
                                msg!("Nearby player found in x: {:?}, y: {:?} with energy {:?}!", player.x_coord, player.y_coord, player.energy);
                                //Implement last_player_attacked by comparing the user key with the last attacked key
                                player.energy -= 5;
                                msg!("Player {:?} attacked! Remaining energy is {:?}", player.address, player.energy);
                                last_player_attacked = player.address;
                            }
                        }
                        None => continue,
                    }
                }
                msg!("Total number of nearby players {:?}", counter);
                
                return Ok(());
            }
            None => {
                msg!("Player {:?} not found", self.user.key());
                return Ok(());
            }
        }
    }

    pub fn check_add(&mut self, coord: u8) -> u8 {
        if coord > self.account.cell.cells[0].len() as u8 {
            return self.account.cell.cells[0].len() as u8
        }
        coord
    }
}