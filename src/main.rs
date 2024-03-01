#[macro_use]
extern crate glium;
mod tortury;

fn main() {
    let event_loop = winit::event_loop::EventLoopBuilder::new()
        .build()
        .expect("event loop building");
    
    let mut tortury = tortury::OknoWroga::new(image::load(std::io::Cursor::new(&include_bytes!("../tortury.png")),
    image::ImageFormat::Png).unwrap().to_rgba8(), 300, 100,2.0, &event_loop);

    let mut papieskie = tortury::OknoWroga::new(image::load(std::io::Cursor::new(&include_bytes!("../papieskie.png")),
image::ImageFormat::Png).unwrap().to_rgba8(), 300, 100,2.5, &event_loop);

    let mut papaj = tortury::OknoWroga::new(image::load(std::io::Cursor::new(&include_bytes!("../papajak.jpg")),
image::ImageFormat::Jpeg).unwrap().to_rgba8(), 300, 100,2.5, &event_loop);

event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);
tortury.draw();
papieskie.draw();
papaj.draw();
    event_loop.run(move |ev, window_target| {
        match ev {
            winit::event::Event::WindowEvent { event, .. } => match event {
                winit::event::WindowEvent::CloseRequested => {
                    //Easy mode:
                    //window_target.exit();

                    //Also there needs to be a way to run program again if someone kills it
                },
                // We now need to render everyting in response to a RedrawRequested event due to the animation
                winit::event::WindowEvent::RedrawRequested => {
                    tortury.draw();
                    papieskie.draw();
                    papaj.draw();
                },
                // Because glium doesn't know about windows we need to resize the display
                // when the window's size has changed.
                winit::event::WindowEvent::Resized(window_size) => {
                    tortury.display.resize(window_size.into());
                },
                _ => (),
            },
            // By requesting a redraw in response to a AboutToWait event we get continuous rendering.
            // For applications that only change due to user input you could remove this handler.
            winit::event::Event::AboutToWait => {
                tortury.window.request_redraw();
                papieskie.window.request_redraw();
            },
            _ => {
                //window.drag_resize_window(winit::window::ResizeDirection::East);
               tortury.update();
               papieskie.update();
               papaj.update();
            },
        }
    })
    .unwrap();
}