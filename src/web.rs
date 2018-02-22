#[macro_use]
extern crate stdweb;

use stdweb::Value;
use stdweb::web::*;
use stdweb::web::html_element::CanvasElement;
use stdweb::unstable::TryInto;
use stdweb::traits::*;

mod snake;
use snake::Snake;

mod rand {
    pub fn random() -> u32 {
        // (js!{
        //     return 0|Math.random() * 0xFFFF;
        // }).into_string().unwrap().parse::<u32>().unwrap()
        0
    }
}

fn main() {
    stdweb::initialize();

    // TODO: snake.kirjava.xyz

    let canvas: CanvasElement = document().query_selector("canvas").unwrap().unwrap().try_into().unwrap();
    let ctx: CanvasRenderingContext2d = canvas.get_context().unwrap();

    let mut snake = Snake::new(64, 48);
    snake.step();

    let board: Value = snake.get_rgb().into();

    js! {
        const board = new ImageData(
            Uint8ClampedArray.from(@{board}),
            64,
            36, // should be 48
        );
        @{ctx}.putImageData(board, 0, 0);
        console.log(board);
    }

    stdweb::event_loop();
}
