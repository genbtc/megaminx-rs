//2024 megaminx-rs face.rs , by genr8eofl - LICENSED APGL3
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(unreachable_code)]
pub mod face {
  use crate::piece::piece::*;
  use crate::center::center::Center;
  use crate::corner::corner::Corner;
  use crate::edge::edge::Edge;
  //Face Data
  #[derive(Default)]
  pub struct Face {
    this_num: usize,
    pub turn_dir: TurnDir,
    rotating: bool,
    angle: f32,
    axis: [f32;3],
    do_axes: bool,
    //Duplicated from Piece Struct since no longer a Piece
    default_piece_num: usize,
    data: PieceData,
    //Boxed References to Trait Objects
    pub center: Vec<Box<dyn Center>>,
    pub corner: Vec<Box<dyn Corner>>,
    pub edge:   Vec<Box<dyn Edge>>,
    //TODO: hold a pointer back to the parent megaminx
    //Megaminx *megaminx;
    center_vertex_list: Vec<[Vertex3;7]>,
    edge_vertex_list:   Vec<[Vertex3;7]>,
    corner_vertex_list: Vec<[Vertex3;7]>,
  }
  /*Initialize constructor */
  impl Face {
    pub fn new(num: usize) -> Self {
      Self {
        this_num: num, turn_dir: TurnDir::None, rotating: false, angle: 0.0, axis: VERTEXZERO, do_axes: false, default_piece_num: num, data: Default::default(),
        center: Default::default(), corner: Default::default(), edge: Default::default(),
        center_vertex_list: vec![VERTEXDATAZERO], edge_vertex_list: vec![VERTEXDATAZERO], corner_vertex_list: vec![VERTEXDATAZERO],
      }
    }
  }
  
  pub trait FaceAttachment {
    fn num(&self) -> usize;
    fn attach_center(&mut self, center: Box<Piece>);     //(Center* c, double* centerVertexBase);
    fn attach_corner_pieces(&mut self, corner: Box<Piece>); //(const Megaminx* megaminx, Corner& cornersPTR);
    fn attach_edge_pieces(&mut self, edge: Box<Piece>);      //(const Megaminx* megaminx, Edge& edgesPTR);
    fn get_edge_piece<Piece:Edge>(&mut self, n: usize, i: usize) -> &mut Box<dyn Edge>;
    fn get_center_piece<Piece:Center>(&mut self, n: usize, i: usize) -> &mut Box<dyn Center>;
    fn get_corner_piece<Piece:Corner>(&mut self, n: usize, i: usize) -> &mut Box<dyn Corner>;    
  }
  impl FaceAttachment for Face {
    fn num(&self) -> usize { 
        return self.this_num;
    }
    fn attach_center(&mut self, center: Box<Piece>) {
        self.center.push(center);
    }
    fn attach_corner_pieces(&mut self, corner: Box<Piece>) {
        self.corner.push(corner);
    }
    fn attach_edge_pieces(&mut self, edge: Box<Piece>) { 
        self.edge.push(edge);
        /*
      let defaultEdges = <crate::megaminx::megaminx::Megaminx as crate::megaminx::megaminx::MegaminxFindPieces>::find_pieces_of_face(self,thisNum+1, edgesPTR, Megaminx::numEdges);
      for i in 0..5 {
          edge[i] = &dyn EdgesPTR + defaultEdges[i];
          assert(edge[i]->data.pieceNum == defaultEdges[i]);
      }  */
    }
    fn get_edge_piece<Piece:Edge>(&mut self, _n: usize, i: usize) -> &mut Box<dyn Edge> {
        &mut self.edge[i]
    }
    fn get_center_piece<Piece:Center>(&mut self, _n: usize, _i: usize) -> &mut Box<dyn Center> {
        &mut self.center[0]
    }
    fn get_corner_piece<Piece:Corner>(&mut self, _n: usize, i: usize) -> &mut Box<dyn Corner> {
        &mut self.corner[i]
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

  pub trait FacePlacement {
    fn place_parts(&mut self, dir: TurnDir) -> bool;
    fn two_edges_flip(&mut self, a: usize, b: usize);
    fn flip_corners(&mut self, a: usize, b: usize, c: usize, d: usize, pack: [usize;4]);
    fn quad_swap_pieces(&mut self, pack: [usize;8]);
    fn quad_swap_edges(&mut self, pack: [usize;8]) ;
    fn quad_swap_corners(&mut self, pack: [usize;8]);
    fn swap_pieces(&mut self, a: usize, b: usize);
    fn get_face_piece(&mut self, n: usize, i: usize);
    fn rotate(&mut self, direction: i8);
    fn render(&mut self) -> bool;
  }
  /**
   * \brief Colorizing function. Intricate series of flips/swaps.
  * \param dir Each case is for each of the 12 faces,
  * / in order to get it to switch colors after it rotates.
  * / called from render()
  */
  impl FacePlacement for Face {
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
        // assert!(self.edge[a].len() > a); assert!(self.edge[b].len() > b);
        // error[E0599]: the method `len` exists for struct `Box<dyn Edge>`, but its trait bounds were not satisfied
        //  |                                ^^^ method cannot be called on `Box<dyn Edge>` due to unsatisfied trait bounds
        //and
        todo!(); // SEG FAULT [a] overflow, Face Not Attached To anything.
        //Has to be attached to the dyn Trait objects to read (get) the Data struct.
        //Hopefully the mut swap still works.
        let mut edge_data_a = &self.edge[a];
        let mut edge_data_b = &self.edge[b];
        //HAS to be broken into two because of mutability slicing
        std::mem::swap(&mut edge_data_a, &mut edge_data_b);
        // ABOVE WORKS BUT BELOW DOES NOT
        //std::mem::swap(&mut self.edge[a].data, &mut self.edge[b].data);
//        |         --------------      ---------               ^^^^^^^^^ second mutable borrow occurs here
//        |         |                   ^ first mutable borrow occurs here
//        |         first borrow later used by call
//  This way definitely does not work.
//        &self.edge[a].swapdata(&mut self.edge[b].data);
//      |          ---------    --------      ^^^^^^^^^ second mutable borrow occurs here
//      |          |            ^ first borrow later used by call
//      |          first mutable borrow occurs here
//        //issues with lifetimes <'a> <_>
//error[E0599]: no method named `swapdata` found for struct `Box<dyn Edge>` in the current scope  
    }
    fn get_face_piece(&mut self, _n: usize, _i: usize) {
        //- `match` arms have incompatible types
        // match n {
        //     1 => self.get_center_piece(n,i),
        //     2 => self.get_edge_piece(n,i),
        //     3 => self.get_corner_piece(n,i),
        // }
    }
    //  = note: expected mutable reference `&mut Box<(dyn center::center::Center + 'static)>`
    //             found mutable reference `&mut Box<dyn Edge> + 'static`

    //    expected `&mut Box<(dyn center::center::Center + 'static)>` because of return type            

        /* if (std::is_same<T, Edge>::value)
        else if (std::is_same<T, Corner>::value)  */
        //return &mut self.center[0];
        //return &mut Box::<T: Piece>::new(Piece::new(1));
        //^^^^^^^^^^^^^^^^^^^ expected `&mut Box<T>`, found `&mut Box<dyn Center>

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
