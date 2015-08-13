#![cfg_attr(not(feature = "gtk_3_10"), allow(unused_variables, unused_mut))]

extern crate gtk;
extern crate cairo;

use std::rc::Rc;
use std::cell::RefCell;
use gtk::traits::*;
use gtk::signal::Inhibit;
use cairo::{Context, RectangleInt};


struct RenderingState {
    width: i32,
    height: i32,
}

impl RenderingState {
    fn on_draw(&self, cairo_ctx: Context) {
        cairo_ctx.save();
        cairo_ctx.move_to(50.0, (self.height as f64) * 0.5);
        cairo_ctx.set_font_size(18.0);
        cairo_ctx.show_text("The only curse they could afford to put on a tomb these days was 'Bugger Off'. --PTerry");
        cairo_ctx.restore();
    }

    fn on_size_allocate(&mut self, rect: &RectangleInt) {
        self.width = rect.width as i32;
        self.height = rect.height as i32;
    }
}


struct RenderingAPITestWindow {
    window: gtk::Window,
    drawing_area: gtk::DrawingArea,
    state: RefCell<RenderingState>,
}

impl RenderingAPITestWindow {
    fn new(width: i32, height: i32) -> Rc<RenderingAPITestWindow> {
        let window = gtk::Window::new(gtk::WindowType::TopLevel).unwrap();
        let drawing_area = gtk::DrawingArea::new().unwrap();
        drawing_area.set_size_request(width, height);
        window.set_title("Cairo API test");
        window.add(&drawing_area);

        let instance = Rc::new(RenderingAPITestWindow {
            window: window,
            drawing_area: drawing_area,
            state: RefCell::new(RenderingState {
                width: width,
                height: height,
            }),
        });

        {
            let instance2 = instance.clone();
            instance.drawing_area.connect_draw(move |widget, cairo_context| {
                instance2.state.borrow().on_draw(cairo_context);
                instance2.drawing_area.queue_draw();
                Inhibit(true)
            });
        }
        {
            let instance2 = instance.clone();
            instance.drawing_area.connect_size_allocate(move |widget, rect| {
                instance2.state.borrow_mut().on_size_allocate(rect);
            });
        }
        instance.window.show_all();
        instance
    }

    fn exit_on_close(&self) {
        self.window.connect_delete_event(|_, _| {
            gtk::main_quit();
            Inhibit(true)
        });
    }
}

fn main() {
    gtk::init().unwrap_or_else(|_| panic!("Failed to initialize GTK."));
    println!("Major: {}, Minor: {}", gtk::get_major_version(), gtk::get_minor_version());

    let window = RenderingAPITestWindow::new(800, 500);
    window.exit_on_close();
    gtk::main();
}
