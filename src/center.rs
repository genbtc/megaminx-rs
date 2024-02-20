//2024 megaminx-rs center.rs , by genr8eofl - LICENSED APGL3
pub mod center {
  use crate::piece::piece::Piecepack;
  use crate::piece::piece::Piece;
  use crate::piece::piece::PieceMath;
  pub trait Center {
	  fn init(&mut self, piecenum: i8);
	  fn create_axis(&mut self, piecenum: i32, index: usize);
	  fn render(&self);
  }
  impl Center for Piece {
	/**
	 * \brief Inits a Center piece
	 * \note  (calls createAxis and initColor)
	 * \param n the number of the piece
	 */
	fn init(&mut self, piecenum: i8) {
		for i in 0..5  {
	        self.create_axis(piecenum as i32, i);
		}
	    //TODO: self.initColor(n + 1);	//from Piece
	}

	/**
	 * \brief createAxis sets up the x,y,z Axes that the Center pieces ride on
	 * \note (called by init on startup)
	 * \param n - the number of the piece
	 * \param *target - the pre-existing Vertex Array
	 */
	fn create_axis(&mut self, piecenum: i32, index: usize) {
	    match piecenum + 1 {
	    2..=6 => {
	        self.CenterSide1(index, Piecepack { axis1: 'z', axis2: 'x', multi: ((piecenum-1) * 2 % 10) }); },
	    7 => {
	        self.CenterCenter(index, Piecepack { axis1: 'x', axis2: '0', multi: 0 }); },
	    8..=12 => {
	        self.CenterSide2(index, Piecepack { axis1: 'y', axis2: 'x', multi: ((piecenum-2) * 2 % 10) }); },
	    _ => println!("Must be within 1-12"),
	    }
	}

	/**
	 * \brief Render Center Node (CONST)
	 */
	fn render(&self) {
/*
	    //Make a solid color pentagon
	    glColor3dv(data._color[0]);
	    makeGLpolygon(_vertex, 1.0, 5);
	    //Make a black line border around pentagon
	    glColor3d(0, 0, 0); //RGB(0,0,0) == Black
	    glLineWidth(3);     //border thickness
	    makeGLpolygon(_vertex, 1.005, 5);   //1 + 0.005 (to account for the border)
	    //label the piece with a number(string), as a floating tag on piece
	    if (openGLGlobalState.textGLCenterLabels)
	        utDrawText3DFont(_vertex[4][0]*1.1,_vertex[4][1]*1.1,_vertex[4][2]*1.1, GLUT_BITMAP_HELVETICA_18, data._colorName[0]);
	                                                    // 1.1x spaces it out
*/
	}
  }
}
