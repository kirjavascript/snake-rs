mod snake;
use snake::Snake;
use sdl2_sys::*;

mod rand {
    pub fn random() -> u32 {
        unsafe { random() as _ }
    }
}

fn s(s: &[u8]) -> *const u8 {
    s as *const _ as *const u8
}

fn main() {
    unsafe {
        let mut snake = Snake::new(64, 48, false);

        SDL_Init(SDL_INIT_EVERYTHING as u32);
        TTF_Init();
        let window = SDL_CreateWindow(s(b"snake-rs"), 0, 0, 1280, 720, 0);
        let renderer = SDL_CreateRenderer(window, 0, 2 | 4);
        SDL_SetRenderDrawBlendMode(renderer, 1);
        SDL_SetHint(s(b"SDL_RENDER_SCALE_QUALITY"), s(b"2"));

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

            SDL_SetRenderDrawColor(renderer, 0, 0, 0, 255);
            SDL_RenderClear(renderer);

            let scale = 15i32;
            for (i, pixel) in snake.get_rgba().chunks(4).enumerate() {
                SDL_SetRenderDrawColor(
                    renderer,
                    pixel[0],
                    pixel[1],
                    pixel[2],
                    pixel[3],
                );
                SDL_RenderFillRect(renderer, &SDL_Rect {
                    x: (scale * (i as i32 % 64)),
                    y: scale * (i as i32 / 64),
                    w: scale,
                    h: scale,
                });
            }

            SDL_RenderPresent(renderer);
        }

        SDL_DestroyRenderer(renderer);
        SDL_DestroyWindow(window);
        TTF_Quit();
        SDL_Quit();
    }
}
