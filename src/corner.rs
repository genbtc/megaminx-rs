//2024 megaminx-rs corner.rs , by genr8eofl - LICENSED APGL3
pub mod corner {
  use crate::piece::piece::*;
  use crate::piece_color::PieceColor::G_CORNERPIECESCOLORS;
  use crate::edge::edge::Edge;
  
  //Corner functions
  pub trait Corner {
      fn init(&mut self, piecenum: usize, do_axes: bool);
      fn render(&self) -> Vec<VertexPositionColor>;
      fn render_lines(&self) -> Vec<VertexPositionColor>;
      fn flip_twice(&mut self);    
      fn flip(&mut self);
  }
  impl Corner for Piece {
    /**
     * \brief Inits a Corner piece
     * \note  (calls create_axis and initColor @ piece.rs)
     * \param n the number of the Corner piece (piecenum)
     * \param do_axes True by default. First Time Initialization Only
     */
    fn init(&mut self, piecenum: usize, do_axes: bool) {
        self.new();
        if do_axes {
            for i in 0..7  {
                self.create_corner_axis(piecenum, i);
            }
        }
        self.initColor(G_CORNERPIECESCOLORS[piecenum], true);
        self.data.pieceNum = piecenum;
        self.defaultPieceNum = piecenum;
        self.points.new(self.vertex);
    }
    /**
     * \brief Render Corner Node (CONST)
     */
    fn render(&self) -> Vec<VertexPositionColor> {
        //println!("DEBUG Corner[{}] self.vertex {:?}", self.defaultPieceNum, self.vertex);
        let (a,b,c,_d) = self.points.calcRaw();
        //println!("edgeA{:?}, edgeB{:?}, normalC{:?}, dotProd={:?}", a,b,c,_d);
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
            VertexPositionColor { position: *a.as_array(), color: self.data.color.colorRGB[0] }, //edgeA
            VertexPositionColor { position: *b.as_array(), color: self.data.color.colorRGB[0] }, //edgeB
            VertexPositionColor { position: *(c*-1.0).as_array(), color: self.data.color.colorRGB[0] }, //normalC            
        ]        
    }

    fn render_lines(&self) -> Vec<VertexPositionColor> {
        let mut returnvec = Vec::new();
        returnvec.extend(Edge::render_lines(self,0));
        returnvec.extend(Edge::render_lines(self,1));
        returnvec.extend(vec![
            VertexPositionColor { position: self.vertex[2], color: VERTEXZERO  },
            VertexPositionColor { position: self.vertex[5], color: VERTEXZERO  },
            VertexPositionColor { position: self.vertex[6], color: VERTEXZERO  },
            VertexPositionColor { position: self.vertex[1], color: VERTEXZERO  }, //loop3
        ]);
        return returnvec;
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
