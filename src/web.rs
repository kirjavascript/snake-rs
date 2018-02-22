#[macro_use]
extern crate stdweb;

use stdweb::Value;
use stdweb::web::*;
use stdweb::web::html_element::CanvasElement;
use stdweb::unstable::TryInto;
// use stdweb::traits::*;

mod snake;
use snake::Snake;

mod rand {
    pub fn random() -> u32 {
        // (js!{
        //     return 0|Math.random() * 0xFFFF;
        // }).into_string().unwrap().parse::<u32>().unwrap()
        4
    }
}

fn main() {
    stdweb::initialize();

    // TODO: snake.kirjava.xyz

    let canvas: CanvasElement = document().query_selector("canvas").unwrap().unwrap().try_into().unwrap();
    let ctx: CanvasRenderingContext2d = canvas.get_context().unwrap();

    let mut snake = Snake::new(64, 48);

    window().request_animation_frame( move |_| {
        snake.step();
        let board: Value = snake.get_rgba().into();
        js! {
            const board = new ImageData(
                Uint8ClampedArray.from(@{board}),
                64,
                48,
            );
            @{ctx}.putImageData(board, 0, 0);
        }
    });

    stdweb::event_loop();
}
