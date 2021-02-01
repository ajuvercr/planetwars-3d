use frontend::{gl::GL, scene::Scene};
use futures::executor::block_on;

extern crate sdl2;
extern crate futures;

fn main() -> Result<(), String> {
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();

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

    let gl = GL::new();
    let scene = Scene::new();
    let mut scene = block_on(scene.init_renderer(&gl))?;
    scene.camera_handle().set_aspect(900 as f32 / 700 as f32);

    let mut event_pump = sdl.event_pump().unwrap();
    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'main,
                _ => {}
            }
        }

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        scene.update(&gl, 0.16)?;
        scene.render_gl(&gl)?;

        window.gl_swap_window();
    }

    Ok(())
}
