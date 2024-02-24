//2024 megaminx-rs edge.rs , by genr8eofl - LICENSED APGL3
pub mod edge {
  use crate::piece::piece::Piecepack;
  use crate::piece::piece::Piece;
  use crate::piece::piece::PieceMath;
  use crate::Vertex3; 
  //Edge functions
  pub trait Edge {
      fn init(&mut self, piecenum: i8, do_axes: bool);
      fn init_data(&mut self, piecenum: i8, edge_vertex_base: [Vertex3; 7]);
      fn create_axis(&mut self, piecenum: i32, index: usize);
      fn render(&mut self);
      fn new(&mut self);
  }
  impl Edge for Piece {
    fn new(&mut self) {
        self.edgeInit();
    }
    /**
     * \brief Inits a Edge piece
     * \note  (calls createAxis and initColor)
     * \param n the number of the Edge piece (piecenum)
     * \param doAxes True by default. First Time Initialization Only
     */
    fn init(&mut self, piecenum: i8, do_axes: bool) {
        if do_axes {
            for i in 0..6 {
                self.create_axis(piecenum as i32, i);
            }
        }
        //TODO:
        //initColor(G_EDGEPIECESCOLORS[piecenum], false);
        self.data.pieceNum = piecenum;
        self.defaultPieceNum = piecenum;
    }
    /**
     * \brief Inits the piece with a pre-existing Vertex Array
     * \param edgeVertexBase the starting points to be memcpy'ed in
     */
    fn init_data(&mut self, piecenum: i8, edge_vertex_base: [Vertex3; 7]) {
        self._vertex = edge_vertex_base;
        self.init(piecenum, false);
    }
    /**
     * \brief createAxis sets up the x,y,z Axes that the Edge pieces ride on
     * \note (called by init on startup)
     * \param n - the number of the piece (piecenum)
     * \param *target - the pre-existing Vertex Array (replaced by index into self)
     */
    fn create_axis(&mut self, piecenum: i32, index: usize) {
        let pack: Piecepack = Piecepack { axis1: 'z', axis2:'x', multi: (piecenum * 2 % 10) };
        match piecenum + 1 {
        1..=5 => {
            self.axis1multi(index, pack); },
        6..=10 => {
            self.EdgeGrp2(index, pack); },
        11..=15 => {
            self.EdgeGrp3(index, pack); },
        16..=20 => {
            self.EdgeGrp4(index, pack); },
        21..=25 => {
            self.EdgeGrp5(index, pack); },
        26..=30 => {
            self.EdgeGrp6(index, pack); },
        _ => println!("Must be within 1-30"),
        }
    }
    /**
     * \brief Render Edge Node (CONST)
     */
    fn render(&mut self) {
        todo!();
/*
    //Edge Side One - Color Fill
    glColor3dv(data._color[0]);
    glBegin(GL_POLYGON);
    for i in 0..4 {
        glVertex3dv(_vertex[i]);
    }
    glEnd();
    //Edge Side Two - Color Fill
    glColor3dv(data._color[1]);
    glBegin(GL_POLYGON);
    for i in 2..6 {
        glVertex3dv(_vertex[i]);
    }
    glEnd();
    glColor3d(0, 0, 0); //Black
    //Edge Side One - Black Border Line 0-4
    glLineWidth(3);
    glBegin(GL_LINE_LOOP);
    for i in 0..4 {
        glVertex3d(_vertex[i][0] * 1.005, _vertex[i][1] *1.005 , _vertex[i][2] *1.005);
    }
    glEnd();
    //(Intersection Line is at 2/3)
    //Edge Side Two - Black Border Line 2-6
    glBegin(GL_LINE_LOOP);
    for i in 2..6 {
        glVertex3d(_vertex[i][0] * 1.005, _vertex[i][1] *1.005 , _vertex[i][2] *1.005);
    }
    glEnd();
*/
    }
  }
}
