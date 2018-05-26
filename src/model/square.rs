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
    pub fn add_adjacent(Square: adjacent){
        for i in self.adjacent_squares.iter() {
            if self.adjacent_squares[i] == null {
                self.adjacent_squares[i] = adjacent;
            }
        }
    }

    // Opens a square if it is not already
    // Returns false if the square is Opened, otherwise true
    pub fn open(){
        if self.status == Status:Opened {
            return false;
        }
        self.status = Status::Opened;
        return true;
    }

    // Toggles the flag status
    // Square must be in the Covered or Flagged state
    // Returns false if the flag is not Covered or Flagged, otherwise true
    pub fn toggle_flag(){
        match self.Status {
            Status::Covered => self.status = Status::Flagged,
            Status::Flagged => self.status = Status::Covered,
            //Catch other Status (Can't flag Opened tiles)
            _ => return false,
        }
        //If we get here, we had success
        return true;
    }
}



#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn it_works() {
	}
}
