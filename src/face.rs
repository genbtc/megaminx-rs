//2024 megaminx-rs face.rs , by genr8eofl - LICENSED APGL3
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
pub mod face {
  use crate::piece::piece::Piece;
  use crate::Vertex3; 
  use crate::center::center::Center;
  use crate::PieceData;
  //Face Data
  pub struct Face {
    this_num: i8,
    turn_dir: i8,
    rotating: bool,
    angle: f32,
    axis: [f32;3],
    do_axes: bool,
    //Duplicated from Piece Struct since no longer a Piece
    default_piece_num: i8,
    data: PieceData,
    center: dyn Center
    //TODO: hold a pointer back to the parent megaminx
    //Megaminx *megaminx;
  }
  //From center.rs already;
/*pub trait Center {
    fn init(&mut self, piecenum: i8);
    fn create_axis(&mut self, piecenum: i32, index: usize);
    fn render(&self);
  } */
  impl Center for Face {
    /**
     * \brief Inits a Face piece based on Center
     * \note  (calls createAxis and initColor)
     * \param n the number of the Face piece (piecenum)
     * \param doAxes True by default. First Time Initialization Only
     */
    fn init(&mut self, piecenum: i8) {
        if self.do_axes {
            for i in 0..5  {
                self.create_axis(piecenum as i32, i);
            }
        }
        //TODO: initColor(G_FACEPIECESCOLORS[piecenum], true);
        self.data.pieceNum = piecenum;
        self.default_piece_num = piecenum;
    }
    fn create_axis(&mut self, piecenum: i32, index: usize) {
        self.init(piecenum as i8);
    }
    fn render(&self) {
        todo!();
    }
  }
  trait FaceFunctions { 
    fn getnum(&self) -> i8;
    fn attach_center(&mut self);    //(Center* c, double* centerVertexBase);
    fn attach_corner_pieces(&self); //(const Megaminx* megaminx, Corner& cornersPTR);
    fn attach_edge_pieces(&self);   //(const Megaminx* megaminx, Edge& edgesPTR);
  }
  impl FaceFunctions for Face { 
    fn getnum(&self) -> i8 { 
        return self.this_num;
    }
    fn attach_center(&mut self){
        //self.center = c;
    }
    fn attach_corner_pieces(&self) {
        todo!();
    }
    fn attach_edge_pieces(&self) {
        todo!();
    }
  }

enum TurnDir { Clockwise = -1, None = 0, CounterClockwise = 1 }
enum TurnDir2 { CW = -1, CCW = 1 }

//Named Flip Direction lists:
static FlipInwards: [i8;4] =     [ 0, 1, 1, 0 ];
static FlipOutwards: [i8;4] =    [ 1, 0, 0, 1 ];
static FlipBackwards: [i8;4] =   [ 0, 0, 1, 1 ];
static FlipForwards: [i8;4]   =  [ 1, 1, 0, 0 ];
static FlipBackwardAlt: [i8;4] = [ 0, 1, 0, 1 ];
static FlipForwardAlt: [i8;4]  = [ 1, 0, 1, 0 ];

//These are invoked when Face::placeParts() is ran, when it's rotating.
//Called from Face::render(), only when something is moved, NEVER on startup.
//Flip direction lists for PlaceParts: //CounterClockwise CORNERS
//CCW Corners
static  CCW0C: [i8;8] = [ 0, 1, 1, 2, 2, 3, 3, 4 ];
static  CCW1C: [i8;8] = [ 0, 2, 0, 4, 0, 3, 0, 1 ];
static  CCW2C: [i8;8] = [ 0, 1, 0, 2, 2, 3, 2, 4 ];
static  CCW3C: [i8;8] = [ 3, 4, 1, 3, 1, 2, 0, 1 ];
static  CCW4C: [i8;8] = [ 0, 1, 0, 3, 0, 4, 0, 2 ];
static  CCW5C: [i8;8] = [ 1, 3, 1, 4, 1, 2, 0, 1 ];
static  CCW6C: [i8;8] = [ 0, 1, 4, 0, 3, 4, 2, 3 ];
static  CCW7C: [i8;8] = [ 1, 3, 3, 4, 4, 2, 2, 0 ];
static  CCW8C: [i8;8] = [ 4, 3, 4, 2, 4, 0, 4, 1 ];
static  CCW9C: [i8;8] = [ 4, 3, 4, 2, 4, 0, 4, 1 ];
static CCW10C: [i8;8] = [ 4, 3, 4, 2, 4, 0, 4, 1 ];
static CCW11C: [i8;8] = [ 4, 2, 4, 3, 3, 1, 1, 0 ];
//Flip direction lists for PlaceParts: //Clockwise CORNERS
//CW Corners
static  CW0C: [i8;8] = [ 0, 1, 4, 0, 3, 4, 2, 3 ];
static  CW1C: [i8;8] = [ 0, 1, 0, 3, 0, 4, 0, 2 ];
static  CW2C: [i8;8] = [ 0, 1, 1, 2, 1, 3, 3, 4 ];
static  CW3C: [i8;8] = [ 0, 1, 1, 2, 1, 3, 3, 4 ];
static  CW4C: [i8;8] = [ 0, 2, 0, 4, 0, 3, 0, 1 ];
static  CW5C: [i8;8] = [ 0, 1, 1, 2, 1, 4, 1, 3 ];
static  CW6C: [i8;8] = [ 0, 1, 1, 2, 2, 3, 3, 4 ];
static  CW7C: [i8;8] = [ 2, 0, 4, 2, 3, 4, 1, 3 ];
static  CW8C: [i8;8] = [ 4, 1, 4, 0, 4, 2, 4, 3 ];
static  CW9C: [i8;8] = [ 4, 1, 4, 0, 4, 2, 4, 3 ];
static CW10C: [i8;8] = [ 4, 1, 4, 0, 4, 2, 4, 3 ];
static CW11C: [i8;8] = [ 1, 0, 3, 1, 4, 3, 2, 4 ];
//Flip direction lists for PlaceParts: //CounterClockwise Edges
//CCW Edges
static  CCW0E: [i8;8] = [ 0, 1, 1, 2, 2, 3, 3, 4 ];
static  CCW1E: [i8;8] = [ 4, 1, 1, 3, 0, 1, 0, 2 ];
static  CCW2E: [i8;8] = [ 1, 0, 1, 2, 1, 3, 3, 4 ];
static  CCW3E: [i8;8] = [ 3, 2, 4, 3, 0, 1, 1, 2 ];
static  CCW4E: [i8;8] = [ 0, 1, 1, 2, 1, 3, 3, 4 ];
static  CCW5E: [i8;8] = [ 2, 4, 2, 3, 0, 2, 0, 1 ];
static  CCW6E: [i8;8] = [ 0, 1, 4, 0, 3, 4, 2, 3 ];
static  CCW7E: [i8;8] = [ 0, 3, 0, 4, 0, 2, 0, 1 ];
static  CCW8E: [i8;8] = [ 0, 1, 1, 2, 2, 4, 3, 4 ];
static  CCW9E: [i8;8] = [ 0, 1, 1, 2, 2, 4, 3, 4 ];
static CCW10E: [i8;8] = [ 0, 2, 0, 4, 0, 3, 0, 1 ];
static CCW11E: [i8;8] = [ 0, 3, 0, 4, 0, 2, 0, 1 ];
//Flip direction lists for PlaceParts: //Clockwise Edges
//CW Edges
static  CW0E: [i8;8] = [ 0, 1, 4, 0, 3, 4, 2, 3 ];
static  CW1E: [i8;8] = [ 0, 2, 0, 1, 1, 3, 4, 1 ];
static  CW2E: [i8;8] = [ 3, 4, 1, 3, 1, 2, 1, 0 ];
static  CW3E: [i8;8] = [ 1, 2, 0, 1, 4, 3, 3, 2 ];
static  CW4E: [i8;8] = [ 3, 4, 1, 3, 1, 2, 0, 1 ];
static  CW5E: [i8;8] = [ 0, 1, 0, 2, 2, 3, 2, 4 ];
static  CW6E: [i8;8] = [ 0, 1, 1, 2, 2, 3, 3, 4 ];
static  CW7E: [i8;8] = [ 0, 1, 0, 2, 0, 4, 0, 3 ];
static  CW8E: [i8;8] = [ 3, 4, 2, 4, 1, 2, 0, 1 ];
static  CW9E: [i8;8] = [ 3, 4, 2, 4, 1, 2, 0, 1 ];
static CW10E: [i8;8] = [ 0, 1, 0, 3, 0, 4, 0, 2 ];
static CW11E: [i8;8] = [ 0, 1, 0, 2, 0, 4, 0, 3 ];

}
