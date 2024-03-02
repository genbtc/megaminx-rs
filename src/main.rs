// megaminx-rs - a rust and SDL2 version of Megaminx - previously a C++ and OpenGL Dodecahedron Cube
// Author: genr8eofl , Date: 2024 , LICENSE - AGPL3
extern crate gl;
use sdl2::{event::Event, keyboard::Keycode};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::video::WindowBuilder;
use sdl2::render::Canvas;
include!{"../glium_sdl2_lib.rs"}
use glium::Surface;
use glium::uniform;
mod megaminx;
mod center;
mod edge;
mod corner;
mod face;
mod piece;
mod piece_color;
use megaminx::megaminx::Corner;
use piece::piece::EdgeCornerMath;

pub fn main() -> Result<(), String> {
    //SDL2 + Glium setup (combined)
    let (width,height) = (640,640);
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem: VideoSubsystem = sdl_context.video().unwrap();
    let mut binding: WindowBuilder = video_subsystem.window("Megaminx_SDL2", width, height);
    let     display: SDL2Facade = binding.build_glium().unwrap();
    let window_b: Window = unsafe { Window::from_ref(display.window().context()) };
    let mut canvas: Canvas<Window> = window_b.into_canvas().accelerated().build().unwrap();

    let mut translate_x:f32=0.0;
    let mut translate_y:f32=0.0;
    let mut translate_z:f32=0.0;
    let mut zoom:      f32=1.25;
    //Depth DrawParameters - needed for the backface culling.
    let depthparams: glium::DrawParameters<'_> = glium::DrawParameters {
        depth: glium::Depth {
            test: glium::draw_parameters::DepthTest::IfLessOrEqual,
            write: true,
            .. Default::default()
        },
        .. Default::default()
    };

    //Define GL Shaders for Color Input
    let vertex_shader_src_color = r#"
        #version 140
        in vec3 position, color;
        out vec3 vertex_color;
        uniform mat4 projmatrix;
        void main() {
            vertex_color = color;
            gl_Position = projmatrix * vec4(position, 1.0);
        }
    "#;
    let fragment_shader_src_color = r#"
        #version 140
        in vec3 vertex_color;
        out vec4 color;
        void main() {
            color = vec4(vertex_color, 1.0);
        }
    "#;
    let indices_triangles = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
    let indices_lineloop = glium::index::NoIndices(glium::index::PrimitiveType::LineLoop);

    //Megaminx.rs = WORK IN PROGRESS:
    let mut megaminx = megaminx::megaminx::Megaminx::new();
    megaminx.init_reset();

    //Initialize GL display draw target
    let mut target = display.draw();
    //Set Blue background & Depth Buffer Reset (to 1.0 Z)
    target.clear_color_and_depth((0.0, 0.0, 0.5, 0.5), 1.0);


    //Main Event Loop
    let mut i: u8 = 0;
    let mut event_pump = sdl_context.event_pump()?;
    'mainevent: loop {
        //Color var cycles
        i = (i + 1) % 255;
        //Color Changing Square
        canvas.set_draw_color(Color::RGB(i, 0, 255 - i));
        let _ = canvas.fill_rect(Rect::new(0, 0, 25, 25));
        canvas.present();

        //Orthographic Projection Matrix
        let projmatrix: [[f32; 4]; 4] = [
            [0.01, 0.0, 0.0, translate_x],
            [0.0, 0.01, 0.0, translate_y],
            [0.0, 0.0, 0.01, translate_z],
            [0.0, 0.0, 1.0, zoom]
        ];

        //Glium compile GL shaders - Color,
        let program_color = glium::Program::from_source(&display, vertex_shader_src_color, fragment_shader_src_color, None).unwrap();

        //CORNERS render
        for i in 0..20 {
            //Glium GL VBO 3 - CORNER - FILL
            let vertex_buffer_3 = glium::VertexBuffer::new(&display, &Corner::render(&*megaminx.corners[i])).unwrap();
            target.draw(&vertex_buffer_3, &indices_triangles, &program_color, &uniform! { projmatrix: projmatrix }, &depthparams).unwrap();
            //Glium GL VBO 3 - CORNER - LINES
            let vertex_buffer_3 = glium::VertexBuffer::new(&display, &Corner::render_lines(&*megaminx.corners[i])).unwrap();
            target.draw(&vertex_buffer_3, &indices_lineloop, &program_color, &uniform! { projmatrix: projmatrix }, &depthparams).unwrap();
        }

        //EDGES render
        for i in 0..30 {
            //Glium GL VBO 2 - EDGE - FILL
            let vertex_buffer_2 = glium::VertexBuffer::new(&display, &EdgeCornerMath::render(&*megaminx.edges[i])).unwrap();
            target.draw(&vertex_buffer_2, &indices_triangles, &program_color, &uniform! { projmatrix: projmatrix }, &depthparams).unwrap();
            //Glium GL VBO 2 - EDGE - LINES
            let vertex_buffer_2b = glium::VertexBuffer::new(&display, &EdgeCornerMath::render_lines(&*megaminx.edges[i], 0)).unwrap();
            let vertex_buffer_2c = glium::VertexBuffer::new(&display, &EdgeCornerMath::render_lines(&*megaminx.edges[i], 1)).unwrap();        
            target.draw(&vertex_buffer_2b, &indices_lineloop, &program_color, &uniform! { projmatrix: projmatrix }, &depthparams).unwrap();
            target.draw(&vertex_buffer_2c, &indices_lineloop, &program_color, &uniform! { projmatrix: projmatrix }, &depthparams).unwrap();                
        }

        //CENTERS render
        for i in 0..12 {
            //Glium GL VBO 1 - CENTER - FILL
            let vertex_buffer_1 = glium::VertexBuffer::new(&display, &megaminx.centers[i].render()).unwrap();
            target.draw(&vertex_buffer_1, &indices_triangles, &program_color, &uniform! { projmatrix: projmatrix }, &depthparams).unwrap();
            //Glium GL VBO 1 - CENTER - LINES
            let vertex_buffer_1 = glium::VertexBuffer::new(&display, &megaminx.centers[i].render_lines()).unwrap();
            target.draw(&vertex_buffer_1, &indices_lineloop, &program_color, &uniform! { projmatrix: projmatrix }, &depthparams).unwrap();        
        }
        // target.finish().unwrap();
        // |         ---------- move occurs because `target` has type `Frame`, which does not implement the `Copy` trait
        //      |                 ---- inside of this loop
        //      |         ^^^^^^ -------- `target` moved due to this method call, in previous iteration of loop
        // note: `Frame::finish` takes ownership of the receiver `self`, which moves `target`

        //WARNING: The `Frame` object must be explicitly destroyed by calling `.finish()`

        //Keyboard Event Handler
        for event in event_pump.poll_iter() {
            match event {
                // Event::KeyDown { keycode: Some(Keycode::F1), .. 
                // } => {  megaminx.faces[0].place_parts();
                //     = help: items from traits can only be used if the trait is implemented and in scope
                //     note: `FacePlaceFunctions` defines an item `place_parts`, perhaps you need to implement it                    
                // }
                Event::KeyDown { keycode: Some(Keycode::F1), .. 
                } => { zoom+=0.1; }
                Event::KeyDown { keycode: Some(Keycode::F2), .. 
                } => { translate_x+=0.001; }
                Event::KeyDown { keycode: Some(Keycode::F3), .. 
                } => { translate_y+=0.001; }
                Event::KeyDown { keycode: Some(Keycode::F4), .. 
                } => { translate_z+=0.001; }                
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
