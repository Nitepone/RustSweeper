//
// square.rs
// Copyright (C) 2018 Nitepone
// Distributed under terms of the MIT license.
//

use std::rc::Rc;

#[derive(PartialEq)]
#[derive(Debug)]
pub enum Status{
    Opened,
    Flagged,
    Covered
}

#[derive(PartialEq)]
#[derive(Debug)]
pub enum Contents{
    Empty,
    Mine
}

pub struct Square<>{
    adjacent_squares: Vec<Rc<Square>>,
    status: Status,
    contents: Contents,
}

impl Square{
    //A simple constructor
    pub fn new(c: Contents) -> Square {
        Square {
            status: Status::Covered,
            contents: c,
            adjacent_squares: Vec::new()
        }
    }

    // Adds an adjacent Square to a Square
    // If there is no room for an adjacent, you will be ignored
    pub fn add_adjacent(&mut self, adjacent: Square){
        self.adjacent_squares.push(Rc::new(adjacent))
    }

    // Opens a square if it is not already
    // Returns false if the square is Opened, otherwise true
    pub fn open(&mut self) -> bool {
        if self.status == Status::Opened {
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
        match self.status {
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
    pub fn get_value(&self) -> u32 {
        let mut count = 0;
        for adjacent in self.adjacent_squares.iter(){
            //let curr_adjacent = Rc::try_unwrap(Rc::clone(adjacent));
            match adjacent.contents {
                Contents::Mine => count+=1,
                _ => (),
            }
        }
        count
    }

}



#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn construction_test() {
        let test_square = Square::new(Contents::Mine);
        assert_eq!(test_square.contents, Contents::Mine);
	}
}
