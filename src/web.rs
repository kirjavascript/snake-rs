#[macro_use]
extern crate stdweb;

use stdweb::Value;
use stdweb::web::*;
use stdweb::web::html_element::CanvasElement;
use stdweb::unstable::TryInto;

use std::cell::RefCell;
use std::rc::Rc;

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
    // TODO: https://rustbyexample.com/attribute/cfg.html
    // https://github.com/koute/pinky/blob/master/pinky-web/src/main.rs#L478

    let canvas: CanvasElement = document().query_selector("canvas").unwrap().unwrap().try_into().unwrap();
    let ctx: CanvasRenderingContext2d = canvas.get_context().unwrap();

    let snake = Rc::new(RefCell::new(Snake::new(64, 48)));
    async_render_loop(snake, ctx);
    stdweb::event_loop();
}

fn async_render_loop(snake: Rc<RefCell<Snake>>, ctx: CanvasRenderingContext2d) {
    window().request_animation_frame( move |_| {
        snake.borrow_mut().step();

        let board: Value = snake.borrow_mut().get_rgba().into();
        js! {
            @{&ctx}.putImageData(new ImageData(
                // TODO: avoid this clone
                Uint8ClampedArray.from(@{board}),
                64,
                48,
            ), 0, 0);
        }

        async_render_loop(snake, ctx);
    });
}
