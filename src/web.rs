#[macro_use]
extern crate stdweb;

use stdweb::Value;
use stdweb::web::*;
use stdweb::web::html_element::CanvasElement;
use stdweb::web::event::KeyDownEvent;
use stdweb::traits::*;
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
    //You can, of course, use the js! macro too if you want. (Just remember that any callback which you pass through the js! macro needs to be manually dropped by calling .drop() when it's not used anymore;

    // load snake, canvas
    let canvas: CanvasElement = document().query_selector("canvas").unwrap().unwrap().try_into().unwrap();
    let ctx: CanvasRenderingContext2d = canvas.get_context().unwrap();
    let snake = Rc::new(RefCell::new(Snake::new(64, 48)));

    // input
    let snake_input_clone = snake.clone();
    document().add_event_listener(move |e: KeyDownEvent| {
        if snake_input_clone.borrow().is_running() {
            let change = match e.key().as_str() {
                "ArrowUp" => Some(snake::Direction::Up),
                "ArrowDown" => Some(snake::Direction::Down),
                "ArrowRight" => Some(snake::Direction::Right),
                "ArrowLeft" => Some(snake::Direction::Left),
                _ => None,
            };
            if change.is_some() {
                snake_input_clone.borrow_mut().change_direction(change.unwrap());
            }
        }
    });

    // TODO: is clone needed?
    async_render_loop(snake.clone(), ctx);

    stdweb::event_loop();
}
// pub fn set_timeout<F: FnOnce() + 'static>(callback: F, timeout: u32)

fn async_render_loop(snake: Rc<RefCell<Snake>>, ctx: CanvasRenderingContext2d) {
    window().request_animation_frame(move |timestamp| {
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
