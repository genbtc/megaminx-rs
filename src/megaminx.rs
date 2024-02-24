//2024 megaminx-rs megaminx.rs , by genr8eofl - LICENSED APGL3
#![allow(unused_variables)]
#![allow(dead_code)]
pub mod megaminx {
  use crate::face::face::FaceFunctions;
  use crate::face::face::Face;
  use crate::piece::piece::Piece;
  use crate::piece::piece::PieceMath;
  use crate::center::center::Center;
  use crate::edge::edge::Edge;
  use crate::corner::corner::Corner;

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
  /*Initialize constructor */
  impl Megaminx {
      pub fn new() -> Self {
      Self {
        invisible: false,
        is_rotating: false,
        rotating_face_index: -1,
        faces: vec![Box::<Face>::new(Default::default())],
        centers: vec![Box::<Piece>::new(Default::default())],
        corners: vec![Box::<Piece>::new(Default::default())],
        edges: vec![Box::<Piece>::new(Default::default())],
        g_current_face: Box::<Face>::new(Default::default()),
      }
    }
  } //
  //
  impl Megaminx {
    /**
     * \brief Megaminx main simple constructor for init.
     * \note   Setup, Solve Puzzle (aka Reset), Render
     */
    pub fn init_reset(&mut self) {
        //self.g_currentFace = NULL;
        self.rotating_face_index = 0;
        self.is_rotating = false;
        self.invisible = false;
        self.init_edge_pieces();
        self.init_corner_pieces();
        self.init_face_pieces();
        self.render_all_pieces();
    }
  }

  //Piece Init Setup
  pub trait MegaminxInitFunctions {
    fn init_corner_pieces(&mut self);
    fn init_edge_pieces(&mut self);
    fn init_face_pieces(&mut self);
    fn render_all_pieces(&self);
  }
  impl MegaminxInitFunctions for Megaminx {

    /**
     * \brief Init the Edge pieces.
     * \note   numEdges = 30
     */
    fn init_edge_pieces(&mut self) {
        //store a list of the basic starting Edge vertexes
        let mut edgepiece: Piece = Piece::new(0);
        let edge_vertex_list = *edgepiece.edgeInit();
        for i in 0..NUM_EDGES {
            self.edges[i].init_data(i as i8, edge_vertex_list);
        }
    }

    /**
     * \brief Init the Corner pieces.
     * \note   numCorners = 20
     */
    fn init_corner_pieces(&mut self) {
        //store a list of the basic starting Corner vertexes
        let mut cornerpiece: Piece = Piece::new(0);
        let corner_vertex_list = *cornerpiece.cornerInit();
        for i in 0..NUM_CORNERS {
            self.corners[i].init_data(i as i8, corner_vertex_list);
        }
    }

    /**
     * \brief Init the Faces and All Pieces.
     * \note - Init the Centers, attach them to Faces.
     *         Set up the Axes of the faces,
     *          and attach the Edge and Corner pieces to the Faces.
     */
    fn init_face_pieces(&mut self) {
//        let mut centerpiece: Piece = Piece::new(0);
//        let center_vertex_list = centerpiece.centerInit();
        let mut facepiece: Piece = Piece::new(0);
        let center_vertex_list = *facepiece.faceInit();
        for i in 0..NUM_FACES {
            self.centers[i].init(i as i8);
            self.faces[i].create_axis(i as i32, 0);
            self.faces[i].attach_center();//self.centers[i], *center_vertex_list);
            self.faces[i].attach_edge_pieces(self.edges[0]);
            self.faces[i].attach_corner_pieces(self.corners[0]); 
        }
    }

    /**                                                                                                                                     
     * \brief Default Render ALL the pieces (unconditionally)                                                                               
     */
    fn render_all_pieces(&self) {
        for center in &self.centers {
            center.render();
        }
        for edge in &self.edges {
            edge.render();
        }
        for corner in &self.corners {
            corner.render();
        }
    }    
  }

  //Control Functions
  pub trait MegaminxMoveFunctions {
    //fn render_all_pieces();
    fn render();
    fn undo();
    fn undo_double();
    fn undo_quad();
    fn undo_bulk();
    fn reset_queue();
    fn scramble();
  }
}
