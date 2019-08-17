use std::io;

const SIDE_LENGTH: usize = 4;

#[derive(Copy, Clone, PartialEq)]
enum TTTCell {
    Invalid,
    Empty,
    Nought,
    Cross,
}

struct TTTGrid {
    grid: [TTTCell; SIDE_LENGTH * SIDE_LENGTH],
}

impl TTTCell {
    fn is_player(&self) -> bool {
        match self {
            TTTCell::Cross => true,
            TTTCell::Nought => true,
            _ => false,
        }
    }
}

impl TTTGrid {
    fn new() -> TTTGrid {
        TTTGrid {
            grid: [TTTCell::Empty; SIDE_LENGTH * SIDE_LENGTH],
        }
    }

    fn get_cell(&self, x: usize, y: usize) -> Option<&TTTCell> {
        let index: usize = (y * SIDE_LENGTH) + x;

        self.grid.get(index)
    }

    fn set_cell(&mut self, x: usize, y: usize, state: TTTCell) -> Result<(), ()> {
        let index: usize = (y * SIDE_LENGTH) + x;
        if self.get_cell(x, y).unwrap_or(&TTTCell::Invalid) != &TTTCell::Empty {
            Result::Err(())
        } else {
            self.grid[index] = state;
            Result::Ok(())
        }
    }

    fn check_winner(&self) -> TTTCell {
        //Check rows
        for row in 0..SIDE_LENGTH {
            let contender = self.get_cell(0, row).unwrap();
            let mut is_won = true;

            if !contender.is_player() {
                continue;
            }

            for col in 1..SIDE_LENGTH {
                if self.get_cell(col, row).unwrap() != contender {
                    is_won = false;
                    break;
                }
            }
            if is_won {
                return *contender;
            }
        }

        //Check columns
        for col in 0..SIDE_LENGTH {
            let contender = self.get_cell(col, 0).unwrap();
            let mut is_won = true;

            if !contender.is_player() {
                continue;
            }

            for row in 1..SIDE_LENGTH {
                if self.get_cell(col, row).unwrap() != contender {
                    is_won = false;
                    break;
                }
            }
            if is_won {
                return *contender;
            }
        }

        // Diagonal pos
        let contender = self.get_cell(0, 0).unwrap();
        if contender.is_player() {
            let mut is_won = true;
            for i in 1..SIDE_LENGTH {
                if self.get_cell(i, i).unwrap() != contender {
                    is_won = false;
                    break;
                }
            }
            if is_won {
                return *contender;
            }
        }

        //Diagonal neg
        let contender = self.get_cell(SIDE_LENGTH-1, 0).unwrap();
        if contender.is_player() {
            let mut is_won = true;
            for i in 1..SIDE_LENGTH {
                if self.get_cell(SIDE_LENGTH - (i + 1), i).unwrap() != contender {
                    is_won = false;
                    break;
                }
            }
            if is_won {
                return *contender;
            }
        }

        TTTCell::Empty
    }
}

impl std::fmt::Display for TTTCell {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            TTTCell::Cross => write!(f, "X"),
            TTTCell::Nought => write!(f, "O"),
            _ => write!(f, " "),
        }
    }
}

impl std::fmt::Display for TTTGrid {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        //Header
        write!(f, " .")?;
        for i in 0..SIDE_LENGTH {
            write!(f, "{}.", i)?;
        }
        writeln!(f, "")?;

        //Row by row
        for row in 0..SIDE_LENGTH {
            write!(f, "{}.", row)?;
            for col in 0..SIDE_LENGTH {
                write!(f, "{}.", self.get_cell(col, row).unwrap())?;
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}

fn main() {
    let mut grid = TTTGrid::new();

    let mut cur_turn = TTTCell::Cross;
    while cur_turn != TTTCell::Empty {
        println!("{}", grid);

        let winner = grid.check_winner();
        if winner != TTTCell::Empty {
            println!("Winner! The winner is {}", winner);
            break;
        }

        let mut x = String::new();
        let mut y = String::new();

        println!("It's {}'s turn! Type x coord: ", cur_turn);
        io::stdin().read_line(&mut x).expect("Failed to read line");
        println!("Type y coord: ");
        io::stdin().read_line(&mut y).expect("Failed to read line");

        let x = x.trim().parse::<usize>();
        let y = y.trim().parse::<usize>();

        if x.is_err() || y.is_err() {
            println!("Coordinates must be integers in range 0-{}", SIDE_LENGTH);
            continue;
        }

        let x = x.unwrap();
        let y = y.unwrap();

        if grid.set_cell(x, y, cur_turn).is_err() {
            println!("Invalid cell selected!");
            continue;
        }

        cur_turn = match cur_turn {
            TTTCell::Cross => TTTCell::Nought,
            TTTCell::Nought => TTTCell::Cross,
            _ => TTTCell::Empty,
        }
    }
}
