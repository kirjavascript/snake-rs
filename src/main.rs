// gtk / canvas
// lerp
// slowly get faster?
// score in title

extern crate gtk;
extern crate gdk;
extern crate gdk_pixbuf;

use gtk::prelude::*;
use gtk::{DrawingArea};
use gdk::ContextExt;
use gdk_pixbuf::Pixbuf;

use std::cell::RefCell;
use std::rc::Rc;

mod snake;
use snake::Snake;

fn main() {
    if gtk::init().is_err() {
        panic!("Failed to initialize GTK.");
    }
    let window = gtk::Window::new(gtk::WindowType::Toplevel);

    window.set_title("snake-rs");
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

    let mut snake = Rc::new(RefCell::new(Snake::new(64, 48)));

    let snake_draw_clone = snake.clone();

    canvas.connect_draw(move |_, ctx| {
        let pixels = Pixbuf::new_from_vec(
            snake_draw_clone.borrow_mut().get_rgb(),
            0, // colourspace
            false, // has_alpha
            8, // bits_per_sample
            64, // width
            48, // height
            64 * 3, // row_stride
        );
        let pixels_scaled = pixbuf_scale(pixels, 64, 48, 10);
        ctx.set_source_pixbuf(&pixels_scaled, 0f64, 0f64);
        ctx.paint();
        Inhibit(false)
    });

    // step loop
    let tick = move || {
        snake.borrow_mut().step();
        canvas.queue_draw();
        gtk::Continue(true)
    };
    gtk::timeout_add(50, tick);
    gtk::main();
}

fn pixbuf_scale(buf: Pixbuf, width: i32, height: i32, scale: i32) -> Pixbuf {
    let scaled_width = width * scale;
    let scaled_height = height * scale;
    let pixbuf_scale = Pixbuf::new_from_vec(
        vec![0; (scaled_width * scaled_height * 3) as usize],
        0, false, 8,
        scaled_width, scaled_height, scaled_width * 3
    );
    buf.scale(&pixbuf_scale,
        0, 0, scaled_width, scaled_height,
        0., 0., scale as f64, scale as f64, 1,
    );
    pixbuf_scale
}
