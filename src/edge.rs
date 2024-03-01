//2024 megaminx-rs edge.rs , by genr8eofl - LICENSED APGL3
pub mod edge {
  use crate::piece::piece::PieceInit;
  use crate::piece::piece::PiecePack;
  use crate::piece::piece::Piece;
  use crate::piece::piece::PieceMath;
  use crate::piece::piece::PieceColor;
  use crate::piece_color::PieceColor::G_EDGEPIECESCOLORS;
  use crate::Vertex3;
  use crate::piece::piece::VertexPositionColor;
  use crate::piece::piece::VERTEXZERO;
  //Edge functions
  pub trait Edge {
      fn new(&mut self);
      fn init(&mut self, piecenum: usize, do_axes: bool);
      fn init_data(&mut self, piecenum: usize, edge_vertex_base: [Vertex3; 7]);
      fn create_axis(&mut self, piecenum: usize, index: usize);
      fn render(&self) -> Vec<VertexPositionColor>;
      fn render_lines(&self, n: i8) -> Vec<VertexPositionColor>;
  }
  impl Edge for Piece {
    fn new(&mut self) {
        self.edgeInit();
        self.init(self.defaultPieceNum, true);
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
        self.init(piecenum, true);
    }
    /**
     * \brief createAxis sets up the x,y,z Axes that the Edge pieces ride on
     * \note (called by init on startup)
     * \param n - the number of the piece (piecenum)
     * \param *target - the pre-existing Vertex Array (replaced by index into self)
     */
    fn create_axis(&mut self, piecenum: usize, index: usize) {
        let pack: PiecePack = PiecePack { axis1: 'z', axis2:'x', multi: (piecenum * 2 % 10) };
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
    fn render(&self) -> Vec<VertexPositionColor> {
        vec![
            VertexPositionColor { position: self.vertex[0], color: self.data.color.colorRGB[0] },
            VertexPositionColor { position: self.vertex[1], color: self.data.color.colorRGB[0] },
            VertexPositionColor { position: self.vertex[2], color: self.data.color.colorRGB[0] }, //tri1
            VertexPositionColor { position: self.vertex[3], color: self.data.color.colorRGB[0] },
            VertexPositionColor { position: self.vertex[0], color: self.data.color.colorRGB[0] },
            VertexPositionColor { position: self.vertex[2], color: self.data.color.colorRGB[0] }, //tri2
            VertexPositionColor { position: self.vertex[2], color: self.data.color.colorRGB[1] }, 
            VertexPositionColor { position: self.vertex[3], color: self.data.color.colorRGB[1] },
            VertexPositionColor { position: self.vertex[4], color: self.data.color.colorRGB[1] }, //tri3
            VertexPositionColor { position: self.vertex[5], color: self.data.color.colorRGB[1] }, 
            VertexPositionColor { position: self.vertex[4], color: self.data.color.colorRGB[1] },
            VertexPositionColor { position: self.vertex[2], color: self.data.color.colorRGB[1] }, //tri4
        ]
    }
    fn render_lines(&self, n: i8) -> Vec<VertexPositionColor> {
        match n {
            0 => { 
                return vec![
                    VertexPositionColor { position: self.vertex[0], color: VERTEXZERO }, //black line
                    VertexPositionColor { position: self.vertex[1], color: VERTEXZERO },
                    VertexPositionColor { position: self.vertex[2], color: VERTEXZERO },
                    VertexPositionColor { position: self.vertex[3], color: VERTEXZERO }, //loop1
                ];
            },  //(Intersection Line is at 2/3)
            1 => {
                return vec![
                    VertexPositionColor { position: self.vertex[2], color: VERTEXZERO }, //black line
                    VertexPositionColor { position: self.vertex[3], color: VERTEXZERO },
                    VertexPositionColor { position: self.vertex[4], color: VERTEXZERO },
                    VertexPositionColor { position: self.vertex[5], color: VERTEXZERO }, //loop2
                ];
            },
            _=> { return Vec::<VertexPositionColor>::new() },
        }
    }
  }
}
