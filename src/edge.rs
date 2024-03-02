//2024 megaminx-rs edge.rs , by genr8eofl - LICENSED APGL3
pub mod edge {
  use crate::piece::piece::EdgeCornerInit;
  use crate::piece::piece::Piece;
  use crate::piece::piece::PieceData;
use crate::piece::piece::VertexPositionColor;
  use crate::piece::piece::VERTEXZERO;
  //Edge functions
  pub trait Edge : EdgeCornerInit {
    fn render(&self) -> Vec<VertexPositionColor>;
    fn render_lines(&self, n: i8) -> Vec<VertexPositionColor>;
    fn flip_twice(&mut self);    
    fn flip(&mut self);
    fn getdata(&self) -> &PieceData;
  }
  impl Edge for Piece {
    fn getdata(&self) -> &PieceData {
        &self.data
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
        //println!("DEBUG Edge[{}] self.vertex {:?}", self.defaultPieceNum, self.vertex);
    }
    fn render_lines(&self, n: i8) -> Vec<VertexPositionColor> {
        match n {
            0 => { 
                vec![
                    VertexPositionColor { position: self.vertex[0], color: VERTEXZERO }, //black line
                    VertexPositionColor { position: self.vertex[1], color: VERTEXZERO },
                    VertexPositionColor { position: self.vertex[2], color: VERTEXZERO },
                    VertexPositionColor { position: self.vertex[3], color: VERTEXZERO }, //loop1
                ]
            },  //(Intersection Line is at 2/3)
            1 => {
                vec![
                    VertexPositionColor { position: self.vertex[2], color: VERTEXZERO }, //black line
                    VertexPositionColor { position: self.vertex[3], color: VERTEXZERO },
                    VertexPositionColor { position: self.vertex[4], color: VERTEXZERO },
                    VertexPositionColor { position: self.vertex[5], color: VERTEXZERO }, //loop2
                ]
            },
            _=> { Vec::<VertexPositionColor>::new() },
        }
    }
    //Flip - Changes colors. rotate/switches colors for current piece
    fn flip(&mut self) {
        self.data.color.colorRGB[0].rotate_left(3);
        self.data.color.colorNum.rotate_left(1);
        self.data.color.colorName.rotate_left(1);
//        let is_corner: bool = self.numSides == 3;
        if self.data.flipStatus == 0 {
            self.data.flipStatus += 1;
        }
        else {
            self.data.flipStatus = 0;
        }
    }
    //Does two flips. Thats it.
    fn flip_twice(&mut self) {
        self.flip();
        self.flip();
    }
  }
}
