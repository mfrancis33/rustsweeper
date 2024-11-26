use rand::Rng;

struct Position(i32, i32);

pub struct Board {
	width: i32,
	height: i32,
	mines_total: i32,
	flags_left: i32,
	spaces: Vec<Vec<Space>>
}

impl Board {
	pub fn generate_board(width: i32, height: i32, mines: i32) -> Board {
		let mut board = Board {
			width: width,
			height: height,
			mines_total: mines,
			flags_left: mines,
			spaces: Vec::<Vec<Space>>::new()
		};

		// Fill spaces vec with blank spaces
		for x in 0..width {
			let mut list = Vec::<Space>::new();
			for y in 0..height {
				list.push(Space {
					state: State::Hidden(false),
					value: 0,
					pos: Position(x, y)
				});
			}
		}

		// Generate mines
		let mut to_place = mines;
		while to_place > 0 {
			let x = rand::thread_rng().gen_range(0..width);
			let y = rand::thread_rng().gen_range(0..height);

			match board.get_space_at(x, y) {
				None => continue,
				Some(space) => {
					if space.is_mine() { continue; }
					space.value = -1;
					to_place -= 1;
				}
			}
		}

		// Update numbers across the board
		for list in &mut board.spaces {
			for space in mut list {
				space.determine_value(&mut board);
			}
		}
	
		return board;
	}

	pub fn try_get_space(&self, input: &String) -> &Space {
		// TODO: make space parsing
		&self.spaces[0][0]
	}

	fn get_space_at<'a>(&'a mut self, x: i32, y: i32) -> Option<&'a mut Space> {
		if x >= 0 && x < self.width && y >= 0 && y <= self.height {
			return Some(&mut self.spaces[x as usize][y as usize]);
		}
		return None;
	}

	pub fn print(&self) {
		//
	}
}

pub enum State {
	Hidden(bool), 
	Revealed,
}

pub struct Space {
	state: State,
	value: i8, // below 0 is mine
	pos: Position
}

impl Space {
	pub fn is_mine(&self) -> bool {
		return self.value < 0;
	}

	// returns true on game over
	pub fn reveal(&mut self, board: &mut Board) -> bool {
		return match self.state {
			State::Revealed => false,
			State::Hidden(flagged) => {
				// If it's flagged we skip
				if flagged { return false; }

				// Reveal ourselves and figure out if we're a mine or if we need to reveal other spaces
				self.state = State::Revealed;
				if self.is_mine() { return true; }
				if self.value > 0 { return false; }

				// We need to reveal other spaces.
				let surrounding = [
					board.get_space_at(self.pos.0 - 1, self.pos.1 - 1),
					board.get_space_at(self.pos.0,     self.pos.1 - 1),
					board.get_space_at(self.pos.0 + 1, self.pos.1 - 1),
					
					board.get_space_at(self.pos.0 - 1, self.pos.1),
					board.get_space_at(self.pos.0 + 1, self.pos.1),

					board.get_space_at(self.pos.0 - 1, self.pos.1 + 1),
					board.get_space_at(self.pos.0,     self.pos.1 + 1),
					board.get_space_at(self.pos.0 + 1, self.pos.1 + 1)
				];
				for space in surrounding {
					match space {
						None => continue,
						Some(s) => _ = s.reveal(board) // this will do nothing if already revealed
						// From the logic of how the numbers work, this will reveal no mines, thus we discard the value
					}
				}
				return false;
			}
		}
	}

	pub fn toggle_flag(&mut self) {
		self.state = match self.state {
			State::Hidden(b) => State::Hidden(!b),
			State::Revealed => State::Revealed,
		};
	}

	fn get_char(&self) -> char {
		return match self.state {
			State::Hidden(b) => if b {'?'} else {'#'},
			State::Revealed => {
				if self.is_mine() {
					return '*';
				} else if self.value == 0 {
					return ' ';
				} else {
					return self.value
						.to_string()
						.chars()
						.nth(0)
						.expect("Integer could not be converted to string!");
				}
			}
		}

	}

	fn determine_value(&mut self, board: &mut Board) {
		if !self.is_mine() {
			let mut value = 0;

			/*let surrounding = [
				board.get_space_at(self.pos.0 - 1, self.pos.1 - 1),
				board.get_space_at(self.pos.0,     self.pos.1 - 1),
				board.get_space_at(self.pos.0 + 1, self.pos.1 - 1),

				board.get_space_at(self.pos.0 - 1, self.pos.1),
				board.get_space_at(self.pos.0 + 1, self.pos.1),

				board.get_space_at(self.pos.0 - 1, self.pos.1 + 1),
				board.get_space_at(self.pos.0,     self.pos.1 + 1),
				board.get_space_at(self.pos.0 + 1, self.pos.1 + 1)
			];*/
			for space in surrounding {
				match space {
					None => continue,
					Some(s) => {
						if s.is_mine() {
							value += 1;
						}
					}
				}
			}

			self.value = value;
		}
	}
}
