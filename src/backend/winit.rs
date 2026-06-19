use calloop::EventLoop;
use smithay::utils::{Rectangle, Transform};

use crate::state::State;
use smithay::backend::renderer::Color32F;
use smithay::backend::renderer::Frame;
use smithay::backend::renderer::Renderer;
use smithay::backend::renderer::gles::GlesRenderer;
use smithay::backend::winit::{self, WinitGraphicsBackend};

pub fn init_winit(event_loop: &mut EventLoop<State>) {
    let (mut backend, mut winit_evt_loop): (WinitGraphicsBackend<GlesRenderer>, _) =
        winit::init().unwrap();
    event_loop
        .handle()
        .insert_source(winit_evt_loop, move |event, _, _| {
            let size = backend.window_size();
            let damage = Rectangle::from_size(size);
            if let winit::WinitEvent::Redraw = event {
                {
                    let (renderer, mut framebuffer) = backend.bind().unwrap();
                    let mut frame = renderer
                        .render(&mut framebuffer, size, Transform::Flipped180)
                        .unwrap();
                    frame
                        .clear(Color32F::new(0., 0., 0., 1.), &[damage])
                        .unwrap();
                    frame.finish().unwrap();
                }
                backend.submit(Some(&[damage])).unwrap();
            };
        })
        .unwrap();
}
