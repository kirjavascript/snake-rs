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

    // TODO: snake.kirjava.xyz / play online

    // load snake, canvas
    let canvas: CanvasElement = document().query_selector("canvas").unwrap().unwrap().try_into().unwrap();
    let ctx: CanvasRenderingContext2d = canvas.get_context().unwrap();
    let snake = Rc::new(RefCell::new(Snake::new(64, 48)));

    // drawing
    async_render_loop(snake.clone(), ctx);

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
        else if e.key().as_str() == " " {
            snake_input_clone.borrow_mut().restart();
        }
    });

    stdweb::event_loop();
}

fn async_render_loop(snake: Rc<RefCell<Snake>>, ctx: CanvasRenderingContext2d) {
    // TODO: set_timeout can cause lag due to how tasks are queued in the event loop
    // replace with requestAnimationFrame + performance.now()
    set_timeout(move || {

        // step n draw
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
    }, 45);
}
