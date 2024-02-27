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
use glium::uniform;
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
use crate::megaminx::megaminx::Megaminx;
use crate::face::face::FaceFunctions;

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

//Megaminx.rs = WORK IN PROGRESS:
    let mut megaminx: Megaminx = Megaminx::new();
    megaminx.init_reset();
    let faces = megaminx.faces;
    for face in faces {
        let _num = face.getnum();
        //println!("facenum {}", num);
        let centers = face.center;
        for _center in centers {
            //center.init(num);
            //center._vertex[0][0];
        }
    }
//MEGAMINX INIT WORKS FINALLY ^^^^^^ 
    let mut centerpiece: Piece = Piece::new(2);
    centerpiece.centerInit();

    let pentagon = vec![
        Vertex { position: centerpiece.vertex[0] },
        Vertex { position: centerpiece.vertex[1] },
        Vertex { position: centerpiece.vertex[2] }, //tri1
        Vertex { position: centerpiece.vertex[3] },
        Vertex { position: centerpiece.vertex[4] },
        Vertex { position: centerpiece.vertex[0] }, //tri2
        Vertex { position: centerpiece.vertex[0] },
        Vertex { position: centerpiece.vertex[2] },
        Vertex { position: centerpiece.vertex[3] }, //tri3
    ];

    //Glium GL VBO
    let vertex_buffer = glium::VertexBuffer::new(&display, &pentagon).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    //Orthographic Projection Matrix
    let matrix: [[f32; 4]; 4] = [
        [0.01, 0.0, 0.0, 0.0],
        [0.0, 0.01, 0.0, 0.0],
        [0.0, 0.0, 0.01, 0.0],
        [0.0, 0.0, 1.0, 1.0f32]
    ]; //Perspective, Zoom & Camera FOV Matrix
    let perspective: [[f32; 4]; 4] = {
        let aspect_ratio = height as f32 / width as f32;
        let fov: f32 = 3.141592 / 3.0;
        let zfar = 1024.0;
        let znear = 0.1;
        let f = 1.0 / (fov / 2.0).tan();
        [   [f *   aspect_ratio   ,    0.0,              0.0              ,   0.0],
            [         0.0         ,     f ,              0.0              ,   0.0],
            [         0.0         ,    0.0,  (zfar+znear)/(zfar-znear)    ,   1.0],
            [         0.0         ,    0.0, -(2.0*zfar*znear)/(zfar-znear),   0.0],     ]
    }; //COMMENTED OUT TO GET BASIC PENTAGON WORKING
    let vertex_shader_src = r#"
        #version 150
        in vec3 position;
        uniform mat4 perspective;
        uniform mat4 matrix;
        void main() {
            gl_Position =  matrix *  vec4(position, 1.0);
        }        
    "#;
    let fragment_shader_src = r#"
        #version 140
        out vec4 color;
        void main() {
            color = vec4(0.2, 0.6, 0.2, 1.0); //Green
        }
    "#;

    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();
    let params = glium::DrawParameters {
        depth: glium::Depth {
            test: glium::draw_parameters::DepthTest::IfLess,
            write: true,
            .. Default::default()
        },
        .. Default::default()
    };

    let mut target = display.draw();
    //target.clear_color(0.0, 0.0, 1.0, 0.1); // Blue background
    target.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);
    target.draw(&vertex_buffer, &indices, &program, &uniform! { matrix: matrix, perspective: perspective }, &params).unwrap();
    target.finish().unwrap();

    //attempt to draw vertex from pieces using canvas. not correct coordinate space
    let _ = canvas.thick_line(centerpiece.vertex[0][0].round() as i16, centerpiece.vertex[0][1].round() as i16,
                              centerpiece.vertex[2][0].round() as i16, centerpiece.vertex[2][1].round() as i16, 4, Color::RGB(0, 0, 0));

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
