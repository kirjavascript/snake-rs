// gtk / canvas
// lerp
// slowly get faster?

extern crate gtk;
extern crate gdk;
extern crate gdk_pixbuf;

use gtk::prelude::*;
use gtk::{Window, Button, Label, DrawingArea};
use gdk::ContextExt;
use gdk_pixbuf::Pixbuf;

fn main() {
    if gtk::init().is_err() {
        panic!("Failed to initialize GTK.");
    }
    let window = gtk::Window::new(gtk::WindowType::Toplevel);

    window.set_title("snake-rs");
    window.set_position(gtk::WindowPosition::Center);
    window.set_default_size(600, 400);
    window.set_role("__float");

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });
    let canvas = DrawingArea::new();
    window.add(&canvas);

    let pixbuf: Pixbuf = unsafe {
        Pixbuf::new(0, false, 8, 600, 400).unwrap()
    };
    pixbuf.put_pixel(0, 0, 255, 0, 0, 0);

    canvas.connect_draw(move |_, ctx| {
        ctx.set_source_pixbuf(&pixbuf, 0f64, 0f64);
        ctx.paint();
        Inhibit(false)
    });

    window.show_all();

    let tick = move || {
        gtk::Continue(true)
    };
    gtk::idle_add(tick);
    gtk::main();
}
