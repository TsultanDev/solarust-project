use std::sync::Arc;

use winit::{application::ApplicationHandler, event::WindowEvent, event_loop::{ControlFlow, EventLoop}, window::{Window, WindowAttributes}};

pub fn entry(){
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);
    let mut app = Application::default();
    _ = event_loop.run_app(&mut app);
}

#[derive(Default)]
struct Application{
    window: Option<Arc<Window>>
}

impl ApplicationHandler for Application {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let window = Arc::new(event_loop.create_window(WindowAttributes::default()).unwrap());
        self.window = Some(window);
    }
    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    )
    {
        match event {
            WindowEvent::CloseRequested =>{
                event_loop.exit();
            }
            WindowEvent::RedrawRequested =>{
                self.window.as_mut().unwrap().request_redraw();
            }
            _ => ()
        }
    }

}
