//2024 megaminx-rs face.rs , by genr8eofl - LICENSED APGL3
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
pub mod face {
  use crate::piece::piece::Vertex3;
  use crate::piece::piece::VERTEXDATAZERO;
  use crate::piece::piece::VERTEXZERO;
  use crate::piece::piece::PieceData;
  use crate::piece::piece::PieceMath;
  use crate::piece::piece::Piece;
  use crate::piece::piece::VertexPositionColor;
  use crate::center::center::Center;

  //Face Data
  #[derive(Default)]
  pub struct Face {
    this_num: usize,
    turn_dir: TurnDir,
    rotating: bool,
    angle: f32,
    axis: [f32;3],
    do_axes: bool,
    //Duplicated from Piece Struct since no longer a Piece
    default_piece_num: usize,
    data: PieceData,
    //Boxed References to Trait Objects
    pub center: Vec<Box<(dyn Center + 'static)>>,
    pub corner: Vec<Box<Piece>>,
    pub edge: Vec<Box<Piece>>,
    //TODO: hold a pointer back to the parent megaminx
    //Megaminx *megaminx;
    center_vertex_list: [Vertex3; 7],
    edge_vertex_list:   [Vertex3; 7],
    corner_vertex_list: [Vertex3; 7],
  }
  /*Initialize constructor */
  impl Face {
    pub fn new(num: usize) -> Self {
      Self {
        this_num: num, turn_dir: TurnDir::None, rotating: false, angle: 0.0, axis: VERTEXZERO, do_axes: false, default_piece_num: num, data: Default::default(),
        center: Default::default(), corner: Default::default(), edge: Default::default(),
        center_vertex_list: VERTEXDATAZERO, edge_vertex_list: VERTEXDATAZERO, corner_vertex_list: VERTEXDATAZERO,
      }
    }
  }  
/* included from center.rs already;
// pub trait Center {
    fn newa(&mut self);
    fn init(&mut self, piecenum: usize);
    fn create_axis(&mut self, piecenum: usize, index: usize);
    fn render(&mut self) -> Vec<VertexPositionColor>;
    fn render_lines(&self) -> Vec<VertexPositionColor>;
// } */
  impl Center for Face {
    fn getnum(&self) -> usize { 
        return self.default_piece_num;
    }
    fn new(&mut self) {
        return Default::default();
    }
   /**
     * \brief Inits a Face piece based on Center
     * \note  (calls createAxis and initColor)
     * \param n the number of the Face piece (piecenum)
     * \param doAxes True by default. First Time Initialization Only
     */
    fn init(&mut self, piecenum: usize) {
        if self.do_axes {
            for i in 0..5 {
              Center::create_axis(self, piecenum, i);
            }
        }
        //self.initColor(piecenum + 1);  //from Piece
//        |              ^^^^^^^^^ method not found in `&mut Face`
//        = help: items from traits can only be used if the trait is implemented and in scope
//      note: `piece::piece::PieceColor` defines an item `initColor`, perhaps you need to implement it        
        self.data.pieceNum = piecenum;
        self.default_piece_num = piecenum;
    }
    fn create_axis(&mut self, piecenum: usize, _index: usize) {
        self.init(piecenum);
    }
    fn render(&mut self) -> Vec<VertexPositionColor> {
        self.place_parts(self.turn_dir);
        //THIS WAS PLACED HERE ON PURPOSE TO SATISFY THE RETURN VALUE OF CENTER.RS
        vec![VertexPositionColor { position: self.axis, color: self.data.color.colorRGB[0] } ]
    }
    fn render_lines(&self) -> Vec<VertexPositionColor> {
        //THIS WAS PLACED HERE ON PURPOSE TO SATISFY THE RETURN VALUE OF CENTER.RS
        vec![VertexPositionColor { position: self.axis, color: self.data.color.colorRGB[0] } ]
    }    
  }

  pub trait FaceFunctions {
    fn num(&self) -> usize;
    fn attach_center(&mut self, centers: &mut Vec<Box<(dyn Center + 'static)>>);     //(Center* c, double* centerVertexBase);
    fn attach_corner_pieces(&mut self, _corners: &Vec<Box<Piece>>); //(const Megaminx* megaminx, Corner& cornersPTR);
    fn attach_edge_pieces(&mut self, _edges: &Vec<Box<Piece>>);      //(const Megaminx* megaminx, Edge& edgesPTR);
  }
  impl FaceFunctions for Face {
    fn num(&self) -> usize { 
        return self.this_num;
    }
    fn attach_center(&mut self, centers: &mut Vec<Box <(dyn Center + 'static)>>) {
        //println!("face.attach_center() to {}", self.this_num);
        //self.initColor(piecenum + 1);  //from Piece, unavailable here.
        self.init(self.this_num);
        //self.create_axis(self.this_num, self.this_num);
        if self.center.len() == 0 {
            Center::init(&mut *centers[self.this_num], self.this_num);
        }
        //self.center.push(Box::new(&mut *centers[self.this_num]));
        //self.center.push(Box::new(centers[self.this_num]));
        //self.center.push(centers[self.this_num]);
    //     error[E0507]: cannot move out of index of `Vec<Box<dyn center::center::Center>>`
    //      |                          ^^^^^^^^^^^^^^^^^^^^^^ move occurs because value has type `Box<dyn center::center::Center>`, which does not implement the `Copy` trait        
// and
    //     error[E0277]: the trait bound `&mut (dyn center::center::Center + 'static): center::center::Center` is not satisfied
    //      |                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `center::center::Center` is not implemented for `&mut (dyn center::center::Center + 'static)`
    //      = help: the following other types implement trait `center::center::Center`:
    //                Face
    //                Piece
    //      = note: required for the cast from `Box<&mut (dyn center::center::Center + 'static)>` to `Box<(dyn center::center::Center + 'static)>`        
//    }
// and
        //self.center.push(Box::new(*centers[self.this_num]));
    }
//    error[E0277]: the size for values of type `dyn center::center::Center` cannot be known at compilation time
//     |                          -------- ^^^^^^^^^^^^^^^^^^^^^^^ doesn't have a size known at compile-time
//     |                          required by a bound introduced by this call
//     = help: the trait `Sized` is not implemented for `dyn center::center::Center`
// note: required by a bound in `Box::<T>::new`
    
    fn attach_corner_pieces(&mut self, corners: &Vec<Box<Piece>>) { 
        self.corner.push(Box::new(*corners[self.this_num]));
    /* let color = faces[face - 1].center.data._colorNum[0];
      defaultCorners = megaminx->findPiecesOfFace(thisNum+1, cornersPTR, Megaminx::numCorners);
      for i in 0..5 {
          corner[i] = &dyn CornersPTR + defaultCorners[i];
          assert_eq!(corner[i].data.pieceNum, defaultCorners[i]);
      }  */
      //error[E0609]: no field `data` on type `Box<(dyn center::center::Center + 'static)>`
    }
    fn attach_edge_pieces(&mut self, edges: &Vec<Box<Piece>>) { 
        self.edge.push(Box::new(*edges[self.this_num]));
        /*
      defaultEdges = megaminx->findPiecesOfFace(thisNum+1, edgesPTR, Megaminx::numEdges);
      for i in 0..5 {
          edge[i] = &dyn EdgesPTR + defaultEdges[i];
          assert(edge[i]->data.pieceNum == defaultEdges[i]);
      }  */
    }
  }


  #[derive(Copy, Clone, Default, PartialEq)]
  pub enum TurnDir { 
    Clockwise = -1,
    #[default] None = 0,
    CounterClockwise = 1
  }
  use TurnDir::{ Clockwise, CounterClockwise };

  //Named Flip Direction lists:
  static FlipInwards:     [usize;4] = [ 0, 1, 1, 0 ];
  static FlipOutwards:    [usize;4] = [ 1, 0, 0, 1 ];
  static FlipBackwards:   [usize;4] = [ 0, 0, 1, 1 ];
  static FlipForwards:    [usize;4] = [ 1, 1, 0, 0 ];
  static FlipBackwardAlt: [usize;4] = [ 0, 1, 0, 1 ];
  static FlipForwardAlt:  [usize;4] = [ 1, 0, 1, 0 ];

  //These are invoked when Face::placeParts() is ran, when it's rotating.
  //Called from Face::render(), only when something is moved, NEVER on startup.
  //Flip direction lists for PlaceParts: //CounterClockwise CORNERS
  //CCW Corners
  static  CCW0C: [usize;8] = [ 0, 1, 1, 2, 2, 3, 3, 4 ];
  static  CCW1C: [usize;8] = [ 0, 2, 0, 4, 0, 3, 0, 1 ];
  static  CCW2C: [usize;8] = [ 0, 1, 0, 2, 2, 3, 2, 4 ];
  static  CCW3C: [usize;8] = [ 3, 4, 1, 3, 1, 2, 0, 1 ];
  static  CCW4C: [usize;8] = [ 0, 1, 0, 3, 0, 4, 0, 2 ];
  static  CCW5C: [usize;8] = [ 1, 3, 1, 4, 1, 2, 0, 1 ];
  static  CCW6C: [usize;8] = [ 0, 1, 4, 0, 3, 4, 2, 3 ];
  static  CCW7C: [usize;8] = [ 1, 3, 3, 4, 4, 2, 2, 0 ];
  static  CCW8C: [usize;8] = [ 4, 3, 4, 2, 4, 0, 4, 1 ];
  static  CCW9C: [usize;8] = CCW8C;
  static CCW10C: [usize;8] = CCW8C;
  static CCW11C: [usize;8] = [ 4, 2, 4, 3, 3, 1, 1, 0 ];
  //Flip direction lists for PlaceParts: //Clockwise CORNERS
  //CW Corners
  static  CW0C: [usize;8] = CCW6C;
  static  CW1C: [usize;8] = CCW4C;
  static  CW2C: [usize;8] = [ 0, 1, 1, 2, 1, 3, 3, 4 ];
  static  CW3C: [usize;8] = CW2C;
  static  CW4C: [usize;8] = CCW1C;
  static  CW5C: [usize;8] = [ 0, 1, 1, 2, 1, 4, 1, 3 ];
  static  CW6C: [usize;8] = CCW0C;
  static  CW7C: [usize;8] = [ 2, 0, 4, 2, 3, 4, 1, 3 ];
  static  CW8C: [usize;8] = [ 4, 1, 4, 0, 4, 2, 4, 3 ];
  static  CW9C: [usize;8] = CW8C;
  static CW10C: [usize;8] = CW8C;
  static CW11C: [usize;8] = [ 1, 0, 3, 1, 4, 3, 2, 4 ];
  //Flip direction lists for PlaceParts: //CounterClockwise Edges
  //CCW Edges
  static  CCW0E: [usize;8] = [ 0, 1, 1, 2, 2, 3, 3, 4 ];
  static  CCW1E: [usize;8] = [ 4, 1, 1, 3, 0, 1, 0, 2 ];
  static  CCW2E: [usize;8] = [ 1, 0, 1, 2, 1, 3, 3, 4 ];
  static  CCW3E: [usize;8] = [ 3, 2, 4, 3, 0, 1, 1, 2 ];
  static  CCW4E: [usize;8] = [ 0, 1, 1, 2, 1, 3, 3, 4 ];
  static  CCW5E: [usize;8] = [ 2, 4, 2, 3, 0, 2, 0, 1 ];
  static  CCW6E: [usize;8] = [ 0, 1, 4, 0, 3, 4, 2, 3 ];
  static  CCW7E: [usize;8] = [ 0, 3, 0, 4, 0, 2, 0, 1 ];
  static  CCW8E: [usize;8] = [ 0, 1, 1, 2, 2, 4, 3, 4 ];
  static  CCW9E: [usize;8] = CCW8E;
  static CCW10E: [usize;8] = [ 0, 2, 0, 4, 0, 3, 0, 1 ];
  static CCW11E: [usize;8] = CCW7E;
  //Flip direction lists for PlaceParts: //Clockwise Edges
  //CW Edges
  static  CW0E: [usize;8] = CCW6E;
  static  CW1E: [usize;8] = [ 0, 2, 0, 1, 1, 3, 4, 1 ];
  static  CW2E: [usize;8] = [ 3, 4, 1, 3, 1, 2, 1, 0 ];
  static  CW3E: [usize;8] = [ 1, 2, 0, 1, 4, 3, 3, 2 ];
  static  CW4E: [usize;8] = [ 3, 4, 1, 3, 1, 2, 0, 1 ];
  static  CW5E: [usize;8] = [ 0, 1, 0, 2, 2, 3, 2, 4 ];
  static  CW6E: [usize;8] = CCW0E;
  static  CW7E: [usize;8] = [ 0, 1, 0, 2, 0, 4, 0, 3 ];
  static  CW8E: [usize;8] = [ 3, 4, 2, 4, 1, 2, 0, 1 ];
  static  CW9E: [usize;8] = CW8E;
  static CW10E: [usize;8] = [ 0, 1, 0, 3, 0, 4, 0, 2 ];
  static CW11E: [usize;8] = CW7E;

  pub trait FacePlaceFunctions {
    fn place_parts(&mut self, dir: TurnDir) -> bool;
    fn two_edges_flip(&mut self, a: usize, b: usize);
    fn flip_corners(&mut self, a: usize, b: usize, c: usize, d: usize, pack: [usize;4]);
    fn quad_swap_pieces(&mut self, pack: [usize;8]);
    fn quad_swap_edges(&mut self, pack: [usize;8]) ;
    fn quad_swap_corners(&mut self, pack: [usize;8]);
    fn swap_pieces(&mut self, a: usize, b: usize);
    fn get_face_piece<T: PieceMath>(&mut self, n: usize, i: usize) ; //-> &mut Box<Piece>;
    fn rotate(&mut self, direction: i8);
    fn render(&mut self) -> bool;
  }
  /**
   * \brief Colorizing function. Intricate series of flips/swaps.
  * \param dir Each case is for each of the 12 faces,
  * / in order to get it to switch colors after it rotates.
  * / called from render()
  */
  impl FacePlaceFunctions for Face {
    fn two_edges_flip(&mut self, a: usize, b: usize) {
        assert!(a < 5 && b < 5);
        self.edge[a].flip();
        self.edge[b].flip();
    }
    fn flip_corners(&mut self, a: usize, b: usize, c: usize, d: usize, pack: [usize;4]){
        //Feed in 4 ints a,b,c,d representing four of the face's Five Corner indexes (Range 0-4)
        //Feed in these Flip lists like { 0, 1, 1, 0 }; telling each index how to flip
        // Booleanif { 1 = Flip piece once ||  0      = Flip twice
        if pack[0] !=0 { self.corner[a].flip(); } else { self.corner[a].flip_twice(); }
        if pack[1] !=0 { self.corner[b].flip(); } else { self.corner[b].flip_twice(); }
        if pack[2] !=0 { self.corner[c].flip(); } else { self.corner[c].flip_twice(); }
        if pack[3] !=0 { self.corner[d].flip(); } else { self.corner[d].flip_twice(); }
    }
    //Private. Swap 4 Pieces, given a list of 8 indexes
    fn quad_swap_pieces(&mut self, pack: [usize;8]) {
        for i in (0..8).step_by(2) {
            self.swap_pieces(pack[i], pack[i+1]);
        }
    }
    fn quad_swap_edges(&mut self, pack: [usize;8]) {
        self.quad_swap_pieces(pack);
    }
    fn quad_swap_corners(&mut self, pack: [usize;8]) {
        self.quad_swap_pieces(pack);
    }
    /* Public. Given two pieces on the face with local indexes 0-5, swap them. */
    fn swap_pieces(&mut self, a: usize, b: usize) {
        assert!(a < 5 && b < 5);
        let mut edge_data_a = &self.edge[a].data;
        let mut edge_data_b = &self.edge[b].data;
        std::mem::swap(&mut edge_data_a, &mut edge_data_b);
        // ABOVE WORKS BUT BELOW DOES NOT
        //std::mem::swap(&mut self.edge[a].data, &mut self.edge[b].data);
//        |         --------------      ---------               ^^^^^^^^^ second mutable borrow occurs here
//        |         |                   ^ first mutable borrow occurs here
//        |         first borrow later used by call
//  This definitely does not work.
//        &self.edge[a].swapdata(&mut self.edge[b].data);
//      |          ---------    --------      ^^^^^^^^^ second mutable borrow occurs here
//      |          |            |
//      |          |            first borrow later used by call
//      |          first mutable borrow occurs here        
    }
    fn get_face_piece<T: PieceMath>(&mut self, _n: usize, _i: usize) { //-> &mut Box<Piece> {
        // match n {
        //     1 => { return &mut self.corner[i]; }
        // //    expected `&mut Box<(dyn center::center::Center + 'static)>` because of return type
        //     2 => { return &mut self.edge[i];   }
        //     3 => { return &mut self.center[0]; },
        // }
        /* if (std::is_same<T, Edge>::value)
        else if (std::is_same<T, Corner>::value)  */
        //return &mut self.center[0];
        //return &mut Box::<T: Piece>::new(Piece::new(1));
        //^^^^^^^^^^^^^^^^^^^ expected `&mut Box<T>`, found `&mut Box<dyn Center>
    }

    /**
     * \brief Colorizing function. Intricate series of flips/swaps.
    *  \param dir Each case is for each of the 12 faces,
    *   in order to get it to switch colors after it rotates.
    *   called from render() */
    fn place_parts(&mut self, dir: TurnDir) -> bool {
      assert!(dir == CounterClockwise || dir == Clockwise);
      if dir == CounterClockwise { // 1 = CCW = Left Turn = Counter-ClockWise
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
     * \param  direction  turn direction: -1 for Right, +1 for left (seems/is backwards). */
    fn rotate(&mut self, direction: i8) {
        self.rotating = true;
        match direction { 
             1=> { self.turn_dir = CounterClockwise }
            -1=> { self.turn_dir = Clockwise }
            _ => { self.turn_dir = TurnDir::None }
        }
    }

    /**
     * \brief OpenGL Display function. Calling this makes the faces rotate,the only real move.
     * \return  true  if we full-spun, to tell the parent function that rotating=false also.
     */
    fn render(&mut self) -> bool {
        let turnspeed = 32;
        //Start Rotating
        if self.rotating {
            //glPushMatrix();
            self.angle += (self.turn_dir as i8 * turnspeed) as f32;
            //Slow down to half-speed once its 75% complete
            //  (56/72 is ~77.7% but use 56 because % mod 8 == 0)
            if self.angle >= 56. || self.angle <= -56. {
                self.angle -= (self.turn_dir as i8 * (turnspeed/2)) as f32;
            }
            //Rotate axis by angle
            //glRotated(self.angle, self.axis[0], self.axis[1], self.axis[2]);
        }
        else {
            self.angle = 0.;
        }

        //Render parts:
        for center in self.center.iter_mut() {
            center.render();
        }
        for edge in self.edge.iter_mut() {
            edge.render();
        }
        for corner in self.corner.iter_mut() {
            corner.render();
        }

        if self.angle > 0.0 {
            //glPopMatrix();
            //Color Black
            //glColor3d(0, 0, 0);
            //Draw a black pentagon to block out view from see-thru hollow insides
            //makeGLpolygon(self._vertex, 1.0 , 5);
        }
        //Done animating, clean up and commit
        // 72 is one fifth of 360 circle
        if self.angle >= 72. || self.angle <= -72. {
            self.angle = 0.;
            self.rotating = false;
            self.turn_dir = TurnDir::None; //NEW
            //returns True if successful
            return self.place_parts(self.turn_dir);
            //NOTE: ^^ internal structure of pieces is calculated last
        }
        return false;
    }

  }

  }
