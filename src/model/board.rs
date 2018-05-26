//
// board.rs
// Copyright (C) 2018 Nitepone
// Distributed under terms of the MIT license.
//

pub struct Board{
    width: u32,
    height: u32,
    bomb_count: u32,
    board_array: Square[][],
}

impl Board{
    //Get the status of a board position
    pub fn getStatus(&self, u32: x, u32: y) -> u32 {
        self.board_array[x][y].Status
    }

    //Get the value of a board position
    pub fn getValue(&self, u32: x, u32: y) -> u32 {
        self.board_array[x][y]::getValue()
    }
}


#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn it_works() {
	}
}
