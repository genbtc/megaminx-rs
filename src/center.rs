//2024 megaminx-rs center.rs , by genr8eofl - LICENSED APGL3
pub mod center {
  use crate::piece::piece::EdgeCornerInit;
  use crate::piece::piece::PieceInit;
  use crate::piece::piece::Piece;
  use crate::piece::piece::PieceColor;
  use crate::piece::piece::Points;
  use crate::piece_color::PieceColor::ColorData;
  use crate::piece::piece::VertexPositionColor;
  use crate::piece::piece::VERTEXZERO;  
  //Center functions
  pub trait Center {
      fn getnum(&self) -> usize;
      fn getcolor(&self) -> ColorData;
      fn getpoints(&self) -> Points;
      fn getself(&self) -> &Self where Self: Sized; //so it does not apply to trait objects. returns back a Piece ref
      fn init(&mut self, piecenum: usize);
      fn render(&mut self) -> Vec<VertexPositionColor>;
      fn render_lines(&self) -> Vec<VertexPositionColor>;
  }
  impl Center for Piece {
    fn getnum(&self) -> usize { 
        self.defaultPieceNum
    }
    fn getcolor(&self) -> ColorData {
        self.data.color
    }
    fn getpoints(&self) -> Points {
        self.points
    }
    fn getself(&self) -> &Piece {
        self
    }
    /**
     * \brief Inits a Center piece
     * \note  (calls createAxis and initColor)
     * \param n the number of the piece (piecenum)
     */
    fn init(&mut self, piecenum: usize) {
        let _center_vertex_list = self.centerInit();
        for i in 0..5  {
            self.create_center_axis(piecenum, i);
        }
        self.initColorA(piecenum + 1);  //from Piece
    }
    //Center::init_data(&mut centerpiece, i, center_vertex_list);    
    /**
     * \brief Render Center Node (CONST)(mut for face)
     */
    fn render(&mut self) -> Vec<VertexPositionColor> {
        //println!("DEBUG center[{}] self.vertex {:?}", self.defaultPieceNum, self.vertex);
        vec![
            VertexPositionColor { position: self.vertex[0], color: self.data.color.colorRGB[0] },
            VertexPositionColor { position: self.vertex[1], color: self.data.color.colorRGB[0] },
            VertexPositionColor { position: self.vertex[2], color: self.data.color.colorRGB[0] }, //tri1
            VertexPositionColor { position: self.vertex[0], color: self.data.color.colorRGB[0] },
            VertexPositionColor { position: self.vertex[2], color: self.data.color.colorRGB[0] },
            VertexPositionColor { position: self.vertex[3], color: self.data.color.colorRGB[0] }, //tri2
            VertexPositionColor { position: self.vertex[0], color: self.data.color.colorRGB[0] },
            VertexPositionColor { position: self.vertex[3], color: self.data.color.colorRGB[0] },
            VertexPositionColor { position: self.vertex[4], color: self.data.color.colorRGB[0] }, //tri3
        ]
    }
    fn render_lines(&self) -> Vec<VertexPositionColor> {
        let mut returnvec = vec![];
        for l in  [ 0 , 1 , 2 , 3 , 4 , 0 ] {
            returnvec.push(VertexPositionColor { position: self.vertex[l], color: VERTEXZERO });
        }
        return returnvec;
    }
    
    /* 
    glLineWidth(3);     //border thickness
    makeGLpolygon(_vertex, 1.005, 5);   //1 + 0.005 (to account for the border)
    //label the piece with a number(string), as a floating tag on piece
    if (openGLGlobalState.textGLCenterLabels)
        utDrawText3DFont(_vertex[4][0]*1.1,_vertex[4][1]*1.1,_vertex[4][2]*1.1, GLUT_BITMAP_HELVETICA_18, data._colorName[0]);
                                                    // 1.1x spaces it out     */
  }
}
