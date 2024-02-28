// megaminx-rs - a rust and SDL2 version of Megaminx - previously a C++ and OpenGL Dodecahedron Cube
// Author: genr8eofl , Date: 2024 , LICENSE - AGPL3
extern crate gl;
use sdl2::{event::Event, keyboard::Keycode};
use sdl2::pixels::Color;
use sdl2::rect::Rect;//Point};
use sdl2::video::WindowBuilder;
use sdl2::render::Canvas;
include!{"../glium_sdl2_lib.rs"}
use glium::Surface;
use glium::uniform;
//use sdl2::gfx::primitives::DrawRenderer;
mod megaminx;
mod center;
mod edge;
mod corner;
mod face;
mod piece;
mod piece_color;
use crate::piece::piece::Piece;
use crate::piece::piece::VertexPosition;
use crate::piece::piece::Vertex3;
use crate::megaminx::megaminx::Megaminx;

pub fn main() -> Result<(), String> {
    //SDL2 + Glium setup (combined)
    let (width,height) = (640,640);
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem: VideoSubsystem = sdl_context.video().unwrap();
    let mut binding: WindowBuilder = video_subsystem.window("Megaminx_SDL2", width, height);
    let  display: SDL2Facade = binding.build_glium().unwrap();
    let  window_b: Window = unsafe { Window::from_ref(display.window().context()) };
    let mut canvas: Canvas<Window> = window_b
                    .into_canvas()
                    .accelerated()
                    .build().unwrap();
    /*/Canvas Draw Examples: 
    //Black Diagonal 2D Line
    let _ = canvas.draw_line(Point::new(0, 0), Point::new(600, 600));
    //Black Diagonal 2D Thick Line (opposite)
    let _ = canvas.thick_line(556, 0, 0, 556, 4, Color::RGB(0, 0, 0));
    //Black Horizontal Line
    let _ = canvas.hline(0, 600, 280, Color::RGB(0, 0, 0));
    //Green anti-aliased Circle
    let _ = canvas.aa_circle(70, 70, 70, Color::GREEN);
    //Red Filled Triangle
    let _ = canvas.filled_trigon(600, 600, 600, 640, 640, 600, Color::RED); */

//Megaminx.rs = WORK IN PROGRESS:
    let mut megaminx: Megaminx = Megaminx::new();
    megaminx.init_reset();
//MEGAMINX INIT WORKS FINALLY ^^^^^^ 
    let mut pentagon = vec![];
    for i in 0..12 {
        let mut centerpiece: Piece = Piece::new(i);
        center::center::Center::new(&mut centerpiece);
        pentagon.extend(vec![
            VertexPosition { position: centerpiece.vertex[0] },
            VertexPosition { position: centerpiece.vertex[1] },
            VertexPosition { position: centerpiece.vertex[2] }, //tri1
            VertexPosition { position: centerpiece.vertex[0] },
            VertexPosition { position: centerpiece.vertex[2] },
            VertexPosition { position: centerpiece.vertex[3] }, //tri2
            VertexPosition { position: centerpiece.vertex[0] },
            VertexPosition { position: centerpiece.vertex[3] },
            VertexPosition { position: centerpiece.vertex[4] }, //tri3
        ]);
    }

    //Orthographic Projection Matrix
    let projmatrix: [[f32; 4]; 4] = [
        [0.01, 0.0, 0.0, 0.0],
        [0.0, 0.01, 0.0, 0.0],
        [0.0, 0.0, 0.01, 0.0],
        [0.0, 0.0, 1.0, 1.25]
    ];
    let vertex_shader_src = r#"
        #version 140
        in vec3 position;
        uniform mat4 projmatrix;
        void main() {
            gl_Position = projmatrix * vec4(position, 1.0);
        }
    "#;
    let fragment_shader_src = r#"
        #version 140
        uniform vec4 colorIn;
        out vec4 color;
        void main() {
            color = vec4(colorIn);
        }
    "#;
    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();
/*    let params = glium::DrawParameters {
        depth: glium::Depth {
            test: glium::draw_parameters::DepthTest::IfLessOrEqual,
            write: true,
            .. Default::default()
        },
        .. Default::default()
    }; COMMENTED OUT - NOT ACTUALLY NEEDED YET */
    let mut target = display.draw();
    target.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);    // Blue background & Depth Buffer Reset (to 1.0 Z)

    //Glium GL VBO 1
    let vertex_buffer_1 = glium::VertexBuffer::new(&display, &pentagon).unwrap();
    let indices_tri_1 = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
    let color_1: [f32; 4] = [ 0.2, 0.6, 0.1, 1.0 ];
    target.draw(&vertex_buffer_1, &indices_tri_1, &program, &uniform! { projmatrix: projmatrix, colorIn: color_1  }, &Default::default()).unwrap();

    //Glium GL VBO 2
    for i in 0..20 {
        let mut cornerbuffer = vec![];
        let mut cornerpiece: Piece = Piece::new(i);
        corner::corner::Corner::new(&mut cornerpiece);
        cornerbuffer.extend(vec![
            VertexPosition { position: cornerpiece.vertex[0] },
            VertexPosition { position: cornerpiece.vertex[1] },
            VertexPosition { position: cornerpiece.vertex[2] },
            VertexPosition { position: cornerpiece.vertex[3] }, //loop1
            VertexPosition { position: cornerpiece.vertex[2] },
            VertexPosition { position: cornerpiece.vertex[3] },
            VertexPosition { position: cornerpiece.vertex[4] }, 
            VertexPosition { position: cornerpiece.vertex[5] }, //Loop2
            VertexPosition { position: cornerpiece.vertex[2] },
            VertexPosition { position: cornerpiece.vertex[5] },
            VertexPosition { position: cornerpiece.vertex[6] },
            VertexPosition { position: cornerpiece.vertex[1] }, //loop3
        ]);
        let vertex_buffer_2 = glium::VertexBuffer::new(&display, &cornerbuffer).unwrap();
        let indices_tri_2 = glium::index::NoIndices(glium::index::PrimitiveType::LineLoop);
        let color_2: [f32; 4] = [ 0.6, 0.2, 0.1, 1.0 ];
        target.draw(&vertex_buffer_2, &indices_tri_2, &program, &uniform! { projmatrix: projmatrix, colorIn: color_2 }, &Default::default()).unwrap();
    }    

    //Glium GL VBO 3
    let mut edgebuffer = vec![];
    for i in 0..30 {
        let mut edgepiece: Piece = Piece::new(i);
        edge::edge::Edge::new(&mut edgepiece);
        edgebuffer.extend(vec![
            VertexPosition { position: edgepiece.vertex[0] },
            VertexPosition { position: edgepiece.vertex[1] },
            VertexPosition { position: edgepiece.vertex[2] }, //Tri0
            VertexPosition { position: edgepiece.vertex[3] },
            VertexPosition { position: edgepiece.vertex[0] },
            VertexPosition { position: edgepiece.vertex[1] }, //Tri0            
            
            VertexPosition { position: edgepiece.vertex[2] },
            VertexPosition { position: edgepiece.vertex[3] },
            VertexPosition { position: edgepiece.vertex[4] }, //Tri1
            VertexPosition { position: edgepiece.vertex[5] }, 
            VertexPosition { position: edgepiece.vertex[2] }, 
            VertexPosition { position: edgepiece.vertex[3] }, //Tri3
        ]);
        let vertex_buffer_3 = glium::VertexBuffer::new(&display, &edgebuffer).unwrap();
        let indices_tri_3 = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
        let color_3: [f32; 4] = [ 0.8, 0.8, 0.8, 1.0 ]; 
        target.draw(&vertex_buffer_3, &indices_tri_3, &program, &uniform! { projmatrix: projmatrix, colorIn: color_3 }, &Default::default()).unwrap();
    }   
    
    //Glium end GL
    target.finish().unwrap();

    //Main Event Loop
    let mut i: u8 = 0;
    let mut event_pump = sdl_context.event_pump()?;
    'mainevent: loop {
        //color var cycles
        i = (i + 1) % 255;
        //Color Changing Square
        canvas.set_draw_color(Color::RGB(i, 0, 255 - i));
        let _ = canvas.fill_rect(Rect::new(0, 0, 20, 20));

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
