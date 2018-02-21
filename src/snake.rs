#[derive(Debug)]
pub struct Snake {
    facing: Direction,
    head: Point,
    tail: Vec<Point>,
    // fruit: Point,
    width: u32,
    height: u32,
    //score
    //color_seed
}
#[derive(Debug)]
pub struct Point {
    x: i32,
    y: i32,
}
#[derive(Debug)]
pub enum Direction {
    Up, Down, Left, Right,
}

impl Point {
    fn clone(&self) -> Self {
        Point {
            x: self.x,
            y: self.y,
        }
    }
}

impl Snake {
    pub fn new(width: u32, height: u32) -> Self {
        Snake {
            facing: Direction::Right,
            head: Point { x: 0, y: 0 },
            tail: Vec::new(),
            width,
            height,
        }
    }

    // stop
    // restart

    pub fn step(&mut self) {
        // clone head to tail

        // move head
        match self.facing {
            Direction::Right => { self.head.x += 1 },
            Direction::Down => { self.head.y += 1 },
            Direction::Up => { self.head.y -= 1 },
            Direction::Left => { self.head.x -= 1 },
        };

        // wrap
        if self.head.x >= self.width as i32 {
            self.head.x = 0;
        }
        else if self.head.x < 0 {
            self.head.x = self.width as i32 -1;
        }
        else if self.head.y >= self.height as i32 {
            self.head.y = 0;
        }
        else if self.head.y < 0 {
            self.head.y = self.height as i32 -1;
        }

        // check collision with tail
        // check collision with fruit
        // pop tail
    }

    pub fn change_direction(&mut self, direction: Direction) {
        self.facing = direction;
    }

    fn cell_qty(&self) -> usize {
        (self.width * self.height) as usize
    }

    pub fn get_board(&self) -> Vec<bool> {
        let mut board = Vec::with_capacity(self.cell_qty());

        for y in 0..self.height as i32 {
            for x in 0..self.width as i32 {
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
    pub fn get_rgb(&self) -> Vec<u8> {
        let mut rgb = Vec::with_capacity(self.cell_qty() * 3);

        for cell in self.get_board() {
            // TODO: lerp differently each pixel
            if cell {
                rgb.push(0);
                rgb.push(255);
                rgb.push(0);
            }
            else {
                rgb.push(0x60);
                rgb.push(0x60);
                rgb.push(0x60);
            }
        }

        rgb
    }
}
