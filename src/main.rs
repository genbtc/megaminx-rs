// megaminx-rs - a rust and SDL2 version of Megaminx - previously a C++ and OpenGL Dodecahedron Cube
// Author: genr8eofl , Date: 2024 , LICENSE - AGPL3
extern crate gl;
use sdl2::{event::Event, keyboard::Keycode};
use sdl2::pixels::Color;
use sdl2::rect::{Rect, Point};
use sdl2::video::WindowBuilder;
use sdl2::render::Canvas;
include!{"../glium_sdl2_lib.rs"}
use glium::Surface;
use sdl2::gfx::primitives::DrawRenderer;
mod megaminx;
mod center;
mod edge;
mod corner;
mod face;
mod piece;
mod piece_color;
use crate::piece::piece::Piece;
use crate::piece::piece::PieceMath;
use crate::piece::piece::Vertex;
use crate::piece::piece::Vertex3;

pub fn main() -> Result<(), String> {
    //SDL2 + Glium setup (combined)
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem: VideoSubsystem = sdl_context.video().unwrap();
    let mut binding: WindowBuilder = video_subsystem.window("Megaminx_SDL2", 640, 640);
    let  display: SDL2Facade = binding.build_glium().unwrap();
    let  window_b: Window = unsafe { Window::from_ref(display.window().context()) };
    let mut canvas: Canvas<Window> = window_b
                    .into_canvas()
                    .accelerated()
                    .build().unwrap();
    //Canvas Draw Examples:
    //Black Diagonal 2D Line
    let _ = canvas.draw_line(Point::new(0, 0), Point::new(600, 600));
    //Black Diagonal 2D Thick Line (opposite)
    let _ = canvas.thick_line(556, 0, 0, 556, 4, Color::RGB(0, 0, 0));
    //Black Horizontal Line
    let _ = canvas.hline(0, 600, 280, Color::RGB(0, 0, 0));
    //Green anti-aliased Circle
    let _ = canvas.aa_circle(70, 70, 70, Color::GREEN);
    //Red Filled Triangle
    let _ = canvas.filled_trigon(600, 600, 600, 640, 640, 600, Color::RED);

//WORK IN PROGRESS:
   let mut centerpiece: Piece = Piece::new(1);
    centerpiece.centerInit();

for i in 0..5 {
    for j in 0..2 {
//      centerpiece._vertex[i][j] *= 0.01; scale //reset scale from 100 to 1
      centerpiece._vertex[i][j] = centerpiece._vertex[i][j].abs();  //Clamp to >0 
    }
    centerpiece._vertex[i][2] = 0.0;    
  }    
    print!("Center Piece 1 Vertex Array: [ ");
    for i in 0..5 {
      print!("[ ");
      for j in 0..3 {
        print!("{}", centerpiece._vertex[i][j].to_string());
        if j < centerpiece._vertex[i].len() - 1  { print!(", "); }
      }
      if i < centerpiece._vertex.len() - 1  { print!(" ], "); }
    }
    println!("]");

//NOT YET DONE!

    let _shape = vec![
        Vertex { position: [ -0.4, -0.9, 0.0 ] },
        Vertex { position: [ 0.0,  0.9, 0.0 ] },
        Vertex { position: [ 0.8, -0.5, 0.0 ] }
    ];
    let _pentagon = vec![
        Vertex { position: centerpiece._vertex[0] },
        Vertex { position: centerpiece._vertex[1] },
        Vertex { position: centerpiece._vertex[2] },
        Vertex { position: centerpiece._vertex[3] },
        Vertex { position: centerpiece._vertex[4] },
        Vertex { position: centerpiece._vertex[0] }
    ];
    //Glium GL VBO
    let vertex_buffer = glium::VertexBuffer::new(&display, &_pentagon).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);  //LineLoop isnt Fill'ed

    let vertex_shader_src = r#"
        #version 140
        in vec3 position;
        void main() {
            gl_Position = vec4(position, 1.0);
        }
    "#;
    let fragment_shader_src = r#"
        #version 140
        out vec4 color;
        void main() {
            color = vec4(0.2, 0.6, 0.2, 0.4); //Green
        }
    "#;

    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

    let mut target = display.draw();
    target.clear_color(0.0, 0.0, 1.0, 0.1); // Blue background
    target.draw(&vertex_buffer, &indices, &program, &glium::uniforms::EmptyUniforms, &Default::default()).unwrap();
    target.finish().unwrap();

    //attempt to draw vertex from pieces using canvas. not correct coordinate space
    let _ = canvas.thick_line(centerpiece._vertex[0][0].round() as i16, centerpiece._vertex[0][1].round() as i16,
                              centerpiece._vertex[2][0].round() as i16, centerpiece._vertex[2][1].round() as i16, 4, Color::RGB(0, 0, 0));

    //Main Event Loop
    let mut i = 0;
    let mut event_pump = sdl_context.event_pump()?;
    'mainevent: loop {
        //color var cycles
        i = (i + 1) % 255;
        //Color Changing Square
        canvas.set_draw_color(Color::RGB(i, 0, 255 - i));
        let _ = canvas.fill_rect(Rect::new(36, 36, 80, 80));

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
