//2024 megaminx-rs center.rs , by genr8eofl - LICENSED APGL3
pub mod center {
  use crate::piece::piece::*;
  use crate::piece_color::PieceColor::ColorData;
  //Center functions
  pub trait Center {
      fn getcolor(&self) -> ColorData;
      fn init(&mut self, piecenum: usize) -> &Self where Self: Sized;
      fn render(&mut self) -> Vec<VertexPositionColor>;
      fn render_lines(&self) -> Vec<VertexPositionColor>;
  }
  impl Center for Piece {
    fn getcolor(&self) -> ColorData {
        self.data.color
    }
    /* \brief Inits a Center piece
     * \note  (calls createAxis and initColor)
     * \param n the number of the piece (piecenum)
     */
    fn init(&mut self, piecenum: usize) -> &Piece {
        let _center_vertex_list = self.centerInit();
        for i in 0..5  {
            self.create_center_axis(piecenum, i);
        }
        self.initColorA(piecenum + 1);  //from Piece
        self
    }
    /* \brief Render Center Block Node */
    fn render(&mut self) -> Vec<VertexPositionColor> {
        //println!("DEBUG center[{}] self.vertex {:?}", self.defaultPieceNum, self.vertex);
        let (a,b,c,_d) = self.points.calcRaw();
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
            VertexPositionColor { position: *a.as_array(), color: self.data.color.colorRGB[0] }, //edgeA
            VertexPositionColor { position: *b.as_array(), color: self.data.color.colorRGB[0] }, //edgeB
            VertexPositionColor { position: *c.as_array(), color: self.data.color.colorRGB[0] }, //normalC
        ]
    }
    /* \brief Render Center Lines around */
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
