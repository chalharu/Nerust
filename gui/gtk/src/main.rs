#[macro_use]
extern crate log;

mod glarea;
mod window;

use self::window::{Window, WindowExtend};
use gio::prelude::*;
use gtk::prelude::*;
use nerust_console::Console;
use nerust_core::controller::standard_controller::Buttons;
use nerust_screen_buffer::ScreenBuffer;
use nerust_screen_opengl::GlView;
use nerust_screen_traits::{LogicalSize, PhysicalSize};
use nerust_sound_openal::OpenAl;
use nerust_timer::CLOCK_RATE;
use std::cell::RefCell;
use std::rc::Rc;

pub struct State {
    view: Option<GlView>,
    running: bool,
    keys: Buttons,
    paused: bool,
    console: Console,
    physical_size: PhysicalSize,
    logical_size: LogicalSize,
}

impl State {
    pub fn new(screen_buffer: ScreenBuffer) -> Self {
        let physical_size = screen_buffer.physical_size();
        let logical_size = screen_buffer.logical_size();
        let speaker = OpenAl::new(48000, CLOCK_RATE as i32, 128, 20);
        let console = Console::new(speaker, screen_buffer);
        Self {
            view: None,
            console,
            running: true,
            keys: Buttons::empty(),
            paused: false,
            physical_size,
            logical_size,
        }
    }
}

fn app_activate(app: &gtk::Application) {
    let ui = include_str!("../resources/ui.xml");
    let builder = gtk::Builder::new_from_string(ui);

    let window: gtk::ApplicationWindow = builder.get_object("window").unwrap();
    let state: Rc<RefCell<Option<State>>> = Rc::new(RefCell::new(None));

    app.set_menubar(&builder.get_object::<gio::Menu>("menu").unwrap());
    app.add_window(&window);

    let quit_action = gio::SimpleAction::new("quit", None);
    {
        let app = app.clone();
        quit_action.connect_activate(move |_, _| {
            app.quit();
        });
    }
    app.add_action(&quit_action);

    Window::bind(
        app.clone(),
        window,
        builder.get_object("glarea").unwrap(),
        state,
    );
}

fn main() {
    // log initialize
    simple_logger::init().unwrap();

    let app = gtk::Application::new("com.github.chalharu", gio::ApplicationFlags::HANDLES_OPEN)
        .expect("Application start up error");

    app.connect_activate(app_activate);

    app.run(&std::env::args().collect::<Vec<_>>());
}
