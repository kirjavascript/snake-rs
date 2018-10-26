use rand;

pub struct Snake {
    facing: Direction,
    last_dir: Direction,
    head: Point,
    tail: Vec<Point>,
    fruit: Point,
    width: u32,
    height: u32,
    score: u64,
    running: bool,
    increase_amount: u8,
    color_seed: u16,
}

#[derive(PartialEq)]
pub struct Point {
    x: i32,
    y: i32,
}

enum Item {
    Snake, Fruit, Nothing,
}

#[derive(PartialEq, Clone)]
pub enum Direction {
    Up, Down, Left, Right,
}

impl Direction {
    fn opposite(direction: &Direction) -> Direction {
        match direction {
           &Direction::Up => Direction::Down,
           &Direction::Down => Direction::Up,
           &Direction::Left => Direction::Right,
           &Direction::Right => Direction::Left
        }
    }
}

impl Point {
    fn clone(&self) -> Self {
        Point {
            x: self.x,
            y: self.y,
        }
    }

    fn collides(&self, point: &Point) -> bool {
        self.x == point.x && self.y == point.y
    }

    fn random(width: u32, height: u32) -> Self {
        let rnd1: u32 = rand::random();
        let rnd2: u32 = rand::random();
        Point {
            x: (rnd1 % width) as i32,
            y: (rnd2 % height) as i32,
        }
    }
}

impl Snake {
    pub fn new(width: u32, height: u32) -> Self {
        Snake {
            facing: Direction::Right,
            last_dir: Direction::Right,
            head: Point::random(width, height),
            tail: Vec::new(),
            fruit: Point::random(width, height),
            width,
            height,
            score: 0,
            increase_amount: 0,
            running: true,
            color_seed: 0,
        }
    }

    pub fn restart(&mut self) {
        self.score = 0;
        self.running = true;
        self.tail = Vec::new();
    }

    pub fn step(&mut self) {
        if self.running {
            // clone head to tail
            self.tail.push(self.head.clone());

            // update last direction
            self.last_dir = self.facing.clone();

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
            if self.tail.contains(&self.head) {
                self.running = false;
            }
            // check collision with fruit
            else if self.head == self.fruit {
                self.score += 10;
                self.fruit = loop {
                    // create new fruit
                    let fruit = Point::random(self.width, self.height);
                    // ensure the new fruit isn't in the head or tail
                    let head_col = self.head.collides(&fruit);
                    if !self.tail.contains(&fruit) && !head_col {
                        break fruit;
                    }
                };
                // add tail extension amount
                self.increase_amount = 3;
            }
            else if self.increase_amount != 0 {
                self.increase_amount -= 1;
            }
            else {
                // unshift tail
                self.tail.remove(0);
            }
        }
    }

    fn cell_qty(&self) -> usize {
        (self.width * self.height) as usize
    }

    pub fn change_direction(&mut self, direction: Direction) {
        // check if direction is opposite the last direction
        let opposite = Direction::opposite(&direction) == self.last_dir;
        if !opposite {
            self.facing = direction;
        }
    }

    pub fn get_score(&self) -> u64 {
        self.score
    }

    pub fn is_running(&self) -> bool {
        self.running
    }

    fn get_board(&self) -> Vec<Item> {
        let mut board = Vec::with_capacity(self.cell_qty());

        for y in 0..self.height as i32 {
            for x in 0..self.width as i32 {
                // TODO: improve O(n^2)
                let point = Point{ x, y };
                let head_col = self.head.collides(&point);
                let fruit_col = self.fruit.collides(&point);
                let tail_col = self.tail.contains(&Point { x, y });
                let value = if head_col || tail_col {
                    Item::Snake
                }
                else if fruit_col {
                    Item::Fruit
                }
                else {
                    Item::Nothing
                };

                board.push(value);
            }
        }

        board
    }

    fn get_color(&mut self) -> (u8, u8, u8) {
        self.color_seed = self.color_seed.wrapping_add(4);
        let seed = self.color_seed;
        let group = ((seed / 255) % 5) as usize;
        let i = (seed % 255) as u8;
        let groups: [(u8, u8, u8); 5] = [
            (0,255,4),
            (0,81,255),
            (75,0,130),
            (255,141,0),
            (227,255,0),
        ];

        let start = groups[group];
        let end = groups[(group + 1) % 5];

        let r = lerp(start.0, end.0, i);
        let g = lerp(start.1, end.1, i);
        let b = lerp(start.2, end.2, i);

        (r, g, b)
    }

    pub fn get_rgba(&mut self) -> Vec<u8> {
        let mut rgb_board = Vec::with_capacity(self.cell_qty() * 3);
        let (r, g, b) = self.get_color();

        for cell in self.get_board() {
            match cell {
                Item::Fruit => {
                    rgb_board.push(255);
                    rgb_board.push(60);
                    rgb_board.push(60);
                    rgb_board.push(255);
                },
                Item::Nothing => {
                    rgb_board.push(0x60);
                    rgb_board.push(0x60);
                    rgb_board.push(0x60);
                    rgb_board.push(255);
                },
                Item::Snake => {
                    if self.running {
                        rgb_board.push(r);
                        rgb_board.push(g);
                        rgb_board.push(b);
                        rgb_board.push(255);
                    }
                    else {
                        rgb_board.push(0);
                        rgb_board.push(0);
                        rgb_board.push(0);
                        rgb_board.push(255);
                    }
                },
            }
        }

        rgb_board
    }
}

pub fn lerp(start: u8, end: u8, i: u8) -> u8 {
    (start as f32 + ((end as i64 - start as i64) as f32 * (i as f32 / 255.0))) as u8
}
