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

    fn set_cell(&mut self, x: u8, y: u8, state: TTTCell) {
        let index: usize = ((y * 3) + x) as usize;
        self.grid[index] = state;
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
    let grid = TTTGrid::new();
    println!("{}", grid);
}
