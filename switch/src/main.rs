// extern crate sdl2_sys;
// extern crate nx;

mod snake;
use snake::Snake;

mod rand {
    pub fn random() -> u32 {
        unsafe { sdl2_sys::random() as _ }
    }
}


fn main() {
    unsafe {
        let mut snake = Snake::new(64, 48, false);

        sdl2_sys::SDL_Init(sdl2_sys::SDL_INIT_EVERYTHING as u32);
        let window = sdl2_sys::SDL_CreateWindow(b"snake-rs" as *const _ as *const u8, 0, 0, 1280, 720, 0);
        let renderer = sdl2_sys::SDL_CreateRenderer(window, 0, 2 | 4);
        sdl2_sys::SDL_SetRenderDrawBlendMode(renderer, 1);
        sdl2_sys::SDL_SetHint(b"SDL_RENDER_SCALE_QUALITY" as *const _ as *const u8, b"2" as *const _ as *const u8);

        let mut fc = 0;
        loop {
            fc += 1;
            if fc % 4 == 0 {
                snake.step();
            }

            let ipt = nx::hid::input_down(nx::hid::Controller::Auto);
            if nx::input_any!(ipt, nx::hid::Key::Minus) {
                break; // minus to exit
            } else if nx::input_any!(ipt, nx::hid::Key::Plus) {
                snake.restart();
            } else if nx::input_any!(ipt, nx::hid::Key::DPadLeft) {
                snake.change_direction(snake::Direction::Left);
            } else if nx::input_any!(ipt, nx::hid::Key::DPadRight) {
                snake.change_direction(snake::Direction::Right);
            } else if nx::input_any!(ipt, nx::hid::Key::DPadUp) {
                snake.change_direction(snake::Direction::Up);
            } else if nx::input_any!(ipt, nx::hid::Key::DPadDown) {
                snake.change_direction(snake::Direction::Down);
            }

            sdl2_sys::SDL_SetRenderDrawColor(renderer, 0, 0, 0, 255);
            sdl2_sys::SDL_RenderClear(renderer);

            let scale = 15i32;
            for (i, pixel) in snake.get_rgba().chunks(4).enumerate() {
                sdl2_sys::SDL_SetRenderDrawColor(
                    renderer,
                    pixel[0],
                    pixel[1],
                    pixel[2],
                    pixel[3],
                );
                sdl2_sys::SDL_RenderFillRect(renderer, &sdl2_sys::SDL_Rect {
                    x: (scale * (i as i32 % 64)),
                    y: scale * (i as i32 / 64),
                    w: scale,
                    h: scale,
                });
            }

            sdl2_sys::SDL_RenderPresent(renderer);
        }

        sdl2_sys::SDL_DestroyRenderer(renderer);
        sdl2_sys::SDL_DestroyWindow(window);
        sdl2_sys::SDL_Quit();
    }
}
