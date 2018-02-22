// gtk / canvas
// lerp
// score in title

extern crate gtk;
extern crate gdk;
extern crate gdk_pixbuf;
extern crate rand;

use gtk::prelude::*;
use gtk::{DrawingArea};
use gdk::ContextExt;
use gdk_pixbuf::Pixbuf;

use std::cell::RefCell;
use std::rc::Rc;

mod snake;
use snake::Snake;

static TITLE: &str = "snake-rs";

fn main() {
    if gtk::init().is_err() {
        panic!("Failed to initialize GTK.");
    }

    // init stuff
    let window = gtk::Window::new(gtk::WindowType::Toplevel);

    window.set_title(TITLE);
    window.set_position(gtk::WindowPosition::Center);
    window.set_default_size(640, 480);
    window.set_role("__float");

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    let canvas = DrawingArea::new();
    window.add(&canvas);
    window.show_all();

    // load snake
    let snake = Rc::new(RefCell::new(Snake::new(64, 48)));

    // drawing callback
    let snake_draw_clone = snake.clone();
    canvas.connect_draw(move |_, ctx| {
        let pixels = Pixbuf::new_from_vec(
            snake_draw_clone.borrow_mut().get_rgba(),
            0, // colourspace
            true, // has_alpha
            8, // bits_per_sample
            64, // width
            48, // height
            64 * 4, // row_stride (Distance in bytes between row starts)
        );
        let pixels_scaled = pixbuf_scale(pixels, 64, 48, 10);
        ctx.set_source_pixbuf(&pixels_scaled, 0f64, 0f64);
        ctx.paint();
        Inhibit(false)
    });

    // input
    let snake_input_clone = snake.clone();
    window.connect_key_press_event(move |_, e| {
        if snake_input_clone.borrow().is_running() {
            let change = match e.get_keyval() {
                65362 => Some(snake::Direction::Up),
                65364 => Some(snake::Direction::Down),
                65361 => Some(snake::Direction::Left),
                65363 => Some(snake::Direction::Right),
                _ => None,
            };
            if change.is_some() {
                snake_input_clone.borrow_mut().change_direction(change.unwrap());
            }
        }
        else if e.get_keyval() == 32 {
            snake_input_clone.borrow_mut().restart();
        }
        Inhibit(false)
    });

    // step loop
    let tick = move || {
        // set score
        let mut new_title = format!("{} - score: {}", TITLE, snake.borrow().get_score());
        if !snake.borrow().is_running() {
            new_title.push_str(" - press SPACE to restart");
        }
        else {
            // step & draw
            snake.borrow_mut().step();
            canvas.queue_draw();
        }

        window.set_title(&new_title);
        gtk::Continue(true)
    };
    gtk::timeout_add(45, tick);

    // gtk::idle_add
    gtk::main();
}

fn pixbuf_scale(buf: Pixbuf, width: i32, height: i32, scale: i32) -> Pixbuf {
    let scaled_width = width * scale;
    let scaled_height = height * scale;
    let pixbuf_scale = Pixbuf::new_from_vec(
        vec![0; (scaled_width * scaled_height * 4) as usize],
        0, true, 8,
        scaled_width, scaled_height, scaled_width * 4
    );
    buf.scale(&pixbuf_scale,
        0, 0, scaled_width, scaled_height,
        0., 0., scale as f64, scale as f64, 0,
    );
    pixbuf_scale
}
