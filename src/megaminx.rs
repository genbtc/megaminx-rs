//2024 megaminx-rs megaminx.rs , by genr8eofl - LICENSED APGL3
#![allow(unused_variables)]
#![allow(dead_code)]
mod megaminx {
//  use crate::center::center::Center;
  use crate::face::face::Face;

  static NUM_FACES: i8 = 12;
  static NUM_CORNERS: i8 = 20;
  static NUM_EDGES: i8 = 30;

  pub struct Megaminx<'a> { 
    pub invisible: bool,
    pub is_rotating: bool,
        rotating_face_index: i8,
    pub g_current_face: &'a Face,
  }
  impl Megaminx<'_> {

    /**
     * \brief Megaminx main simple constructor for init.
     * \note   Setup, Solve Puzzle (aka Reset), Render
     */
    fn new(&mut self) {
//        self.g_currentFace = NULL;
        self.rotating_face_index = 0;
        self.is_rotating = false;
        self.invisible = false;
        self.init_edge_pieces();
        self.init_corner_pieces();
        self.init_face_pieces();
//        self.renderAllPieces();
    }

    /**
     * \brief Init the Edge pieces.
     * \note   numEdges = 30
     */
    fn init_edge_pieces(&self) {
/*        //store a list of the basic starting Edge vertexes
        double* edgeVertexList = edges[0].edgeInit();
        for i in 0..numEdges {
            edges[i].init(i, edgeVertexList);
        }*/
    }

    /**
     * \brief Init the Corner pieces.
     * \note   numCorners = 20
     */
    fn init_corner_pieces(&self) {
/*        //store a list of the basic starting Corner vertexes
        double* cornerVertexList = corners[0].cornerInit();
        for i in 0..numCorners {
            corners[i].init(i, cornerVertexList);
        }*/
    }

    /**
     * \brief Init the Faces and All Pieces.
     * \note - Init the Centers, attach them to Faces.
     *         Set up the Axes of the faces,
     *          and attach the Edge and Corner pieces to the Faces.
     */
    fn init_face_pieces(&self) {
//        let centerVertexList = faces[0].faceInit();
/*        for i in 0..numFaces {
            centers[i].init(i);
            faces[i].attachCenter(centers + i, centerVertexList);
            faces[i].initAxis(i);
            faces[i].attachEdgePieces(this, edges[0]);
            faces[i].attachCornerPieces(this, corners[0]); 
        }*/
    }
  }
}
