mod snake;
use snake::Snake;
use sdl2_sys::*;

mod rand {
    pub fn random() -> u32 {
        unsafe { sdl2_sys::random() as _ }
    }
}

fn s(s: &str) -> *const u8 {
    format!("{}\0", s).as_bytes().as_ptr()
}

fn main() {
    unsafe {
        let mut snake = Snake::new(64, 48, false);

        SDL_Init(SDL_INIT_EVERYTHING as u32);
        TTF_Init();
        nx_sys::romfsMount(s("romfs"));
        let window = SDL_CreateWindow(s("snake-rs"), 0, 0, 1280, 720, 0);
        let renderer = SDL_CreateRenderer(window, 0, 2 | 4);
        SDL_SetRenderDrawBlendMode(renderer, 1);
        SDL_SetHint(s("SDL_RENDER_SCALE_QUALITY"), s("2"));

        let scale = 15i32;
	let font = TTF_OpenFont(s("romfs:/small_font.ttf"), 24);

        let mut fc = 0;
        loop {
            fc += 1;
            if fc % 4 == 0 {
                snake.step();
            }

            // input

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

            // clear screen

            SDL_SetRenderDrawColor(renderer, 0, 0, 0, 255);
            SDL_RenderClear(renderer);

            // HUD

        // let surface = TTF_RenderText_Solid(font, s("test"), SDL_Color {
        //     r: 255,
        //     g: 255,
        //     b: 255,
        //     a: 255,
        // });
        // let texture = SDL_CreateTextureFromSurface(renderer, surface);
        // SDL_RenderCopy(renderer, texture, core::ptr::null(), &SDL_Rect {
        //     x: scale * 0,
        //     y: 0,
        //     w: (*surface).w,
        //     h: (*surface).h,
        // });

            // draw playfield
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
