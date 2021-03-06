use super::State;
use gtk::prelude::*;
use nerust_screen_opengl::GlView;
use shared_library::dynamic_library::DynamicLibrary;
use std::cell::RefCell;
use std::ptr;
use std::rc::Rc;

pub(crate) struct GLAreaCore {
    gl_area: gtk::GLArea,
    state: Rc<RefCell<State>>,
}

pub(crate) type GLArea = Rc<RefCell<GLAreaCore>>;

pub(crate) trait GLAreaExtend {
    fn bind(gl_area: gtk::GLArea, state: Rc<RefCell<State>>) -> GLArea;
    fn realize(&self);
    fn resize(&self, width: i32, height: i32);
    fn render(&self) -> bool;
    fn unrealize(&self);
    fn tick(&self) -> bool;
    fn glarea(&self) -> gtk::GLArea;
    fn state(&self) -> Rc<RefCell<State>>;
}

impl GLAreaExtend for GLArea {
    fn glarea(&self) -> gtk::GLArea {
        self.borrow().gl_area.clone()
    }

    fn state(&self) -> Rc<RefCell<State>> {
        self.borrow().state.clone()
    }

    fn bind(gl_area: gtk::GLArea, state: Rc<RefCell<State>>) -> GLArea {
        let result = Rc::new(RefCell::new(GLAreaCore {
            gl_area: gl_area.clone(),
            state,
        }));
        {
            let result = result.clone();
            let _ = gl_area.connect_realize(move |_gl_area| result.realize());
        }
        {
            let result = result.clone();
            let _ = gl_area.connect_resize(move |_gl_area, w, h| {
                result.resize(w, h);
            });
        }
        {
            let result = result.clone();
            let _ = gl_area.connect_render(move |_gl_area, _context| Inhibit(result.render()));
        }
        {
            let result = result.clone();
            let _ = gl_area.connect_unrealize(move |_gl_area| result.unrealize());
        }
        {
            let result = result.clone();
            let _ =
                gl_area.add_tick_callback(move |_gl_area, _frame_clock| Continue(result.tick()));
        }
        result
    }

    fn realize(&self) {
        let mut view = GlView::new();
        view.use_vao(true);
        self.glarea().make_current();
        if let Some(e) = self.glarea().get_error() {
            log::error!("{}", e);
        }
        epoxy::load_with(|s| unsafe {
            match DynamicLibrary::open(None).unwrap().symbol(s) {
                Ok(v) => v,
                Err(e) => {
                    log::error!("{}", e);
                    ptr::null()
                }
            }
        });
        GlView::load_with(epoxy::get_proc_addr);
        {
            let state = self.state();
            let mut state = state.borrow_mut();
            view.on_load(state.logical_size);
            state.view = Some(view);
        }
    }

    fn resize(&self, width: i32, height: i32) {
        self.glarea().make_current();
        if let Some(e) = self.glarea().get_error() {
            log::error!("{}", e);
        }
        // unsafe {epoxy::Viewport(0, 0, w, h);}
        // let dpi_factor = self.glarea().get_scale_factor();

        let rate_x = f64::from(width) / f64::from(self.state().borrow_mut().physical_size.width);
        let rate_y = f64::from(height) / f64::from(self.state().borrow_mut().physical_size.height);
        let rate = f64::min(rate_x, rate_y);
        let scale_x = (rate / rate_x) as f32;
        let scale_y = (rate / rate_y) as f32;

        // self.context.resize(logical_size.to_physical(dpi_factor));
        // unsafe {epoxy::Viewport(0, 0, w * dpi_factor, h * dpi_factor);}
        if let Some(ref mut view) = self.state().borrow_mut().view {
            view.on_resize(scale_x, scale_y);
        }
    }

    fn render(&self) -> bool {
        render(&self.glarea(), self.state());
        true
    }

    fn unrealize(&self) {
        let state = self.state();
        let mut state = state.borrow_mut();
        if let Some(ref mut view) = state.view {
            view.on_close();
        }
        state.view = None;
    }

    fn tick(&self) -> bool {
        self.glarea().queue_render();
        true
    }
}

fn render(gl_area: &gtk::GLArea, state: Rc<RefCell<State>>) {
    gl_area.make_current();
    if let Some(e) = gl_area.get_error() {
        log::error!("{}", e);
    }
    {
        if let Ok(mut state) = state.try_borrow_mut() {
            let logical_size = state.console.logical_size();
            let ptr = state.console.as_ptr();
            if let Some(ref mut view) = state.view {
                view.on_update(logical_size, ptr);
            }
        }
    }
    unsafe {
        epoxy::Flush();
    }
}
