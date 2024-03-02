//2024 megaminx-rs corner.rs , by genr8eofl - LICENSED APGL3
pub mod corner {
  use crate::piece::piece::EdgeCornerInit;
  use crate::piece::piece::Piece;
  use crate::piece::piece::PieceColor;
  use crate::piece_color::PieceColor::G_CORNERPIECESCOLORS;
  use crate::piece::piece::VertexPositionColor;
  use crate::piece::piece::VERTEXZERO;
  use crate::piece::piece::Vertex3;
  //Corner functions
  pub trait Corner : EdgeCornerInit {
      fn init(&mut self, piecenum: usize, do_axes: bool);
      fn init_data(&mut self, piecenum: usize, corner_vertex_base: [Vertex3; 7]);
      fn render(&self) -> Vec<VertexPositionColor>;
      fn render_lines(&self) -> Vec<VertexPositionColor>;
      fn flip_twice(&mut self);    
      fn flip(&mut self);
  }
  impl Corner for Piece {
    /**
     * \brief Inits a Corner piece
     * \note  (calls createAxis and initColor)
     * \param n the number of the Corner piece (piecenum)
     * \param do_axes True by default. First Time Initialization Only
     */
    fn init(&mut self, piecenum: usize, do_axes: bool) {
        if do_axes {
            for i in 0..7  {
                self.create_corner_axis(piecenum, i);
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
        Corner::init(self, piecenum, true)
    }
    /**
     * \brief createAxis sets up the x,y,z Axes that the Corner pieces ride on
     * \note (called by init on startup)
     * \param n - the number of the piece (piecenum)
     */

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
        //println!("DEBUG Corner[{}] self.vertex {:?}", self.defaultPieceNum, self.vertex);
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
    //Flip - Changes colors. rotate/switches colors for current piece
    fn flip(&mut self) {
        self.data.color.colorRGB[0].rotate_left(3);
        self.data.color.colorNum.rotate_left(1);
        self.data.color.colorName.rotate_left(1);
        if self.data.flipStatus < 2 {
            self.data.flipStatus += 1;
        }
        else {
            self.data.flipStatus = 0;
        }
    }
    //Does two flips. Thats it.
    fn flip_twice(&mut self) {
        Corner::flip(self);
        Corner::flip(self);
    }
  }
}
