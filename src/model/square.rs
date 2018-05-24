//
// square.rs
// Copyright (C) 2018 Nitepone
// Distributed under terms of the MIT license.
//

final int MAX_ADJACENT = 8;

pub enum Status{
    Openned,
    Flagged,
    Covered
}

pub struct Square{
    adjacent_squares: Square[MAX_ADJACENT],
    status: Status,
    value: int,
}



#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn it_works() {
	}
}
