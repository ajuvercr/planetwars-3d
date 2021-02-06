use frontend::{engine::CameraHandle, gl::GL, scene::Scene};
use futures::executor::block_on;
use sdl2::{event::WindowEvent, video::{SwapInterval, Window}};
use sdl2::event::Event;
extern crate futures;
extern crate sdl2;

fn resize(window: &Window, camera_handle: &CameraHandle) {
    let (w, h) = window.size();
    let aspect = w as f32 / h as f32;
    camera_handle.set_aspect(aspect);
    unsafe {gl::Viewport(0,0, w as i32, h as i32); }
}

fn main() -> Result<(), String> {
    let sdl = sdl2::init()?;
    let video_subsystem = sdl.video()?;
    let timer_subsystem = sdl.timer()?;

    let gl_attr = video_subsystem.gl_attr();

    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 1);

    let window = video_subsystem
        .window("Game", 900, 700)
        .opengl()
        .resizable()
        .build()
        .unwrap();

    let _gl_context = window.gl_create_context().unwrap();
    let _gl =
        gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

    unsafe {
        gl::Viewport(0, 0, 900, 700);
        gl::ClearColor(0.3, 0.3, 0.5, 1.0);
        gl::Enable(gl::CULL_FACE);
        gl::Enable(gl::DEPTH_TEST);
    }
    // video_subsystem.gl_set_swap_interval(SwapInterval::Immediate)?;

    let gl = GL::new();
    let scene = Scene::new();
    let mut scene = block_on(scene.init_renderer(&gl))?;
    let camera_handle = scene.camera_handle();
    camera_handle.add_position(-150.0, 0.0, 0.0);

    let mut event_pump = sdl.event_pump().unwrap();
    let mut last = timer_subsystem.performance_counter();

    'main: loop {
        let delta = {
            let current = timer_subsystem.performance_counter();
            let delta = current - last;
            last = current;

            delta as f64 / 1000000.0
        };

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } | Event::AppTerminating {..} => break 'main,
                Event::Window {win_event, ..} => {
                    match win_event {
                        WindowEvent::Resized(width, height) => {
                            resize(&window, &camera_handle);

                            // camera_handle.set_aspect(width as f32 / height as f32);
                            // unsafe {gl::Viewport(0, 0, width, height); }
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        scene.update(&gl, delta)?;
        scene.render_gl(&gl)?;

        window.gl_swap_window();
    }

    Ok(())
}
