//2024 megaminx-rs megaminx.rs , by genr8eofl - LICENSED APGL3
pub mod megaminx {
  use crate::face::face::FaceFunctions;
  use crate::face::face::Face;
  use crate::piece::piece::Piece;
  use crate::piece::piece::PieceInit;
  use crate::center::center::Center;
  use crate::edge::edge::Edge;
  use crate::corner::corner::Corner;
  use crate::piece::piece::Vertex3;
  use crate::piece_color::PieceColor::{NUM_EDGES,NUM_CORNERS,NUM_FACES};
  use crate::piece::piece::PieceColor;
  use std::collections::VecDeque;

  pub struct Megaminx { 
    pub invisible: bool,
    pub is_rotating: bool,
        rotating_face_index: i8,
    pub faces: Vec<Box<Face>>,
    pub centers: Vec<Box<dyn Center>>,
    pub corners: Vec<Box<Piece>>,
    pub edges: Vec<Box<Piece>>,
    pub g_current_face: Box<Face>,
    pub rotate_queue: VecDeque<usize>
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
        //MegaminxInitFunctions
        self.init_edge_pieces();
        self.init_corner_pieces();
        self.init_center_pieces();
        self.init_face_pieces();
        //self._render_all_pieces();
    }
    /**                                                                                                                                     
     * \brief Default Render ALL the pieces (unconditionally)                                                                               
     */
    fn _render_all_pieces(&mut self) {
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
        if !self.rotate_queue.is_empty() {
            let Some(&op) = self.rotate_queue.front();
            self.rotating_face_index = op.num;    //this is set only here
            assert!(self.rotating_face_index != -1);   //ensure safety
            self.is_rotating = true;
            self.faces[self.rotating_face_index as usize].rotate(op.dir);
//|                                                       ^^^^^^ method not found in `Box<Face>
        }

        // Full Re-render all if non-rotating or early startup
        //Conditionally Process all pieces that are NOT part of a rotating face.
        for i in 0..NUM_FACES {
          let center = self.centers;
          if center[i] != self.faces[self.rotating_face_index as usize].center[0] {
//error[E0369]: binary operation `!=` cannot be applied to type `Box<dyn center::center::Center>`
//|              --------- ^^ ------------------------------------------------------- Box<dyn center::center::Center>
//|              Box<dyn center::center::Center>
            Center::render(&mut center[i]);
            //center.render(); 
          }
        }
        let k: usize = 0;
        for i in 0..NUM_EDGES {
          let edge = self.edges;
          if edge[i] != self.faces[self.rotating_face_index as usize].edge[k] {
            //edge[i].render();
            Edge::render(&edge[i]);
//|         ------------ ^^^^^^^^ the trait `Edge` is not implemented for `Box<Piece>`
          } else {
            k += 1;
          }
        }
        let k: usize = 0;
        for i in 0..NUM_CORNERS {
          let corner = self.corners;
          if corner[i] != self.faces[self.rotating_face_index as usize].corner[k] {
            //corner[i].render();
            Corner::render(&corner[i]);
//|         -------------- ^^^^^^^^^^ the trait `Corner` is not implemented for `Box<Piece>`
          } else {
            k += 1;
          }
        }
            //_edge.render();
            //error[E0034]: multiple applicable items in scope
              //Edge::render(&mut *edge);
    //          |           ------------ ^^^^^^^^^^ the trait `Edge` is not implemented for `Box<Piece>`
    //          |           required by a bound introduced by this call          
    // and
    //          Corner::render(&mut *corner);

        // (starts up with rotating_face_index is -1)
        //rest of function can be skipped to avoid array[-1] error
        if self.rotating_face_index == -1 {
            return;
        }

        //call .RENDER() and find out if successful
        let didrender = self.faces[self.rotating_face_index as usize].render();
        if didrender.len() > 0 && self.is_rotating {
            //If yes, then Finish the Rotation & advance the Queue
            self.rotate_queue.pop_front();
        }
    }

  }

  //Megaminx Init Pieces Setup
  pub trait MegaminxInitFunctions {
    fn init_edge_pieces(&mut self);
    fn init_corner_pieces(&mut self);
    fn init_center_pieces(&mut self);
    fn init_face_pieces(&mut self);
    fn print_vector(&mut self,piece: &Piece);
  }
  impl MegaminxInitFunctions for Megaminx {

    /** \brief Init the Edge pieces. (numEdges = 30)  */
    fn init_edge_pieces(&mut self) {
        //store a list of the basic starting Edge vertexes
        for i in 0..NUM_EDGES {
            //println!("initing edge: {}", i);
            let mut edgepiece: Piece = Piece::new(i);
            let edge_vertex_list: [Vertex3;7] = *edgepiece.edgeInit();
            Edge::init_data(&mut edgepiece, i, edge_vertex_list);
            self.edges.push(Box::new(edgepiece));
            //self.print_vector(&edgepiece);
        }
        assert_eq!(self.edges.len(), NUM_EDGES);        
    }

    /** \brief Init the Corner pieces. (numCorners = 20)  */
    fn init_corner_pieces(&mut self) {
        //store a list of the basic starting Corner vertexes
        for i in 0..NUM_CORNERS {
            //println!("initing corner: {}", i);
            let mut cornerpiece: Piece = Piece::new(i);
            let corner_vertex_list: [Vertex3;7] = *cornerpiece.cornerInit();
            Corner::init_data(&mut cornerpiece, i, corner_vertex_list);
            self.corners.push(Box::new(cornerpiece));
            //self.print_vector(&cornerpiece);
        }
        assert_eq!(self.corners.len(), NUM_CORNERS);        
    }

    /** \brief Init the Centers, attach them to Faces. (numFaces = 12) */
    fn init_center_pieces(&mut self) {
        for i in 0..NUM_FACES {
            //println!("initing center: {}", i);
            let mut centerpiece: Piece = Piece::new(i);
            let center_vertex_list: [Vertex3;7] = *centerpiece.centerInit();
            Center::init_data(&mut centerpiece, i, center_vertex_list);
            self.centers.push(Box::new(centerpiece));
            //self.print_vector(&centerpiece);
        }
        assert_eq!(self.centers.len(), NUM_FACES);        
    }

    /**
     * \brief Init the Faces and All Pieces.
     *         Set up the Axes of the faces, attach the centers, 
     *          and attach the Edge and Corner pieces to the Faces.
     */
    fn init_face_pieces(&mut self) {
        for i in 0..NUM_FACES {
            //println!("initing face: {}", i);
            let mut face: Face = Face::new(i);
            face.attach_center(&mut self.centers);
            face.attach_edge_pieces(&self.edges);
            face.attach_corner_pieces(&self.corners);
            self.faces.push(Box::new(face));
        }
        assert_eq!(self.faces.len(), NUM_FACES);        
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
  pub trait MegaminxMoveFunctions {
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
  * \param face Nth-face number (1-12)
  * \param pieceRef Takes a reference to the [0]th member of Pointer_array of (either Corner/Edge's)
  * \param times how many times to iterate over the ref'd array
  * \return Returns the list of 5 positions where the starting face's pieces have ended up at.
  * \note    NOTE: Finds pieces BEFORE they are attached to a face.
  */
  impl MegaminxFindPieces for Megaminx {
    fn find_pieces_of_face(&mut self, face: usize, piece_ref: &Piece, times: i8) -> Vec<i8> {
      let mut piece_list = Vec::<i8>::new();
      let color = 0;
      //let color = Piece::getdata((*self.faces[face - 1].center[0])).color.colorNum[0];
//      error[E0609]: no field `data` on type `Box<(dyn center::center::Center + 'static)>`
//      error[E0599]: no method named `getdata` found for mutable reference `&mut (dyn center::center::Center + 'static)` in the current scope
//
//|                                                      ^^^^^^^ method not found in `&mut dyn Center`
//      error[E0599]: no method named `getdata` found for trait object `(dyn center::center::Center + 'static)` in the current scope
//
//|                   -------------- ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `&Piece`, found `dyn Center`
//|                   arguments to this function are incorrect
      assert_eq!(face,color);
        for i in 0..times  {
          if piece_list.len() >= 5 {
              break;
          }
          let result: bool = (*piece_ref).matchesColor(color);
          if result {
              piece_list.push(i);
          }
      }
      return piece_list;
    }
  }
 
}
