use std::io;

#[derive(Copy, Clone, PartialEq)]
enum TTTCell {
    Empty,
    Nought,
    Cross,
}

struct TTTGrid {
    grid: [TTTCell; 9],
}

impl TTTGrid {
    fn new() -> TTTGrid {
        TTTGrid {
            grid: [TTTCell::Empty; 9],
        }
    }

    fn get_cell(&self, x: u8, y: u8) -> TTTCell {
        let index: usize = ((y * 3) + x) as usize;

        self.grid[index]
    }

    fn set_cell(&mut self, x: u8, y: u8, state: TTTCell) -> Result<(), ()> {
        let index: usize = ((y * 3) + x) as usize;

        if self.get_cell(x, y) != TTTCell::Empty {
            Result::Err(())
        } else {
            self.grid[index] = state;
            Result::Ok(())
        }
    }

    fn check_winner(&self) -> TTTCell {
        // horiz wins
        for i in 0..2 {
            let index: usize = (i * 3) as usize;

            if self.grid[index] != TTTCell::Empty
                && self.grid[index] == self.grid[index + 1]
                && self.grid[index] == self.grid[index + 2]
            {
                return self.grid[index];
            }
        }

        // vert wins
        for i in 0..2 {
            let index: usize = i as usize;

            if self.grid[index] != TTTCell::Empty
                && self.grid[index] == self.grid[index + 3]
                && self.grid[index] == self.grid[index + 6]
            {
                return self.grid[index];
            }
        }

        // Diagonals
        if self.grid[0] != TTTCell::Empty
            && self.grid[0] == self.grid[4]
            && self.grid[0] == self.grid[8]
        {
            return self.grid[0];
        }
        if self.grid[2] != TTTCell::Empty
            && self.grid[2] == self.grid[4]
            && self.grid[2] == self.grid[6]
        {
            return self.grid[0];
        }

        TTTCell::Empty
    }
}

impl std::fmt::Display for TTTCell {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            TTTCell::Empty => write!(f, " "),
            TTTCell::Cross => write!(f, "X"),
            TTTCell::Nought => write!(f, "O"),
        }
    }
}

impl std::fmt::Display for TTTGrid {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let &grid = &self.grid;
        write!(
            f,
            " .0.1.2.\n0.{}.{}.{}.\n1.{}.{}.{}.\n2.{}.{}.{}.\n",
            grid[0], grid[1], grid[2], grid[3], grid[4], grid[5], grid[6], grid[7], grid[8]
        )
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

        let x = x.trim().parse::<u8>();
        let y = y.trim().parse::<u8>();

        if x.is_err() || y.is_err() {
            println!("Coordinates must be integers in range 0-2");
            continue;
        }

        let x = x.unwrap();
        let y = y.unwrap();

        if x > 2 || y > 2 {
            println!("Coordinates must be integers in range 0-2");
            continue;
        }

        if grid.set_cell(x, y, cur_turn).is_err() {
            println!("Invalid cell selected!");
            continue;
        }

        cur_turn = match cur_turn {
            TTTCell::Cross => TTTCell::Nought,
            TTTCell::Nought => TTTCell::Cross,
            TTTCell::Empty => TTTCell::Empty,
        }
    }
}
