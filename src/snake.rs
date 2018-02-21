use rand;

pub struct Snake {
    facing: Direction,
    head: Point,
    tail: Vec<Point>,
    fruit: Point,
    width: u32,
    height: u32,
    score: u64,
    running: bool,
    increase_amount: u8,
    //color_seed
}

#[derive(PartialEq)]
pub struct Point {
    x: i32,
    y: i32,
}

#[derive(PartialEq)]
pub enum Direction {
    Up, Down, Left, Right,
}

impl Direction {
    fn is_horiz(direction: &Direction) -> bool {
        direction == &Direction::Right || direction == &Direction::Left
    }
    fn is_vert(direction: &Direction) -> bool {
        direction == &Direction::Up || direction == &Direction::Down
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
        Point {
            x: (rand::random::<u32>() % width) as i32,
            y: (rand::random::<u32>() % height) as i32,
        }
    }
}

impl Snake {
    pub fn new(width: u32, height: u32) -> Self {
        Snake {
            facing: Direction::Right,
            head: Point { x: 0, y: 0 },
            tail: Vec::new(),
            fruit: Point::random(width, height),
            width,
            height,
            score: 0,
            increase_amount: 0,
            running: true,
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
                let _end = self.tail.splice(..1, vec![]);
            }
        }
    }

    fn cell_qty(&self) -> usize {
        (self.width * self.height) as usize
    }

    pub fn change_direction(&mut self, direction: Direction) {
        // check vertical axis
        let horiz = Direction::is_horiz(&direction)
            && Direction::is_horiz(&self.facing);
        let vert = Direction::is_vert(&direction)
            && Direction::is_vert(&self.facing);

        if self.score == 0 || !horiz && !vert {
            self.facing = direction;
        }
    }

    pub fn get_score(&self) -> u64 {
        self.score
    }

    pub fn is_running(&self) -> bool {
        self.running
    }

    pub fn get_board(&self) -> Vec<bool> {
        let mut board = Vec::with_capacity(self.cell_qty());

        for y in 0..self.height as i32 {
            for x in 0..self.width as i32 {
                // TODO: improve O(n^2)
                let point = Point{ x, y };
                let head_col = self.head.collides(&point);
                let fruit_col = self.fruit.collides(&point);
                let tail_col = self.tail.contains(&Point { x, y });
                let value = if head_col || fruit_col || tail_col {
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

    pub fn get_rgb(&mut self) -> Vec<u8> {
        let mut rgb = Vec::with_capacity(self.cell_qty() * 3);

        for cell in self.get_board() {
            // TODO: lerp differently each pixel
            if cell && self.running {
                rgb.push(0);
                rgb.push(200);
                rgb.push(0);
            }
            else if cell && !self.running {
                rgb.push(0);
                rgb.push(0);
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
