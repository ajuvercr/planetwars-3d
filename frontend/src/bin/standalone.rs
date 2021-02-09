use frontend::{engine::CameraHandle, gl::GL, scene::Scene};
use futures::executor::block_on;
use sdl2::{event::WindowEvent, video::{SwapInterval, Window}};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

extern crate futures;
extern crate sdl2;

const MOV_SPEED:f32 = 1000.0;
const MOUSE_SENTSITIVITY:f32 = 50.0;

#[derive(Debug, Default)]
struct InputStatus {
    forwards:  bool,
    backwards: bool,
    left: bool,
    right: bool,
    up: bool,
    down: bool,
}

impl InputStatus {
    fn update_camera(&self, dt: f32, handle: &CameraHandle) {
        let mut forward_backward = 0.0;
        let mut left_right = 0.0;
        let mut up_down = 0.0;

        if self.right { left_right += MOV_SPEED * dt};
        if self.left { left_right  -= MOV_SPEED * dt};

        if self.up { up_down += MOV_SPEED * dt };
        if self.down { up_down -= MOV_SPEED * dt };

        if self.forwards { forward_backward -= MOV_SPEED * dt };
        if self.backwards { forward_backward += MOV_SPEED * dt };

        handle.add_position(
            left_right,
            up_down,
            forward_backward,
        );
    }
}

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

    let mut input_status = InputStatus::default();

    let gl = GL::new();
    let scene = Scene::new();
    let mut scene = block_on(scene.init_renderer(&gl))?;
    let camera_handle = scene.camera_handle();
    camera_handle.add_position(-150.0, 0.0, 0.0);

    let mut event_pump = sdl.event_pump().unwrap();
    let mut last = timer_subsystem.performance_counter();

    let mut window_size= window.size(); // (width, height)


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
                            window_size = window.size();
                            resize(&window, &camera_handle);

                            // camera_handle.set_aspect(width as f32 / height as f32);
                            // unsafe {gl::Viewport(0, 0, width, height); }
                        }
                        _ => {}
                    }
                },
                Event::KeyDown { keycode: Some(code), ..  } =>{
                    match code {
                        Keycode::W => input_status.forwards = true,
                        Keycode::S => input_status.backwards = true,
                        Keycode::A => input_status.left = true,
                        Keycode::D => input_status.right = true,
                        _ => {}
                    }
                },
                Event::KeyUp { keycode: Some(code), .. } => {
                    match code {
                        Keycode::W => input_status.forwards = false,
                        Keycode::S => input_status.backwards = false,
                        Keycode::A => input_status.left = false,
                        Keycode::D => input_status.right = false,
                        _ => {}
                    }
                }
                Event::MouseMotion{ mousestate, xrel, yrel, .. } => {
                    if mousestate.left() {
                        let dx = xrel as f32 / window_size.0 as f32 * MOUSE_SENTSITIVITY;
                        let dy = yrel as f32 / window_size.1 as f32 * MOUSE_SENTSITIVITY;
                        camera_handle.add_angle(dy, 0.0, dx);
                    }
                }
                _ => {}
            }
        }

        input_status.update_camera(delta as f32, &camera_handle);

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        scene.update(&gl, delta)?;
        scene.render_gl(&gl)?;

        window.gl_swap_window();
    }

    Ok(())
}
