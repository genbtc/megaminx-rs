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
        faces: Default::default(),
        centers: vec![Box::<Piece>::new(Default::default())],
        corners: vec![Box::<Piece>::new(Default::default())],
        edges:   vec![Box::<Piece>::new(Default::default())],
        g_current_face: Box::<Face>::new(Default::default()),
      }
    }
  }
  
  impl Megaminx {
    /**
     * \brief Megaminx main simple constructor for init.
     * \note   Setup, Solve Puzzle (aka Reset), Render
     */
    pub fn init_reset(&mut self) {
        println!("initing Megaminx!");
        //(re)/initialize Struct w/ defaults
        self.g_current_face = Box::<Face>::new(Default::default());
        self.rotating_face_index = 0;
        self.is_rotating = false;
        self.invisible = false;
        //   = note: the following trait bounds were not satisfied:
        // `Box<Face>: Clone`
        //`Box<(dyn center::center::Center + 'static)>: Clone`
        //self.faces.resize(NUM_FACES);
        //self.centers.resize(NUM_FACES);
        //self.corners.resize(NUM_CORNERS);
        //self.edges.resize(NUM_EDGES);
        //self.faces.push(Box::<Face>::new());
        //MegaminxInitFunctions
        self.init_edge_pieces();
        self.init_corner_pieces();
        self.init_center_pieces();
        self.init_face_pieces();
        //self.render_all_pieces();
    }
  }

  //Megaminx Init Pieces Setup
  pub trait MegaminxInitFunctions {
    fn init_edge_pieces(&mut self);
    fn init_corner_pieces(&mut self);
    fn init_center_pieces(&mut self);
    fn init_face_pieces(&mut self);
    fn render_all_pieces(&mut self);
  }
  impl MegaminxInitFunctions for Megaminx {

    /**
     * \brief Init the Edge pieces.
     * \note   numEdges = 30
     */
    fn init_edge_pieces(&mut self) {
        //store a list of the basic starting Edge vertexes
        for i in 0..NUM_EDGES {
            println!("initing edge: {}", i);
            let mut edgepiece: Piece = Piece::new(i as i8);
            let edge_vertex_list = *edgepiece.edgeInit();
            self.edges.push(Box::new(edgepiece));
            self.edges[i].init_data(i as i8, edge_vertex_list);
        }
    }

    /**
     * \brief Init the Corner pieces.
     * \note   numCorners = 20
     */
    fn init_corner_pieces(&mut self) {
        //store a list of the basic starting Corner vertexes
        for i in 0..NUM_CORNERS {
            println!("initing corner: {}", i);
            let mut cornerpiece: Piece = Piece::new(i as i8);
            let corner_vertex_list = *cornerpiece.cornerInit();
            self.corners.push(Box::new(cornerpiece));
            self.corners[i].init_data(i as i8, corner_vertex_list);
        }
    }

    /** \brief - Init the Centers, attach them to Faces. */
    fn init_center_pieces(&mut self) {
        for i in 0..NUM_FACES {
          println!("initing center: {}", i);
            let mut centerpiece: Piece = Piece::new(i as i8);
            let center_vertex_list = *centerpiece.centerInit();
            self.centers.push(Box::new(centerpiece));
            self.centers[i].init(i as i8);
        }        
    }
    /**
     * \brief Init the Faces and All Pieces.
     *         Set up the Axes of the faces,
     *          and attach the Edge and Corner pieces to the Faces.
     */
    fn init_face_pieces(&mut self) {
        for i in 0..NUM_FACES {
            println!("initing face: {}", i);
            let mut facepiece: Face = Face::new(i as i8);
            facepiece.init(i as i8);
            facepiece.create_axis(i as i32, i);
            facepiece.attach_center(&self.centers[i]);//, *center_vertex_list);
            facepiece.attach_edge_pieces(&self.edges[i]);
            facepiece.attach_corner_pieces(&self.corners[i]);
            self.faces.push(Box::new(facepiece));
        }
    }

    /**                                                                                                                                     
     * \brief Default Render ALL the pieces (unconditionally)                                                                               
     */
    fn render_all_pieces(&mut self) {
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
