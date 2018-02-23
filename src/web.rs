#[macro_use]
extern crate stdweb;

use stdweb::Value;
use stdweb::web::*;
use stdweb::web::html_element::CanvasElement;
use stdweb::unstable::TryInto;

mod snake;
use snake::Snake;

mod rand {
    use stdweb::unstable::TryInto;
    pub fn random() -> u32 {
        (js!{
            return Math.floor(Math.random() * 0xFFFFFFFF);
        }).try_into().unwrap()
    }
}

fn main() {
    stdweb::initialize();

    // TODO: snake.kirjava.xyz
    // TODO: fix blurry edges in chrome

    let canvas: CanvasElement = document().query_selector("canvas").unwrap().unwrap().try_into().unwrap();
    let ctx: CanvasRenderingContext2d = canvas.get_context().unwrap();

    let mut snake = Snake::new(64, 48);

    let board: Value = snake.get_rgba().into();
    js! {
        const board = new ImageData(
            Uint8ClampedArray.from(@{board}),
            64,
            48,
        );
        @{&ctx}.putImageData(board, 0, 0);
    }

    window().request_animation_frame(move |_| {
        snake.step();
        let board: Value = snake.get_rgba().into();
        js! {
            const board = new ImageData(
                Uint8ClampedArray.from(@{board}),
                64,
                48,
            );
            @{&ctx}.putImageData(board, 0, 0);
        }
    });

    stdweb::event_loop();
}
