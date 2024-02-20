// megaminx-rs - a rust and SDL2 version of Megaminx - previously a C++ and OpenGL Dodecahedron Cube
// Author: genr8eofl , Date: 2024 , LICENSE - AGPL3
mod piece;
extern crate gl;
use sdl2::{event::Event, keyboard::Keycode};
use sdl2::pixels::Color;
use sdl2::rect::{Rect, Point};
use sdl2::video::WindowBuilder;
use sdl2::render::Canvas;
include!{"../glium_sdl2_lib.rs"}
use glium::Surface;                                                                                                                                                                                                                                                           
use sdl2::gfx::primitives::DrawRenderer;
mod center;
use crate::center::center::Center;
use crate::piece::piece::Piece;
use crate::piece::piece::PieceMath;
use crate::piece::piece::PieceData;
use crate::piece::piece::Vertex;

pub fn main() -> Result<(), String> {
	//SDL2 + Glium setup combined
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem: VideoSubsystem = sdl_context.video().unwrap();
    let mut binding: WindowBuilder = video_subsystem.window("Megaminx_SDL2", 640, 640);
	let  display: SDL2Facade = binding.build_glium().unwrap();
    let  window_b: Window = unsafe { Window::from_ref(display.window().context()) };
    let mut canvas: Canvas<Window> = window_b
	                .into_canvas()
	                .accelerated()
	                .build().unwrap();

//WORK IN PROGRESS:
	let vertexdata: [[f32; 3]; 7] = [ [0.,0.,0.], [0.,0.,0.], [0.,0.,0.], [0.,0.,0.], [0.,0.,0.], [0.,0.,0.], [0.,0.,0.] ];
//	 and other private fields `_color`, `_colorNum`, `_colorName`, `pieceNum`, `flipStatus` and `hotPieceMoving` that were not provided
	let piecedata: PieceData = PieceData { _color: [[0.5,0.5,0.5],[0.5,0.5,0.5],[0.5,0.5,0.5]] , _colorNum: [1,1,1] , _colorName: ["WHITE","WHITE","WHITE"] , pieceNum: 1, flipStatus: 0, hotPieceMoving: false };
//	= note: ... and other private fields `_vertex`, `defaultPieceNum`, `numSides` and `data` that were not provided
	let mut centerpiece: Piece = Piece { _vertex: vertexdata, defaultPieceNum: 1, numSides: 1, data: piecedata  };  // dyn Center = { }; //center::center::Center` cannot be made into an object
	centerpiece.init(1);
	centerpiece.centerInit();
//NOT YET DONE!

	let shape = vec![
		Vertex { position: [ -0.4, -0.9, 0.0 ] },
		Vertex { position: [ 0.0,  0.9, 0.0 ] },
		Vertex { position: [ 0.8, -0.5, 0.0 ] }
	];
    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

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
            color = vec4(0.2, 0.6, 0.2, 0.4);
        }
    "#;

    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

    let mut target = display.draw();
    target.clear_color(0.0, 0.0, 1.0, 0.1);	// Blue background
    target.draw(&vertex_buffer, &indices, &program, &glium::uniforms::EmptyUniforms, &Default::default()).unwrap();
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
