//2024 megaminx-rs face.rs , by genr8eofl - LICENSED APGL3
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]
#![allow(unused_imports)]
pub mod face {
  use crate::piece::piece::PieceData;
  use crate::piece::piece::PieceMath;  
  use crate::piece::piece::Piece;
  use crate::center::center::Center;
  use crate::piece::piece::VERTEXZERO;

  //Face Data
  #[derive(Default)]
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
    //Boxed References to Trait Objects
    pub center: Vec<Box<dyn Center>>,
    corners: Vec<Box<dyn Corner>>,
    edges: Vec<Box<dyn Edge>>,
    //TODO: hold a pointer back to the parent megaminx
    //Megaminx *megaminx;
  }
  /*Initialize constructor */
  impl Face {
    pub fn new(num: i8) -> Self {
      Self {
        this_num: num, turn_dir: 0, rotating: false, angle: 0.0, axis: VERTEXZERO, do_axes: false, default_piece_num: num, data: Default::default(),
        center: vec![Box::<Piece>::new(Default::default())], corners: vec![Box::<Piece>::new(Default::default())], edges: vec![Box::<Piece>::new(Default::default())],
      }
    }
  }  
  //included from center.rs already;
/*pub trait Center {
    fn init(&mut self, piecenum: i8);
    fn create_axis(&mut self, piecenum: i32, index: usize);
    fn render(&self);
    fn new(&mut self);
  } */
  impl Center for Face {
    fn new(&mut self){
      return Default::default();
    }
   /**
     * \brief Inits a Face piece based on Center
     * \note  (calls createAxis and initColor)
     * \param n the number of the Face piece (piecenum)
     * \param doAxes True by default. First Time Initialization Only
     */
    fn init(&mut self, piecenum: i8) {
        if self.do_axes {
            for i in 0..5 {
              Center::create_axis(self, piecenum as i32, i);
            }
        }
        //TODO: initColor(G_FACEPIECESCOLORS[piecenum], true);
        self.data.pieceNum = piecenum;
        self.default_piece_num = piecenum;
    }
    fn create_axis(&mut self, piecenum: i32, _index: usize) {
        self.init(piecenum as i8);
    }
    fn render(&mut self) {
        self.place_parts(self.this_num);
    }
  }

  use crate::edge::edge::Edge;
  use crate::corner::corner::Corner;

  pub trait FaceFunctions {
    fn getnum(&self) -> i8;
    fn attach_center(&mut self, _center: &Box <dyn Center>);     //(Center* c, double* centerVertexBase);
    fn attach_corner_pieces(&self, _corners: &Box <dyn Corner>); //(const Megaminx* megaminx, Corner& cornersPTR);
    fn attach_edge_pieces(&self, _edges: &Box<dyn Edge>);      //(const Megaminx* megaminx, Edge& edgesPTR);
  }
  impl FaceFunctions for Face {
    fn getnum(&self) -> i8 { 
        return self.this_num;
    }
    fn attach_center(&mut self, _center: &Box <dyn Center>) {
        self.center[self.this_num as usize].init(self.this_num);
        println!("attach_center to face num {}", self.this_num);
    }
    fn attach_corner_pieces(&self, _corners: &Box <dyn Corner>) { /*
      const int color = faces[face - 1].center->data._colorNum[0];
      defaultCorners = megaminx->findPiecesOfFace(thisNum+1, cornersPTR, Megaminx::numCorners);
      for i in 0..5 {
          corner[i] = &dyn CornersPTR + defaultCorners[i];
          assert(corner[i]->data.pieceNum == defaultCorners[i]);
      }  */
    }
    fn attach_edge_pieces(&self, _edges: &Box <dyn Edge>) {  /*
      defaultEdges = megaminx->findPiecesOfFace(thisNum+1, edgesPTR, Megaminx::numEdges);
      for i in 0..5 {
          edge[i] = &dyn EdgesPTR + defaultEdges[i];
          assert(edge[i]->data.pieceNum == defaultEdges[i]);
      }  */
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

trait FacePlaceFunctions {
  fn place_parts(&mut self, dir: i8) -> bool;
  fn two_edges_flip(&mut self, a: i8, b: i8);
  fn flip_corners(&mut self, a: i8, b: i8, c: i8, d: i8, pack: [i8;4]);
  fn quad_swap_pieces(&mut self, pack: [i8;8]);
  fn quad_swap_edges(&mut self, pack: [i8;8]) ;
  fn quad_swap_corners(&mut self, pack: [i8;8]);
  fn swap_pieces(&mut self, a: i8, b: i8);
  fn get_face_piece(&mut self, i: i8);
  fn rotate(&mut self, direction: i8);
}
use crate::face::face::TurnDir::{CounterClockwise,Clockwise};
/**
 * \brief Colorizing function. Intricate series of flips/swaps.
 * \param dir Each case is for each of the 12 faces,
 * / in order to get it to switch colors after it rotates.
 * / called from render()
 */
impl FacePlaceFunctions for Face {
  fn two_edges_flip(&mut self, a: i8, b: i8) {
      assert!(a >= 0  && a < 5 && b >= 0 && b < 5);
      todo!(); 
      /*
      edge[a]->flip();
      edge[b]->flip(); */
  }
  fn flip_corners(&mut self, a: i8, b: i8, c: i8, d: i8, pack: [i8;4]){
      todo!(); /*
      //Feed in 4 ints a,b,c,d representing four of the face's Five Corner indexes (Range 0-4)
      //Feed in these Flip lists like { 0, 1, 1, 0 }; telling each index how to flip
      // Boolean ? 1 = Flip piece once ||  0      = Flip twice
      pack[0] ? corner[a]->flip() : corner[a]->flipTwice();
      pack[1] ? corner[b]->flip() : corner[b]->flipTwice();
      pack[2] ? corner[c]->flip() : corner[c]->flipTwice();
      pack[3] ? corner[d]->flip() : corner[d]->flipTwice();   */
  }
  //Private. Swap 4 Pieces, given a list of 8 indexes
  fn quad_swap_pieces(&mut self, pack: [i8;8]) {
      for i in (0..8).step_by(2) {
          self.swap_pieces(pack[i], pack[i+1]);
      }
  }
  fn quad_swap_edges(&mut self, pack: [i8;8]) {
      self.quad_swap_pieces(pack);
  }
  fn quad_swap_corners(&mut self, pack: [i8;8]) {
      self.quad_swap_pieces(pack);
  }
  /* Public. Given two pieces on the face with local indexes 0-5, swap them. */
  fn swap_pieces(&mut self, a: i8, b: i8) {
      assert!(a >= 0 && a < 5 && b >= 0 && b < 5);
      todo!(); /*
      Piece* pieceA = getFacePiece(a);
      Piece* pieceB = getFacePiece(b);
      pieceA->swapdata(pieceB->data); */
  }
  fn get_face_piece(&mut self, i: i8) {
      todo!(); /*
      if (std::is_same<T, Edge>::value)
          return edge[i];
      else if (std::is_same<T, Corner>::value)
          return corner[i];
      return center; */
  }
  /**
   * \brief Colorizing function. Intricate series of flips/swaps.
  *  \param dir Each case is for each of the 12 faces,
  *   in order to get it to switch colors after it rotates.
  *   called from render() */
  fn place_parts(&mut self, dir: i8) -> bool {
    assert!(dir == CounterClockwise as i8 || dir == Clockwise as i8);
    if dir == CounterClockwise as i8 { // 1 = CCW = Left Turn = Counter-ClockWise
        match self.this_num {
        0 => { //WHITE
            self.quad_swap_edges(CCW0E);
            self.quad_swap_corners(CCW0C); }
        1 => { //DARK_BLUE
            self.quad_swap_edges(CCW1E);
            self.two_edges_flip(1, 2);
            self.quad_swap_corners(CCW1C);
            self.flip_corners(0, 2, 3, 4, FlipBackwards); }
        2 => { //RED
            self.quad_swap_edges(CCW2E);
            self.two_edges_flip(1, 2);
            self.quad_swap_corners(CCW2C);
            self.flip_corners(1, 2, 3, 4, FlipBackwardAlt); }
        3 => { //DARK_GREEN
            self.quad_swap_edges(CCW3E);
            self.two_edges_flip(1, 2);
            self.quad_swap_corners(CCW3C);
            self.flip_corners(1, 2, 3, 4, FlipBackwardAlt); }
        4 => { //PURPLE
            self.quad_swap_edges(CCW4E);
            self.two_edges_flip(1, 2);
            self.quad_swap_corners(CCW4C);
            self.flip_corners(1, 2, 3, 4, FlipBackwardAlt); }
        5 => { //YELLOW
            self.quad_swap_edges(CCW5E);
            self.two_edges_flip(1, 2);
            self.quad_swap_corners(CCW5C);
            self.flip_corners(1, 2, 3, 4, FlipBackwardAlt); }
        6 => { //GRAY
            self.quad_swap_edges(CCW6E);
            self.quad_swap_corners(CCW6C); }
        7 => { //LIGHT_BLUE Front Face, Left Turn = Counter-ClockWise;
            self.quad_swap_edges(CCW7E);
            self.two_edges_flip(3, 4);
            self.quad_swap_corners(CCW7C);
            self.flip_corners(0, 1, 2, 3, FlipForwardAlt); }
        8 => { //ORANGE
            self.quad_swap_edges(CCW8E);
            self.two_edges_flip(3, 4);
            self.quad_swap_corners(CCW8C);
            self.flip_corners(0, 1, 2, 3, FlipForwards); }
        9 => { //GREEN
            self.quad_swap_edges(CCW9E);
            self.two_edges_flip(3, 4);
            self.quad_swap_corners(CCW9C);
            self.flip_corners(0, 1, 2, 3, FlipForwards); }
        10 => { //PINK
            self.quad_swap_edges(CCW10E);
            self.two_edges_flip(2, 4);
            self.quad_swap_corners(CCW10C);
            self.flip_corners(0, 1, 2, 3, FlipForwards); }
        11 => { //BEIGE
            self.quad_swap_edges(CCW11E);
            self.two_edges_flip(3, 4);
            self.quad_swap_corners(CCW11C);
            self.flip_corners(0, 1, 2, 4, FlipForwards); }
        _ => {
            println!("CCW Face must be in 0-11") },
        }
    }
    else {  // -1 = CW = Right Turn = ClockWise
        match self.this_num {
        0 => { //WHITE
            self.quad_swap_edges(CW0E);
            self.quad_swap_corners(CW0C); }
        1 => { //DARK_BLUE
            self.quad_swap_edges(CW1E);
            self.two_edges_flip(0, 3);
            self.quad_swap_corners(CW1C);
            self.flip_corners(0, 1, 2, 4, FlipForwards); }
        2 => { //RED
            self.quad_swap_edges(CW2E);
            self.two_edges_flip(0, 3);
            self.quad_swap_corners(CW2C);
            self.flip_corners(0, 1, 3, 4, FlipForwards); }
        3 => { //DARK_GREEN
            self.quad_swap_edges(CW3E);
            self.two_edges_flip(0, 3);
            self.quad_swap_corners(CW3C);
            self.flip_corners(0, 1, 3, 4, FlipForwards); }
        4 => { //PURPLE
            self.quad_swap_edges(CW4E);
            self.two_edges_flip(0, 3);
            self.quad_swap_corners(CW4C);
            self.flip_corners(0, 1, 3, 4, FlipForwards); }
        5 => { //YELLOW
            self.quad_swap_edges(CW5E);
            self.two_edges_flip(0, 3);
            self.quad_swap_corners(CW5C);
            self.flip_corners(0, 1, 3, 4, FlipForwards); }
        6 => { //GRAY
            self.quad_swap_edges(CW6E);
            self.quad_swap_corners(CW6C); }
        7 => { //LIGHT_BLUE Front Face, Right Turn = ClockWise;
            self.quad_swap_edges(CW7E);
            self.two_edges_flip(0, 3);
            self.quad_swap_corners(CW7C);
            self.flip_corners(0, 1, 3, 4, FlipBackwards); }
        8 => { //ORANGE
            self.quad_swap_edges(CW8E);
            self.two_edges_flip(0, 3);
            self.quad_swap_corners(CW8C);
            self.flip_corners(0, 2, 3, 4, FlipBackwards); }
        9 => { //GREEN
            self.quad_swap_edges(CW9E);
            self.two_edges_flip(0, 3);
            self.quad_swap_corners(CW9C);
            self.flip_corners(0, 2, 3, 4, FlipBackwards); }
        10 => { //PINK
            self.quad_swap_edges(CW10E);
            self.two_edges_flip(0, 2);
            self.quad_swap_corners(CW10C);
            self.flip_corners(0, 2, 3, 4, FlipBackwards); }
        11 => { //BEIGE
            self.quad_swap_edges(CW11E);
            self.two_edges_flip(0, 3);
            self.quad_swap_corners(CW11C);
            self.flip_corners(0, 2, 3, 4, FlipBackwards); }
        _ => {
              println!("CW Face must be in 0-11") },
          }
    }
    return true;
  }
  
  /**
   * \brief Public. Calling this sets off a chain of events in the render loops to rotate.
   * \param direction turn direction: -1 for Right, +1 for left (seems/is backwards). */
  fn rotate(&mut self, direction: i8) {
      assert!(direction == Clockwise as i8 || direction == CounterClockwise as i8);
      self.rotating = true;
      self.turn_dir = direction;
  }

}

}
