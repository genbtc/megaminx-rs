//2024 megaminx-rs edge.rs , by genr8eofl - LICENSED APGL3
pub mod edge {
  use crate::megaminx::megaminx::EdgeCornerMath;
  use crate::piece::piece::EdgeCornerInit;
  use crate::piece::piece::PieceInit;
  use crate::piece::piece::Piece;
  use crate::piece::piece::PieceColor;
  use crate::piece_color::PieceColor::G_EDGEPIECESCOLORS;
  use crate::piece::piece::VertexPositionColor;
  use crate::piece::piece::Vertex3;
  //Edge functions
  pub trait Edge : EdgeCornerInit {
    fn new(&mut self);
    fn init(&mut self, piecenum: usize, do_axes: bool);
    fn init_data(&mut self, piecenum: usize, corner_vertex_base: [Vertex3; 7]);
  }
  pub trait EdgeMath : EdgeCornerMath {
    fn create_axis(&mut self, piecenum: usize, index: usize);
    fn render(&self) -> Vec<VertexPositionColor>;
    fn render_lines(&self) -> Vec<VertexPositionColor>;
  }
  impl Edge for Piece {
    fn new(&mut self) {
        self.edgeInit();
        Edge::init(self, self.defaultPieceNum, true);
    }
    /**
     * \brief Inits a Edge piece
     * \note  (calls createAxis and initColor)
     * \param n the number of the Edge piece (piecenum)
     * \param doAxes True by default. First Time Initialization Only
     */
    fn init(&mut self, piecenum: usize, do_axes: bool) {
        if do_axes {
            for i in 0..6 {
                self.create_axis(piecenum, i);
            }
        }
        self.initColor(G_EDGEPIECESCOLORS[piecenum], false);
        self.data.pieceNum = piecenum;
        self.defaultPieceNum = piecenum;
    }
    /**
     * \brief Inits the piece with a pre-existing Vertex Array
     * \param edgeVertexBase the starting points to be memcpy'ed in
     */
    fn init_data(&mut self, piecenum: usize, edge_vertex_base: [Vertex3; 7]) {
        self.vertex = edge_vertex_base;
        Edge::init(self, piecenum, true);
    }
  }
}
