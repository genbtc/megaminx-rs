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

    //Orthographic Projection Matrix
    let projmatrix: [[f32; 4]; 4] = [
        [0.01, 0.0, 0.0, 0.0],
        [0.0, 0.01, 0.0, 0.0],
        [0.0, 0.0, 0.01, 0.0],
        [0.0, 0.0, 1.0, 1.25]
    ];
    //Define GL Shaders
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
    //Glium compile GL shaders
    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();
    //Depth DrawParameters - needed for the backface culling.
    let depthparams: glium::DrawParameters<'_> = glium::DrawParameters {
        depth: glium::Depth {
            test: glium::draw_parameters::DepthTest::IfLessOrEqual,
            write: true,
            .. Default::default()
        },
        .. Default::default()
    };
    //Initialize GL display draw target
    let mut target = display.draw();
    //Set Blue background & Depth Buffer Reset (to 1.0 Z)
    target.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);


    //Megaminx.rs = WORK IN PROGRESS:
    let mut megaminx: Megaminx = Megaminx::new();
    megaminx.init_reset();
    //MEGAMINX INIT WORKS FINALLY ^^^^^^ 

    //Glium GL VBO 2 - CORNER - FILL
    for i in 0..20 {
        let mut cornerbuffer = vec![];
        let mut cornerbuffer_lines = vec![];
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
        cornerbuffer_lines.extend(vec![
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
        //TODO: bundle these ops:
        let vertex_buffer_2 = glium::VertexBuffer::new(&display, &cornerbuffer).unwrap();
        let indices_tri_2 = glium::index::NoIndices(glium::index::PrimitiveType::TriangleFan);
        let color_2: [f32; 4] = [ 0.6, 0.2, 0.1, 1.0 ];
        target.draw(&vertex_buffer_2, &indices_tri_2, &program, &uniform! { projmatrix: projmatrix, colorIn: color_2 }, &depthparams).unwrap();
    
        //Glium GL VBO 2 - CORNER - LINES
        let vertex_buffer_2 = glium::VertexBuffer::new(&display, &cornerbuffer_lines).unwrap();
        let indices_tri_2 = glium::index::NoIndices(glium::index::PrimitiveType::LineLoop);
        let color_2: [f32; 4] = [ 0.0, 0.0, 1.0, 1.0 ];
        target.draw(&vertex_buffer_2, &indices_tri_2, &program, &uniform! { projmatrix: projmatrix, colorIn: color_2 }, &depthparams).unwrap();
    }

    //Glium GL VBO 3 - EDGE
    for i in 0..30 {
        let mut edgebuffer = vec![];
        let mut edge1_lines = vec![];
        let mut edge2_lines = vec![];        
        let mut edgepiece: Piece = Piece::new(i);
        edge::edge::Edge::new(&mut edgepiece);
        edgebuffer.extend(vec![
            VertexPosition { position: edgepiece.vertex[0] },
            VertexPosition { position: edgepiece.vertex[1] },
            VertexPosition { position: edgepiece.vertex[2] }, //tri1
            VertexPosition { position: edgepiece.vertex[3] },
            VertexPosition { position: edgepiece.vertex[0] },
            VertexPosition { position: edgepiece.vertex[2] }, //tri2
            VertexPosition { position: edgepiece.vertex[2] }, 
            VertexPosition { position: edgepiece.vertex[3] },
            VertexPosition { position: edgepiece.vertex[4] }, //tri3
            VertexPosition { position: edgepiece.vertex[5] }, 
            VertexPosition { position: edgepiece.vertex[4] },
            VertexPosition { position: edgepiece.vertex[2] }, //tri4
        ]);
        edge1_lines.extend(vec![
            VertexPosition { position: edgepiece.vertex[0] },
            VertexPosition { position: edgepiece.vertex[1] },
            VertexPosition { position: edgepiece.vertex[2] },
            VertexPosition { position: edgepiece.vertex[3] }, //loop1
        ]);
        edge2_lines.extend(vec![
            VertexPosition { position: edgepiece.vertex[2] },
            VertexPosition { position: edgepiece.vertex[3] },
            VertexPosition { position: edgepiece.vertex[4] },
            VertexPosition { position: edgepiece.vertex[5] }, //loop2
        ]);
        let vertex_buffer_3 = glium::VertexBuffer::new(&display, &edgebuffer).unwrap();
        let indices_tri_3 = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
        let color_3: [f32; 4] = [ 0.8, 0.8, 0.8, 1.0 ]; 
        target.draw(&vertex_buffer_3, &indices_tri_3, &program, &uniform! { projmatrix: projmatrix, colorIn: color_3 }, &depthparams).unwrap();
        let vertex_buffer_3b = glium::VertexBuffer::new(&display, &edge1_lines).unwrap();
        let indices_tri_3b = glium::index::NoIndices(glium::index::PrimitiveType::LineLoop);
        let color_3b: [f32; 4] = [ 0.0, 0.0, 0.0, 1.0 ];
        target.draw(&vertex_buffer_3b, &indices_tri_3b, &program, &uniform! { projmatrix: projmatrix, colorIn: color_3b }, &depthparams).unwrap();
        let vertex_buffer_3c = glium::VertexBuffer::new(&display, &edge2_lines).unwrap();
        let indices_tri_3c = glium::index::NoIndices(glium::index::PrimitiveType::LineLoop);
        target.draw(&vertex_buffer_3c, &indices_tri_3c, &program, &uniform! { projmatrix: projmatrix, colorIn: color_3b }, &depthparams).unwrap();                
    }

    let mut center_pentagon = vec![];
    let mut pentagon_lines = vec![];
    //Can buffer all at once
    for i in 0..12 {
        let mut centerpiece: Piece = Piece::new(i);
        center::center::Center::new(&mut centerpiece);
        center_pentagon.extend(vec![
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
        pentagon_lines.extend(vec![
            VertexPosition { position: centerpiece.vertex[0] },
            VertexPosition { position: centerpiece.vertex[1] },
            VertexPosition { position: centerpiece.vertex[2] },
            VertexPosition { position: centerpiece.vertex[3] },
            VertexPosition { position: centerpiece.vertex[4] }, //loop1
        ]);
    }

    //Glium GL VBO 1
    let vertex_buffer_1 = glium::VertexBuffer::new(&display, &center_pentagon).unwrap();
    let indices_tri_1 = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
    let color_1: [f32; 4] = [ 0.2, 0.6, 0.1, 1.0 ];
    target.draw(&vertex_buffer_1, &indices_tri_1, &program, &uniform! { projmatrix: projmatrix, colorIn: color_1  }, &depthparams).unwrap();
    
    //Glium GL VBO 1 Center - Lines
    let vertex_buffer_1 = glium::VertexBuffer::new(&display, &pentagon_lines).unwrap();
    let indices_tri_1 = glium::index::NoIndices(glium::index::PrimitiveType::LineLoop);
    let color_4: [f32; 4] = [ 0.0, 0.0, 0.0, 1.0 ];
    target.draw(&vertex_buffer_1, &indices_tri_1, &program, &uniform! { projmatrix: projmatrix, colorIn: color_4  }, &depthparams).unwrap();

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
