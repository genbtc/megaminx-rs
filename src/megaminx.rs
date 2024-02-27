//2024 megaminx-rs megaminx.rs , by genr8eofl - LICENSED APGL3
pub mod megaminx {
  use crate::face::face::FaceFunctions;
  use crate::face::face::Face;
  use crate::piece::piece::Piece;
  use crate::piece::piece::PieceMath;
  use crate::center::center::Center;
  use crate::edge::edge::Edge;
  use crate::corner::corner::Corner;
  use crate::piece::piece::Vertex3;

  const NUM_FACES: usize = 12;
  const NUM_CORNERS: usize = 20;
  const NUM_EDGES: usize = 30;

  pub struct Megaminx { 
    pub invisible: bool,
    pub is_rotating: bool,
        rotating_face_index: i8,
    pub faces: Vec<Box<Face>>,
        centers: Vec<Box<dyn Center>>,
        corners: Vec<Box<dyn Corner>>,
        edges: Vec<Box<dyn Edge>>,
    pub g_current_face: Box<Face>,
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
      for center in &mut self.centers {
          center.render();
      }
      for edge in &mut self.edges {
          edge.render();
      }
      for corner in &mut self.corners {
          corner.render();
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
            println!("initing edge: {}", i);
            let mut edgepiece: Piece = Piece::new(i);
            let edge_vertex_list: [Vertex3;7] = *edgepiece.edgeInit();
            Edge::init_data(&mut edgepiece, i, edge_vertex_list);
            self.edges.push(Box::new(edgepiece));
            self.print_vector(&edgepiece);
        }
        assert_eq!(self.edges.len(), NUM_EDGES);        
    }

    /** \brief Init the Corner pieces. (numCorners = 20)  */
    fn init_corner_pieces(&mut self) {
        //store a list of the basic starting Corner vertexes
        for i in 0..NUM_CORNERS {
            println!("initing corner: {}", i);
            let mut cornerpiece: Piece = Piece::new(i);
            let corner_vertex_list: [Vertex3;7] = *cornerpiece.cornerInit();
            Corner::init_data(&mut cornerpiece, i, corner_vertex_list);
            self.corners.push(Box::new(cornerpiece));
            self.print_vector(&cornerpiece);
        }
        assert_eq!(self.corners.len(), NUM_CORNERS);        
    }

    /** \brief Init the Centers, attach them to Faces. (numFaces = 12) */
    fn init_center_pieces(&mut self) {
        for i in 0..NUM_FACES {
            println!("initing center: {}", i);
            let mut centerpiece: Piece = Piece::new(i);
            let _center_vertex_list: [Vertex3;7] = *centerpiece.centerInit();
            Center::init(&mut centerpiece, i);//, center_vertex_list);
            self.centers.push(Box::new(centerpiece));
            self.print_vector(&centerpiece);
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
            let mut facepiece: Face = Face::new(i);
            facepiece.attach_center(&self.centers);//, *center_vertex_list);
            facepiece.attach_edge_pieces(&self.edges);
            facepiece.attach_corner_pieces(&self.corners);
            self.faces.push(Box::new(facepiece));
        }
        assert_eq!(self.faces.len(), NUM_FACES);        
    }
    fn print_vector(&mut self, piece: &Piece) {
      //Array Print
      print!("{}Piece _ Vertex Array: [ ", piece.defaultPieceNum);
      for i in 0..5 {
        print!("[ ");
        for j in 0..3 {
          print!("{}", piece._vertex[i][j].to_string());
          if j < piece._vertex[i].len() - 1  { print!(", "); }
        }
        if i < piece._vertex.len() - 1  { print!(" ], "); }
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
}
