//2024 megaminx-rs corner.rs , by genr8eofl - LICENSED APGL3
pub mod corner {
  use crate::piece::piece::Piecepack;
  use crate::piece::piece::Piece;
  use crate::piece::piece::PieceMath;
  use crate::Vertex3; 
  //Corner functions
  pub trait Corner {
      fn init(&mut self, piecenum: i8, do_axes: bool);
      fn init_data(&mut self, piecenum: i8, corner_vertex_base: [Vertex3; 7]);
      fn create_axis(&mut self, piecenum: i32, index: usize);
      fn render(&self);
  }
  impl Corner for Piece {
    /**
     * \brief Inits a Corner piece
     * \note  (calls createAxis and initColor)
     * \param n the number of the Corner piece (piecenum)
     * \param doAxes True by default. First Time Initialization Only
     */
    fn init(&mut self, piecenum: i8, do_axes: bool) {
        if doAxes {
            for i in 0..7  {
                self.create_axis(piecenum as i32, i);
            }
        }
        //TODO:
        //initColor(G_CORNERPIECESCOLORS[piecenum], true);
        self.data.pieceNum = piecenum;
        self.defaultPieceNum = piecenum;
    }
    /**
     * \brief Inits the piece with a pre-existing Vertex Array
     * \param cornerVertexBase the starting points to be memcpy'ed in
     */
    fn init_data(&mut self, piecenum: i8, corner_vertex_base: [Vertex3; 7]) {
        self._vertex = cornerVertexBase;
        self.init(piecenum)
    }
    /**
     * \brief createAxis sets up the x,y,z Axes that the Corner pieces ride on
     * \note (called by init on startup)
     * \param n - the number of the piece (piecenum)
     * \param *target - the pre-existing Vertex Array (replaced by index into self)
     */
    fn create_axis(&mut self, piecenum: i32, index: usize) {
        let mut pack: Piecepack = Piecepack { axis1: 'z', axis2:'x', multi: (piecenum * 2 % 10) };
        match piecenum + 1 {
        2..=5 => {
            self.axis1multi(index, pack); },
        6..=10 => {
            self.CenterSide1(index, pack); },
        11..=15 => {
            self.CornerGrp3(index, pack); },
        16..=20 => {
            pack.axis1 = 'x';
            pack.axis2 = 'z';
            self.CornerGrp4(index, pack); },
        _ => println!("Must be within 1-20"),
        }
    }
    /**
     * \brief Render Corner Node (CONST)
     */
    fn render(&self) {
        todo!();
/*
   glColor3dv(data._color[0]);
    glBegin(GL_POLYGON);
    for i in 0..4 {
        glVertex3dv(_vertex[i]);
    }
    glEnd();
    glColor3dv(data._color[1]);
    glBegin(GL_POLYGON);
    for i in 2..6 {
        glVertex3dv(_vertex[i]);
    }
    glEnd();
    glColor3dv(data._color[2]);
    glBegin(GL_POLYGON);
    glVertex3dv(_vertex[2]);
    glVertex3dv(_vertex[5]);
    glVertex3dv(_vertex[6]);
    glVertex3dv(_vertex[1]);
    glEnd();
    glLineWidth(3);
    (data.hotPieceMoving) ?  glColor3d(.4, 1, 0) : glColor3d(0, 0, 0);
    //    makeGLpentagon(_vertex, 1.005, GL_LINE_LOOP);
    //Pentagon can be made manually in three chunks
    glBegin(GL_LINE_LOOP);
    glVertex3d(_vertex[2][0] * 1.005, _vertex[2][1] * 1.005, _vertex[2][2] * 1.005);
    glVertex3d(_vertex[1][0] * 1.005, _vertex[1][1] * 1.005, _vertex[1][2] * 1.005);
    glEnd();
    glBegin(GL_LINE_LOOP);
    glVertex3d(_vertex[2][0] * 1.005, _vertex[2][1] * 1.005, _vertex[2][2] * 1.005);
    glVertex3d(_vertex[3][0] * 1.005, _vertex[3][1] * 1.005, _vertex[3][2] * 1.005);
    glEnd();
    glBegin(GL_LINE_LOOP);
    glVertex3d(_vertex[2][0] * 1.005, _vertex[2][1] * 1.005, _vertex[2][2] * 1.005);
    glVertex3d(_vertex[5][0] * 1.005, _vertex[5][1] * 1.005, _vertex[5][2] * 1.005);
    glEnd();
*/
    }
  }
}
