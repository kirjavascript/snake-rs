#[derive(Debug)]
pub struct Snake {
    facing: Direction,
    head: Point,
    // tail: Vec<Point>,
    // fruit: Point,
    width: u32,
    height: u32,
    //score
}
#[derive(Debug)]
pub struct Point {
    x: u32,
    y: u32,
}
#[derive(Debug)]
pub enum Direction {
    Up, Down, Left, Right,
}

impl Snake {
    pub fn new(width: u32, height: u32) -> Self {
        Snake {
            facing: Direction::Right,
            head: Point { x: 0, y: 0 },
            width,
            height,
        }
    }

    pub fn step(&mut self) {
        self.head.x = self.head.x + 1;

        if self.head.x >= self.width {
            self.head.x = 0;
        }

        // check collision with tail
        // check collision with fruit
    }

    pub fn change_direction(&mut self, direction: Direction) {
        self.facing = direction;
    }

    fn cell_qty(&self) -> usize {
        (self.width * self.height) as usize
    }

    pub fn get_board(&self) -> Vec<bool> {
        let mut board = Vec::with_capacity(self.cell_qty());

        for y in 0..self.height {
            for x in 0..self.width {
                let value = if self.head.x == x && self.head.y == y {
                    true
                }
                else {
                    false
                };

                board.push(value);
            }
        }

        board
    }
    pub fn get_rgb(&self) -> Vec<u8> { // take colour seed?
        let mut rgb = Vec::with_capacity(self.cell_qty() * 3);

        for cell in self.get_board() {
            if cell {
                rgb.push(0);
                rgb.push(255);
                rgb.push(0);
            }
            else {
                rgb.push(0);
                rgb.push(0);
                rgb.push(0);
            }
        }

        rgb
    }
}
