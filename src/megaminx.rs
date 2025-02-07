//2024 megaminx-rs megaminx.rs , by genr8eofl - LICENSED APGL3
pub mod megaminx {
  use crate::piece::piece::*;
  pub use crate::face::face::{Face,FaceAttachment,FacePlacement};
  pub use crate::center::center::Center;
  pub use crate::corner::corner::Corner;
  pub use crate::edge::edge::Edge;
  use std::collections::VecDeque;

  pub const NUM_FACES:   usize = 12;
  pub const NUM_CORNERS: usize = 20;
  pub const NUM_EDGES:   usize = 30;

  pub struct Megaminx { 
    pub invisible: bool,
    pub is_rotating: bool,
        rotating_face_index: i8,
    pub faces: Vec<Box<Face>>,
    pub centers: Vec<Box<dyn Center>>,
    pub corners: Vec<Box<dyn Corner>>,
    pub edges:   Vec<Box<dyn Edge>>,
    pub g_current_face: Box<Face>,
    pub rotate_queue: VecDeque<NumDir>
  }
  
  impl Megaminx {
      /*Initializing new() constructor */
      pub fn new() -> Self {
      Self {
        invisible: false,
        is_rotating: false,
        rotating_face_index: -1,
        faces:   Default::default(),
        centers: Default::default(),
        corners: Default::default(),
        edges:   Default::default(),
        g_current_face: Default::default(),
        rotate_queue: Default::default(),
      }
    }
    /**
     * \brief Megaminx main simple constructor for init.
     * \note   Setup, Solve Puzzle (aka Reset), Render
     */
    pub fn init_reset(&mut self) {
        println!("Initializing Megaminx!");
        //(re)/initialize Struct w/ defaults
        self.g_current_face = Default::default();
        self.rotating_face_index = 0;
        self.is_rotating = false;
        self.invisible = false;
        //MegaminxInit Functions
        self.init_face_pieces();
        self.init_center_pieces();
        self.init_edge_pieces();
        self.init_corner_pieces();
        self.render();
    }

    /**
     * \brief Main Render Logic function - (start handling rotation calls and sub-object render calls)
     * \note Conditionally call each OpenGL .render() func (each rotating face, center, edge, corner)
     */
    fn render(&mut self) {
        //Skip everything if its invisible
        if self.invisible {
            return;
        }
        //Start the face rotation Queue for multiple ops.
        if ! self.rotate_queue.is_empty() {
            let Some(&ref op) = self.rotate_queue.front() else { todo!() };
            self.rotating_face_index = op.num;    //this is set only here
            assert!(self.rotating_face_index != -1);   //ensure safety
            self.is_rotating = true;
            self.faces[self.rotating_face_index as usize].rotate(op.dir);
        }
        // Full Re-render all if non-rotating or early startup
        //TODO:[Conditionally] Process all pieces that are NOT part of a rotating face.
        for i in 0..NUM_FACES {
            self.centers[i].render();
        }
        for i in 0..NUM_EDGES {
            self.edges[i].render();
        }
        for i in 0..NUM_CORNERS {
            //Corner::render(&*self.corners[i]);
            self.corners[i].render();
        }
        // (starts up with rotating_face_index is -1)
        //rest of function can be skipped to avoid array[-1] error
        if self.rotating_face_index == -1 {
            return;
        }
        //call .RENDER() and find out if successful
        let didrender: bool = FacePlacement::render(&mut *self.faces[self.rotating_face_index as usize]);
        if didrender && self.is_rotating {
            //If yes, then Finish the Rotation & advance the Queue
            self.rotate_queue.pop_front();
        }
    }

  }

  //Megaminx Init Pieces Setup
  pub trait MegaminxInit {
    fn init_face_pieces(&mut self);
    fn init_center_pieces(&mut self);
    fn init_edge_pieces(&mut self);
    fn init_corner_pieces(&mut self);
    fn print_vector(&mut self, piece: &Piece);
  }
  //use crate::piece::piece::EdgeCornerInit;
  impl MegaminxInit for Megaminx {
    /**
     * \brief Init the Faces and All Pieces.
     *         Set up the Axes of the faces, attach the centers, 
     *          and attach the Edge and Corner pieces to the Faces.
     */
    fn init_face_pieces(&mut self) {
        for i in 0..NUM_FACES {
            //println!("initing face: {}", i);
            let  face: Face = Face::new(i);
            self.faces.push(Box::new(face));
            //face.faceInit();
            //error[E0599]: no method named `faceInit` found for struct `Face` in the current scope
        }
    //fn attach_face_pieces(&mut self) {
            // self.find_pieces_of_face(i, &face, 5);
            // face.attach_center(&mut self.centers);
            // face.attach_edge_pieces(&mut self.edges);
            // face.attach_corner_pieces(&mut self.corners);
        //}
        assert_eq!(self.faces.len(), NUM_FACES);        
    }

    /** \brief Init the Centers, attach them to Faces. (numFaces = 12) */
    fn init_center_pieces(&mut self) {
        for i in 0..NUM_FACES {
            //println!("initing center: {}", i);
            let mut centerpiece: Piece = Piece::new(i);
            Center::init(&mut centerpiece, i);
            self.faces[i].attach_center(Box::new(centerpiece));
            self.centers.push(Box::new(centerpiece));
            //self.print_vector(&centerpiece);
        }
        assert_eq!(self.centers.len(), NUM_FACES);        
    }

    /** \brief Init the Edge pieces. (numEdges = 30)  */
    fn init_edge_pieces(&mut self) {
      //store a list of the basic starting Edge vertexes
      let mut foundedges: Vec<i8> = Vec::new();
      for i in 0..NUM_EDGES {
          //println!("initing edge: {}", i);
          let mut edgepiece: Piece = Piece::new(i);
          let edge_vertex_list: [Vertex3;7] = *edgepiece.edgeInit();
          edgepiece.init_edge_data(i, edge_vertex_list);
          self.edges.push(Box::new(edgepiece));
          //self.print_vector(&edgepiece);
          let [a,b,_] = edgepiece.getcolor().colorNum;
          if edgepiece.matchesColor(a) {
            self.faces[a - 1].attach_edge_pieces(Box::new(edgepiece));
            //print!("EdgecolorA: {:?} ", a);
          }
          if edgepiece.matchesColor(b) {
            self.faces[b - 1].attach_edge_pieces(Box::new(edgepiece));
            //println!("EdgecolorB: {:?}", b);
          }
          foundedges.extend(self.find_pieces_of_face(a - 1, &edgepiece, 1)); //30*5 = 150
      }
      //foundedges: [0, 1, 2, 3, 4, 0, 1, 2, 3, 4, 0, 1, 2, 3, 4, 0, 1, 2, 3, 4, 0, 1, 2, 3, 4]
      //foundedges: [0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 3, 3, 3, 3, 3, 4, 4, 4, 4, 4]
      // initing edge: 29
      // colorA: 7
      // colorB: 12
      // foundedges: [0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 3, 3, 3, 3, 3, 4, 4, 4, 4, 4, 5, 5, 5, 5, 5, 6, 6, 6, 6, 6, 7, 7, 7, 7, 7, 8, 8, 8, 8, 8, 9, 9, 9, 9, 9, 10, 10, 10, 10, 10, 11, 11, 11, 11, 11, 12, 12, 12, 12, 12, 13, 13, 13, 13, 13, 14, 14, 14, 14, 14, 15, 15, 15, 15, 15, 16, 16, 16, 16, 16, 17, 17, 17, 17, 17, 18, 18, 18, 18, 18, 19, 19, 19, 19, 19, 20, 20, 20, 20, 20, 21, 21, 21, 21, 21, 22, 22, 22, 22, 22, 23, 23, 23, 23, 23, 24, 24, 24, 24, 24, 25, 25, 25, 25, 25, 26, 26, 26, 26, 26, 27, 27, 27, 27, 27, 28, 28, 28, 28, 28, 29, 29, 29, 29, 29] 150
      // change function from 5 to 1  :
      // foundedges: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29] 30
      println!("foundedges: {:?} {}", foundedges, foundedges.len());
      assert_eq!(self.edges.len(), NUM_EDGES);        
  }

  /** \brief Init the Corner pieces. (numCorners = 20)  */
  fn init_corner_pieces(&mut self) {
      //store a list of the basic starting Corner vertexes
      for i in 0..NUM_CORNERS {
          //println!("initing corner: {}", i);
          let mut cornerpiece: Piece = Piece::new(i);
          let corner_vertex_list: [Vertex3;7] = *cornerpiece.cornerInit();
          cornerpiece.init_corner_data(i, corner_vertex_list);
          self.corners.push(Box::new(cornerpiece));
          //self.print_vector(&cornerpiece);
          let [a,b,c] = cornerpiece.getcolor().colorNum;
          if cornerpiece.matchesColor(a) {
            self.faces[a - 1].attach_corner_pieces(Box::new(cornerpiece));
            //print!("cornerColorA: {:?} ", a);
          }
          if cornerpiece.matchesColor(b) {
            self.faces[b - 1].attach_corner_pieces(Box::new(cornerpiece));
            //print!("cornerColorB: {:?} ", b);
          }
          if cornerpiece.matchesColor(c) {
            self.faces[c - 1].attach_corner_pieces(Box::new(cornerpiece));
            //print!("cornerColorC: {:?} \n", c);
          }
      }
      assert_eq!(self.corners.len(), NUM_CORNERS);        
  }

    fn print_vector(&mut self, piece: &Piece) {
      print!("Piece {} Vertex Array: [ ", piece.defaultPieceNum);
      for i in 0..5 { //TODO!: 6,7 for edge/corner
        print!("[ ");
        for j in 0..3 {
          print!("{}", piece.vertex[i][j].to_string());
          if j < piece.vertex[i].len() - 1  { print!(", "); }
        }
        if i < piece.vertex.len() - 1  { print!(" ], "); }
      }
      println!("]");
    }

  }

  //Control Functions
  pub trait MegaminxMove {
    fn undo();
    fn undo_double();
    fn undo_quad();
    fn undo_bulk();
    fn reset_queue();
    fn scramble();
  }

  pub trait MegaminxFindPieces {
    fn find_pieces_of_face(&mut self, face: usize, piece_ref: &Piece, times: i8) -> Vec<i8>;
  }
  /* \brief Finds the colored center that is perma-attached to a face, and then
  *         iterates the entire list of pieces to find when the colors match, and outputs a list.
  * \param face Nth-face number (1-12) //TODO: Adjusted -1
  * \param pieceRef Takes a reference to the [0]th member of Pointer_array of (either Corner/Edge's)
  * \param times how many times to iterate over the ref'd array
  * \return Returns the list of 5 positions where the starting face's pieces have ended up at.
  * \note    NOTE: Finds pieces BEFORE they are attached to a face.
  */
  impl MegaminxFindPieces for Megaminx {
    fn find_pieces_of_face(&mut self, face: usize, piece_ref: &Piece, times: i8) -> Vec<i8> {
      let mut piece_list = Vec::<i8>::new();
      let color = self.faces[face/* -1 */].center[0].getcolor().colorNum[0];
      assert_eq!(face+1,color);
        for _ in 0..times  {
          if piece_list.len() >= 5 {
              break;
          }
          let result: bool = (*piece_ref).matchesColor(color);
          if result {
              piece_list.push((*piece_ref).defaultPieceNum as i8);
          }
      }
      return piece_list;
    }
  }
 
}
