//2024 megaminx-rs center.rs , by genr8eofl - LICENSED APGL3
pub mod center {
  use crate::piece::piece::PieceInit;
  use crate::piece::piece::PiecePack;
  use crate::piece::piece::Piece;
  use crate::piece::piece::PieceMath;
  use crate::piece::piece::PieceColor;
  use crate::piece::piece::VertexPositionColor;
  use crate::piece::piece::VERTEXZERO;
  //Center functions
  pub trait Center {
      fn newa(&mut self);
      fn init(&mut self, piecenum: usize);
      //fn init_data(&mut self, piecenum: usize, center_vertex_base: [Vertex3; 7]);
      fn create_axis(&mut self, piecenum: usize, index: usize);
      fn render(&mut self) -> Vec<VertexPositionColor>;
      fn render_lines(&self) -> Vec<VertexPositionColor>;
  }
  impl Center for Piece {
    fn newa(&mut self) {
        self.centerInit();
        self.init(self.defaultPieceNum);
    }    
    /**
     * \brief Inits a Center piece
     * \note  (calls createAxis and initColor)
     * \param n the number of the piece (piecenum)
     */
    fn init(&mut self, piecenum: usize) {
        for i in 0..5  {
            self.create_axis(piecenum, i);
        }
        self.initColorA(piecenum + 1);  //from Piece
    }
    // /**
    //  * \brief Inits the piece with a pre-existing Vertex Array
    //  * \param centerVertexBase the starting points to be memcpy'ed in
    //  */
    // fn init_data(&mut self, _piecenum: usize, _center_vertex_base: [Vertex3; 7]) {
    //     //COMMENTED OUT DELIBERATELY BECAUSE PIECE INIT WAS GETTING CORRUPTED WITH A SECOND INIT
    //     //self.vertex = center_vertex_base;
    //     //self.init(piecenum);
    // }    

    /**
     * \brief createAxis sets up the x,y,z Axes that the Center pieces ride on
     * \note (called by init on startup)
     * \param n - the number of the piece (piecenum)
     */
    fn create_axis(&mut self, piecenum: usize, index: usize) {
        match piecenum + 1 {
        2..=6 => {
            self.CenterSide1(index, PiecePack { axis1: 'z', axis2: 'x', multi: ((piecenum-1) * 2 % 10) }); },
        7 => {
            self.CenterCenter(index, PiecePack { axis1: 'x', axis2: '0', multi: 0 }); },
        8..=12 => {
            self.CenterSide2(index, PiecePack { axis1: 'y', axis2: 'x', multi: ((piecenum-2) * 2 % 10) }); },
        1 => {}, 
        _ => println!("Must be within 1-12"),
        }
    }

    /**
     * \brief Render Center Node (CONST)(mut why?=face said)
     */
    fn render(&mut self) -> Vec<VertexPositionColor> {
         let tri1: (usize, usize, usize) = ( 0 , 1 , 2 );    let tri2: (usize, usize, usize) = ( 0 , 2 , 3 ); let tri3: (usize, usize, usize) = ( 0 , 3 , 4 );
         return vec![
            VertexPositionColor { position: self.vertex[0], color: self.data.color.colorRGB[0] },
            VertexPositionColor { position: self.vertex[1], color: self.data.color.colorRGB[0] },
            VertexPositionColor { position: self.vertex[2], color: self.data.color.colorRGB[0] }, //tri1
            VertexPositionColor { position: self.vertex[0], color: self.data.color.colorRGB[0] },
            VertexPositionColor { position: self.vertex[2], color: self.data.color.colorRGB[0] },
            VertexPositionColor { position: self.vertex[3], color: self.data.color.colorRGB[0] }, //tri2
            VertexPositionColor { position: self.vertex[0], color: self.data.color.colorRGB[0] },
            VertexPositionColor { position: self.vertex[3], color: self.data.color.colorRGB[0] },
            VertexPositionColor { position: self.vertex[4], color: self.data.color.colorRGB[0] }, //tri3
        ];
    }

    //     let tri_array: [(usize, usize, usize); 3] = [tri1,tri2,tri3];
    //     let mut returnvec = Vec::new();
    //     for item in &tri_array {
    //         returnvec.push(VertexPositionColor { position: self.vertex[item.0], color: self.data.color.colorRGB[0] });
    //         returnvec.push(VertexPositionColor { position: self.vertex[item.1], color: self.data.color.colorRGB[0] });
    //         returnvec.push(VertexPositionColor { position: self.vertex[item.2], color: self.data.color.colorRGB[0] });
    //     }
    //     return returnvec;
    //     //println!("DEBUG center[{}] self.vertex {:?}", self.defaultPieceNum, self.vertex);
    // }
    fn render_lines(&self) -> Vec<VertexPositionColor> {
        let loop1 = [ 0 , 1 , 2 , 3 , 4 , 0 ];
        let mut returnvec = Vec::new();
        for l in loop1 {
            returnvec.push(VertexPositionColor { position: self.vertex[l], color: VERTEXZERO });
        }
        return returnvec;
    }
    /*
    //Make a solid color pentagon
    makeGLpolygon(_vertex, 1.005, 5);   //1 + 0.005 (to account for the border)
    //label the piece with a number(string), as a floating tag on piece
    if (openGLGlobalState.textGLCenterLabels)
        utDrawText3DFont(_vertex[4][0]*1.1,_vertex[4][1]*1.1,_vertex[4][2]*1.1, GLUT_BITMAP_HELVETICA_18, data._colorName[0]);
                                                    // 1.1x spaces it out
    */
  }
}
