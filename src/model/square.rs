//
// square.rs
// Copyright (C) 2018 Nitepone
// Distributed under terms of the MIT license.
//

const MAX_ADJACENT: u8 = 8;

pub enum Status{
    Opened,
    Flagged,
    Covered
}

pub enum Contents{
    Empty,
    Mine
}

pub struct Square{
    adjacent_squares: Square[MAX_ADJACENT],
    status: Status,
    contents: Contents,
    cached_adjacent_mines: u8
}

impl Square{
    // Adds an adjacent Square to a Square
    // If there is no room for an adjacent, you will be ignored
    pub fn add_adjacent(&mut self, Square: adjacent){
        for i in self.adjacent_squares.iter() {
            if self.adjacent_squares[i] == null {
                self.adjacent_squares[i] = adjacent;
            }
        }
    }

    // Opens a square if it is not already
    // Returns false if the square is Opened, otherwise true
    pub fn open(&mut self) -> bool {
        if self.status == Status:Opened {
            false
        } else {
            self.status = Status::Opened;
            true
        }
    }

    // Toggles the flag status
    // Square must be in the Covered or Flagged state
    // Returns false if the flag is not Covered or Flagged, otherwise true
    pub fn toggle_flag(&mut self) -> bool {
        match self.Status {
            Status::Covered => {
                self.status = Status::Flagged;
                true
            },
            Status::Flagged => {
                self.status = Status::Covered;
                true
            },
            //Catch other Status (Can't flag Opened tiles)
            _ => false
        }
    }

    // Counts the number of adjacent mines
    pub fn getValue(&self) -> u32 {
        let u32: count = 0;
        for i in self.adjacent_squares.iter(){
            if self.adjacent_squares[i].Contents == Contents::Mine {i++;}
        }
        i
    }

}



#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn it_works() {
	}
}
