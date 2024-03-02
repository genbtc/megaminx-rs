//2024 megaminx-rs corner.rs , by genr8eofl - LICENSED APGL3
pub mod corner {
  use crate::piece::piece::PieceInit;
  use crate::piece::piece::PiecePack;
  use crate::piece::piece::Piece;
  use crate::piece::piece::PieceMath;
  use crate::piece::piece::PieceColor;
  use crate::piece_color::PieceColor::G_CORNERPIECESCOLORS;
  use crate::piece::piece::Vertex3;
  use crate::piece::piece::VertexPositionColor;
  use crate::piece::piece::VERTEXZERO;
  //Corner functions
  pub trait Corner {
      fn new(&mut self);
      fn init(&mut self, piecenum: usize, do_axes: bool);
      fn init_data(&mut self, piecenum: usize, corner_vertex_base: [Vertex3; 7]);
      fn create_axis(&mut self, piecenum: usize, index: usize);
      fn render(&self) -> Vec<VertexPositionColor>;
      fn render_lines(&self) -> Vec<VertexPositionColor>;
  }
  impl Corner for Piece {
    fn new(&mut self) {
        self.cornerInit();
        self.init(self.defaultPieceNum, true);
    }
    /**
     * \brief Inits a Corner piece
     * \note  (calls createAxis and initColor)
     * \param n the number of the Corner piece (piecenum)
     * \param do_axes True by default. First Time Initialization Only
     */
    fn init(&mut self, piecenum: usize, do_axes: bool) {
        if do_axes {
            for i in 0..7  {
                self.create_axis(piecenum, i);
            }
        }
        self.initColor(G_CORNERPIECESCOLORS[piecenum], true);
        self.data.pieceNum = piecenum;
        self.defaultPieceNum = piecenum;
    }
    /**
     * \brief Inits the piece with a pre-existing Vertex Array
     * \param corner_vertex_base the starting points to be memcpy'ed in
     */
    fn init_data(&mut self, piecenum: usize, corner_vertex_base: [Vertex3; 7]) {
        self.vertex = corner_vertex_base;
        self.init(piecenum, true)
    }
    /**
     * \brief createAxis sets up the x,y,z Axes that the Corner pieces ride on
     * \note (called by init on startup)
     * \param n - the number of the piece (piecenum)
     */
    fn create_axis(&mut self, piecenum: usize, index: usize) {
        let mut pack: PiecePack = PiecePack { axis1: 'z', axis2:'x', multi: (piecenum * 2 % 10) };
        match piecenum + 1 {
        1=> { },
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
    fn render(&self) -> Vec<VertexPositionColor> {
        vec![
            VertexPositionColor { position: self.vertex[0], color: self.data.color.colorRGB[0] },
            VertexPositionColor { position: self.vertex[1], color: self.data.color.colorRGB[0] },
            VertexPositionColor { position: self.vertex[2], color: self.data.color.colorRGB[0] }, //tri1
            VertexPositionColor { position: self.vertex[0], color: self.data.color.colorRGB[0] },
            VertexPositionColor { position: self.vertex[2], color: self.data.color.colorRGB[0] },
            VertexPositionColor { position: self.vertex[3], color: self.data.color.colorRGB[0] }, //tri2
            VertexPositionColor { position: self.vertex[3], color: self.data.color.colorRGB[1] },
            VertexPositionColor { position: self.vertex[4], color: self.data.color.colorRGB[1] },
            VertexPositionColor { position: self.vertex[5], color: self.data.color.colorRGB[1] }, //tri1
            VertexPositionColor { position: self.vertex[2], color: self.data.color.colorRGB[1] },
            VertexPositionColor { position: self.vertex[5], color: self.data.color.colorRGB[1] },
            VertexPositionColor { position: self.vertex[3], color: self.data.color.colorRGB[1] }, //tri2
            VertexPositionColor { position: self.vertex[5], color: self.data.color.colorRGB[2] },
            VertexPositionColor { position: self.vertex[6], color: self.data.color.colorRGB[2] },
            VertexPositionColor { position: self.vertex[1], color: self.data.color.colorRGB[2] }, //tri1
            VertexPositionColor { position: self.vertex[5], color: self.data.color.colorRGB[2] },
            VertexPositionColor { position: self.vertex[1], color: self.data.color.colorRGB[2] },
            VertexPositionColor { position: self.vertex[2], color: self.data.color.colorRGB[2] }, //tri2
        ]
    }

    fn render_lines(&self) -> Vec<VertexPositionColor> {
        vec![
            VertexPositionColor { position: self.vertex[0], color: VERTEXZERO  },
            VertexPositionColor { position: self.vertex[1], color: VERTEXZERO  },
            VertexPositionColor { position: self.vertex[2], color: VERTEXZERO  },
            VertexPositionColor { position: self.vertex[3], color: VERTEXZERO  }, //loop1
            VertexPositionColor { position: self.vertex[2], color: VERTEXZERO  },
            VertexPositionColor { position: self.vertex[3], color: VERTEXZERO  },
            VertexPositionColor { position: self.vertex[4], color: VERTEXZERO  }, 
            VertexPositionColor { position: self.vertex[5], color: VERTEXZERO  }, //Loop2
            VertexPositionColor { position: self.vertex[2], color: VERTEXZERO  },
            VertexPositionColor { position: self.vertex[5], color: VERTEXZERO  },
            VertexPositionColor { position: self.vertex[6], color: VERTEXZERO  },
            VertexPositionColor { position: self.vertex[1], color: VERTEXZERO  }, //loop3
        ]
    }
  }
}
