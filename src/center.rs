//2024 megaminx-rs center.rs , by genr8eofl - LICENSED APGL3
pub mod center {
  use crate::piece::piece::EdgeCornerInit;
  use crate::piece::piece::PieceInit;
  use crate::piece::piece::Piece;
  use crate::piece::piece::PieceColor;
  use crate::piece_color::PieceColor::ColorData;
  use crate::piece::piece::VertexPositionColor;
  use crate::piece::piece::VERTEXZERO;  
  //Center functions
  pub trait Center {
      fn start(&mut self, piecenum: usize);
      fn getnum(&self) -> usize;
      fn getcolor(&self) -> ColorData;
      fn init(&mut self, piecenum: usize);
      fn render(&mut self) -> Vec<VertexPositionColor>;
      fn render_lines(&self) -> Vec<VertexPositionColor>;
      fn barse() -> Self where Self: Sized; //so it does not apply to trait objects
      //^^^^ cannot be made into an object ...because method `barse` references the `Self` type in its return type
  }
  impl Center for Piece {
    fn getnum(&self) -> usize { 
        return self.defaultPieceNum;
    }
    fn getcolor(&self) -> ColorData {
        self.data.color
    }
    fn start(&mut self, piecenum: usize) {
        self.centerInit();
        self.init(piecenum);
    } // was in faces.rs @
    //let _center_vertex_list: [Vertex3;7] = *centerpiece.centerInit();
    //Center::init(&mut centerpiece, i);
    //Center::init_data(&mut centerpiece, i, center_vertex_list);    
    /**
     * \brief Inits a Center piece
     * \note  (calls createAxis and initColor)
     * \param n the number of the piece (piecenum)
     */
    fn init(&mut self, piecenum: usize) {
        for i in 0..5  {
            self.create_center_axis(piecenum, i);
        }
        self.initColorA(piecenum + 1);  //from Piece
    }
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
    
    fn barse() -> Self {
        todo!();
//        |        -----      ^^^^ expected `Piece`, found `()`
//        |        implicitly returns `()` as its body has no tail or `return` expression
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
