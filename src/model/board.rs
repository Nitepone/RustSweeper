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
    fn new(w: u32, h: u32, b: u32) -> Self {
        let raw_max = w*h;
        let work_array = Square[w][h];
        //Construct board_array
        for raw_pos in (0..raw_max) {
            //Determine if this is a bomb
            //Generate new square
            //add lesser adjacents to self
            //add self to adjacents
        }
        //Construct board.
        Board {
            width: w,
            height: h,
            bomb_count: b,
            board_array: 
        }
    }


    //Get the status of a board position
    pub fn getStatus(&self, x: u32, y: u32) -> u32 {
        self.board_array[x][y].Status
    }

    //Get the value of a board position
    pub fn getValue(&self, x: u32, y: u32) -> u32 {
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
