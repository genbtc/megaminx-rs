// megaminx-rs - a rust and SDL2 version of Megaminx - previously a C++ and OpenGL Dodecahedron Cube
// Author: genr8eofl , Date: 2024 , LICENSE - AGPL3
mod piece;
mod piece_color;
mod piece_static;
//extern crate sdl2;
extern crate gl;
use sdl2::{event::Event, keyboard::Keycode};
use sdl2::pixels::Color;
use sdl2::rect::{Rect, Point};
use sdl2::video::GLProfile;
//extern crate glium;
use glium::implement_vertex;
include!{"../glium_sdl2_lib.rs"}
use crate::glium::Surface;

pub fn main() -> Result<(), String> {

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let mut window = video_subsystem
                .window("Megaminx_SDL2", 640, 640)
                .opengl().resizable().position_centered()
                .build().unwrap();

	let display = video_subsystem.window("Megaminx_SDL2", 640, 640).build_glium().unwrap();

    let mut canvas = window
                .into_canvas()
                .accelerated()
                .build().unwrap();

    //GL Set Core Profile
    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(GLProfile::Core);
    gl_attr.set_context_version(4, 5);

    //GL Context to Pointer
    gl::load_with(|ptr| video_subsystem.gl_get_proc_address(ptr) as *const _);
    let _setcontext = canvas.window().gl_set_context_to_current();
    debug_assert_eq!(gl_attr.context_profile(), GLProfile::Core);
    debug_assert_eq!(gl_attr.context_version(), (4, 5));

    //Gold Background
    canvas.set_draw_color(Color::RGB(255, 210, 0));
    canvas.clear();

    #[derive(Copy, Clone)]
    struct Vertex {
        position: [f32; 2],
    }
    implement_vertex!(Vertex, position);
    let shape = vec![
        Vertex { position: [-0.5, -0.5] },
        Vertex { position: [ 0.0,  0.5] },
        Vertex { position: [ 0.5, -0.25] }
    ];
    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let vertex_shader_src = r#"
        #version 140

        in vec2 position;

        void main() {
            gl_Position = vec4(position, 0.0, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 140

        out vec4 color;

        void main() {
            color = vec4(1.0, 0.0, 0.0, 1.0);
        }
    "#;

    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

    let mut target = display.draw();
    target.clear_color(0.0, 0.0, 1.0, 1.0);
    target.draw(&vertex_buffer, &indices, &program, &glium::uniforms::EmptyUniforms,
        &Default::default()).unwrap();
    target.finish().unwrap();

    //Main Event Loop
    let mut i = 0;
    let mut event_pump = sdl_context.event_pump()?;
    'mainevent: loop {
        //color var cycles
        i = (i + 1) % 255;

        //Color Changing Square
        canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        let _ = canvas.fill_rect(Rect::new(120, 120, 320, 320));
        //Color Changing Square 2
        canvas.set_draw_color(Color::RGB(i, 0, 255 - i));
        let _ = canvas.fill_rect(Rect::new(180, 60, 320, 320));

        //Black Diagonal 2D Line
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        let _ = canvas.draw_line(Point::new(0, 0), Point::new(600, 600));

		use crate::sdl2::gfx::primitives::DrawRenderer;
        //Black Diagonal 2D Thick Line (opposite)
        let _ = canvas.thick_line(556, 0, 0, 556, 4, Color::RGB(0, 0, 0));

        //Black Horizontal Line
        let _ = canvas.hline(0, 600, 280, Color::RGB(0, 0, 0));

        //Green anti-aliased Circle
        let _ = canvas.aa_circle(70, 70, 70, Color::GREEN);

        //Red Filled Triangle
        let _ = canvas.filled_trigon(600, 600, 600, 640, 640, 600, Color::RED);

        canvas.present();

        //Keyboard Event Handler
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown { keycode: Some(Keycode::Escape), .. 
                } => {
                    break 'mainevent
                },
                _ => {}
            }
        }

        ::std::thread::sleep(std::time::Duration::new(0, 1_000_000_000u32 / 60));
    }
    Ok(())
}
