//2024 megaminx-rs center.rs , by genr8eofl - LICENSED APGL3
pub mod center {
  use crate::piece::piece::Piecepack;
  use crate::piece::piece::Piece;
  use crate::piece::piece::PieceMath;
//  struct Center;
  pub trait Center {
	  fn init(&mut self, n: i8);
	  fn createAxis(&mut self, n: i32, target: [f32; 3]);
	  fn render(&self);
  }
  impl Center for Piece {
	/**
	 * \brief Inits a Center piece
	 * \note  (calls createAxis and initColor)
	 * \param n the number of the piece
	 */
	fn init(&mut self, n: i8) {
		for i in 0..5  {
	        self.createAxis(n as i32, self._vertex[i]);	//error[E0616]: field `_vertex` of struct `Piece` is private
		}
	    //TODO: self.initColor(n + 1);
	}

	/**
	 * \brief createAxis sets up the x,y,z Axes that the Center pieces ride on
	 * \note (called by init on startup)
	 * \param n - the number of the piece
	 * \param *target - the pre-existing Vertex Array
	 */
	fn createAxis(&mut self, n: i32, target: [f32; 3]) {
	    let pack: Piecepack;
	    match n + 1 {
	    2..=6 => {
	        pack = Piecepack { axis1: 'z', axis2: 'x', multi: ((n-1) * 2 % 10) };
	        self.CenterSide1(target, pack); },
	    7 => {
	        pack = Piecepack { axis1: 'x', axis2: '0', multi: 0 };
	        self.CenterCenter(target, pack); },
	    8..=12 => {
	        pack = Piecepack { axis1: 'y', axis2:'x', multi: ((n-2) * 2 % 10) };
	        self.CenterSide2(target, pack); },
	    _ => println!("Must be within 1-12"),
	    }
	}

	/**
	 * \brief Render Center Node (CONST)
	 */
	fn render(&self)	{
/*
	    //Make a solid color pentagon
	    glColor3dv(data._color[0]);
	    makeGLpolygon(_vertex, 1.0, 5);
	    //Make a black line border around pentagon
	    glColor3d(0, 0, 0); //RGB(0,0,0) == Black
	//    glLineWidth(3);     //border thickness
	//    makeGLpentagon(_vertex, 1.005, 3);   //1 + 0.005 (to account for the border)
	    //label the piece with a number(string), as a floating tag on piece
	    if (openGLGlobalState.textGLCenterLabels)
	        utDrawText3DFont(_vertex[4][0]*1.1,_vertex[4][1]*1.1,_vertex[4][2]*1.1, GLUT_BITMAP_HELVETICA_18, data._colorName[0]);
	                                                    // 1.1x spaces it out
	    //TODO: Crude coords, aesthetics of text numbers is suboptimal... Option: configure Disable text shown
*/
	}
  }
}
