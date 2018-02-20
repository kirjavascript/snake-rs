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

    // step loop
    let tick = move || {
        snake.borrow_mut().step();

        let pixels = Pixbuf::new_from_vec(
            snake.borrow_mut().get_rgb(),
            0, // colourspace
            false, // has_alpha
            8, // bits_per_sample
            64, // width
            48, // height
            64 * 3, // row_stride
        );

        canvas.connect_draw(move |_, ctx| {
            let scale = 10;
            let pixbuf_scale: Pixbuf = unsafe {
                Pixbuf::new(0, false, 8, 64*scale, 48*scale).unwrap()
            };

        // let pixbuf_scale = Pixbuf::new_from_vec(vec![0; 640 * 480], 0, false, 8, 640, 480, 64 * 3, );
            pixels.scale(
                &pixbuf_scale,
                0,
                0,
                64*scale,
                48*scale,
                0.,
                0.,
                scale as f64,
                scale as f64,
                1,
            );
            ctx.set_source_pixbuf(&pixbuf_scale, 0f64, 0f64);
            ctx.paint();
            Inhibit(false)
        });

        canvas.queue_draw();

        println!("tick");

        gtk::Continue(true)
    };
    gtk::timeout_add(50, tick);
    gtk::main();
}
