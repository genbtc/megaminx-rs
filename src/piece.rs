// Megaminx-rs/piece.rs - LICENSE - AGPL3 - genr8eofl @ genBTC - for megaminx-rs (2024)
#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
pub mod piece {
// Piece data-members we can swap out all at once
struct PieceData {
    _color: [[f32; 3]; 3],
    _colorNum: [i8; 3],
    _colorName: [&'static str; 3],
    pieceNum: i8,
    flipStatus: i8,
    hotPieceMoving: bool,
}

pub struct Piece { 
	// Piece struct
    //Coords for GL vertex (up to 7, not all used) * max possible sides 3
	pub _vertex: [[f32; 3]; 7],
    //Keeps the default number in the piece. do not swap.
	pub defaultPieceNum: i8,
    //Center has 1, Edge has 2, Corner has 3
	pub numSides: i8,
	//Data Struct (can swap out)
	pub data: PieceData,
}

pub struct Piecepack {
    pub axis1: char,
    pub axis2: char,
    pub multi: i32
}

//CONSTANTS:
//default size in 3d coords for main megaminx
//let something; can't do it, const something can't do it; static something, can't do it;
//error[E0015]: cannot call non-const fn `f32::<impl f32>::acos` in constants
//error[E0015]: cannot call non-const fn `f32::<impl f32>::sin` in statics
//= note: calls in statics are limited to constant functions, tuple structs and tuple variants
//arbitrary size of dodecahedron
macro_rules! dodesize { () => {   100f32   }; }
//common geometric constants
macro_rules! pi { () => {  (-1f32).acos()  }; }           	 //3.1415927410125732
//golden ratio (phi) (also the ratio between the side length of a regular pentagon and one of its diagonals.)
macro_rules! phi { () => {  (1. + (5f32).sqrt()) / 2f32  }; }       //1.6180340051651001
macro_rules! sideangle { () => {  2. * phi!().atan()  }; }      	//2.0344439448698051
//inscribed sphere radius ( ri: f32 = a / 2 * √ ( 25 + 11 * √5 ) / 10 )
macro_rules! inssphererad { () => { dodesize!() * (10. + 22. / (5f32).sqrt()).sqrt() / 4.   }; }    //111.35163307189941
macro_rules! inscirclerad { () => { dodesize!() / ((5. - (5f32).sqrt()) / 2.).sqrt()   }; }      	// 85.065082037033278
//megaminx vertex math shortcuts
macro_rules! twofifths { () => { 2./5.  }; }
fn pim(x: f32) -> f32 {	return x*pi!()/5. }
macro_rules! edgefifth { () => { dodesize!() / pim(2.).sin()   }; }      	//105.14622122913930
macro_rules! cospim35 { () => { inscirclerad!() * pim(3.5).cos()   }; }     //-50.000004917867173
macro_rules! cospim15 { () => { inscirclerad!() * pim(1.5).cos()   }; }      //49.999998901510480
macro_rules! sinpim35 { () => { inscirclerad!() * pim(3.5).sin()   }; }      //68.819093936061520

impl Piece {
    pub fn cornerInit(&mut self) -> &f32 {
        self.numSides = 3;
        for i in 0..7  {
            self._vertex[i][2] = -inssphererad!();
        }

        self._vertex[0][0] = cospim35!() * twofifths!(); //inside corner (aka outside center)
        self._vertex[0][1] = sinpim35!() * twofifths!();

        self._vertex[1][0] = cospim35!() + edgefifth!() * twofifths!(); //corner inside edge a
        self._vertex[1][1] = sinpim35!();

        self._vertex[2][0] = cospim35!();     //outside corner
        self._vertex[2][1] = sinpim35!();

        self._vertex[3][0] = cospim15!() - edgefifth!() * twofifths!(); //corner inside edge b
        self._vertex[3][1] = sinpim35!();
        self.rotateVertex(self._vertex[3], 'z', pim(2.));

        self._vertex[4][0] = cospim15!() * twofifths!(); //brother = 0 or 6
        self._vertex[4][1] = sinpim35!() * twofifths!();
        self.rotateVertex(self._vertex[4], 'z', pim(-3.));
        self.rotateVertex(self._vertex[4], 'x', pi!() - sideangle!());
        self.rotateVertex(self._vertex[4], 'z', pim(2.));

        self._vertex[5][0] = cospim15!() - edgefifth!() * twofifths!(); //brother = 3 or 1
        self._vertex[5][1] = sinpim35!();
        self.rotateVertex(self._vertex[5], 'z', pim(-3.));
        self.rotateVertex(self._vertex[5], 'x', pi!() - sideangle!());
        self.rotateVertex(self._vertex[5], 'z', pim(2.));

        self._vertex[6][0] = cospim15!() * twofifths!(); //brother = 0 or 4
        self._vertex[6][1] = sinpim35!() * twofifths!();
        self.rotateVertex(self._vertex[6], 'z', pim(-5.));
        self.rotateVertex(self._vertex[6], 'x', pi!() - sideangle!());
        return &self._vertex[0][0];
    }
    //Creates the common starting vertexes for all pieces that are EDGES
    pub fn edgeInit(&mut self) -> &f32 {
        self.numSides = 2;
        for i in 0..6 {
            self._vertex[i][2] = -inssphererad!();
        }

        self._vertex[0][0] = cospim35!() * twofifths!();
        self._vertex[0][1] = sinpim35!() * twofifths!();

        self._vertex[1][0] = cospim15!() * twofifths!();
        self._vertex[1][1] = sinpim35!() * twofifths!();

        self._vertex[2][0] = cospim15!() - edgefifth!() * twofifths!();
        self._vertex[2][1] = sinpim35!();

        self._vertex[3][0] = cospim35!() + edgefifth!() * twofifths!();
        self._vertex[3][1] = sinpim35!();

        self._vertex[4][0] = self._vertex[1][0];
        self._vertex[4][1] = self._vertex[1][1];
        self.rotateVertex(self._vertex[4], 'z', pi!());
        self.rotateVertex(self._vertex[4], 'x', pi!() - sideangle!());

        self._vertex[5][0] = self._vertex[0][0];
        self._vertex[5][1] = self._vertex[0][1];
        self.rotateVertex(self._vertex[5], 'z', pi!());
        self.rotateVertex(self._vertex[5], 'x', pi!() - sideangle!());
        return &self._vertex[0][0];
    }
    //Creates the common starting vertexes for all pieces that are CENTERS
    pub fn centerInit(&mut self) -> &f32 {
        self.numSides = 1;
        for i in 0..5 {
            self._vertex[i][0] = inscirclerad!() * (pim(2.) * i as f32 + pim(1.5)).cos() * twofifths!();
            self._vertex[i][1] = inscirclerad!() * (pim(2.) * i as f32 + pim(1.5)).sin() * twofifths!();
            self._vertex[i][2] = -inssphererad!();
        }
        return &self._vertex[0][0];
    }    
    //Creates the common starting vertexes for all pieces that are FACES
    pub fn faceInit(&mut self) -> &f32 {
        self.numSides = 0;
        for i in 0..5 {
            //This puts it on the back face
            self._vertex[i][0] = cospim35!() + edgefifth!() * twofifths!();
            self._vertex[i][1] = -sinpim35!();
            self._vertex[i][2] = -inssphererad!();
            self.rotateVertex(self._vertex[i], 'z', pim(2.));
            self.rotateVertex(self._vertex[i], 'x', pi!() - sideangle!());
            self.rotateVertex(self._vertex[i], 'z', i as f32 * pim(2.));
        }
        return &self._vertex[0][0];
    }

	pub fn rotateVertexX(&mut self, vx: f32, vy: f32, angle: f32) {
	    let r: f32 = (vx * vx + vy * vy).sqrt();
	    let a: f32 = if vy > 0. { (vx / r).acos() } else { 2. * pi!() - (vx / r).acos() };
	    a += angle;
	    vx = r * a.cos();
	    vy = r * a.sin();
	}

	pub fn rotateVertex(&mut self, vertex: [f32; 3], axis: char, angle: f32) {
		match axis {
			'x' => self.rotateVertexX(vertex[1], vertex[2], angle),
		    'y' => self.rotateVertexX(vertex[0], vertex[2], angle),
		    'z' => self.rotateVertexX(vertex[0], vertex[1], angle),
		    _ => println!("Axis must be in x, y, z")
	    }
	}

	//main transform: used in almost every other algo
	pub fn axis1multi(&mut self, target: [f32; 3], pack: Piecepack) {
	    self.rotateVertex(target, pack.axis1, pim(pack.multi as f32));
	}
	pub fn CenterSide1(&mut self, target: [f32; 3], pack: Piecepack) {
	    self.rotateVertex(target, pack.axis1, pim(1.));
	    self.rotateVertex(target, pack.axis2, pi!() - sideangle!());
	    self.axis1multi(target, pack);
	}
	pub fn CenterCenter(&mut self, target: [f32; 3], pack: Piecepack) {
	    self.rotateVertex(target, pack.axis1, pi!());
	}
	pub fn CenterSide2(&mut self, target: [f32; 3], pack: Piecepack) {
	    self.CenterCenter(target, pack);
	    self.rotateVertex(target, pack.axis2, pi!() - sideangle!());
	    self.rotateVertex(target, 'z', pim(pack.multi as f32));
	    //This is always z, because axis1/2 are usually y/x and
	    //is re-used by face, where it is Z.
	}
	pub fn CornerGrp3(&mut self, target: [f32; 3], pack: Piecepack) {
	    self.CenterSide1(target, pack);
	    self.rotateVertex(target, pack.axis2, pi!());
	}
	pub fn CornerGrp4(&mut self, target: [f32; 3], pack: Piecepack) {
	    self.CenterCenter(target, pack);
	    self.rotateVertex(target, pack.axis2, pim(pack.multi as f32));
	}
	pub fn EdgeGrp2(&mut self, target: [f32; 3], pack: Piecepack) {
	    self.rotateVertex(target, pack.axis1, pim(3.));
	    self.rotateVertex(target, pack.axis2, pi!() - sideangle!());
	    self.axis1multi(target, pack);
	}
	pub fn EdgeGrp3(&mut self, target: [f32; 3], pack: Piecepack) {
	    self.rotateVertex(target, pack.axis1, pim(6.));
	    self.EdgeGrp2(target, pack);
	}
	pub fn EdgeGrp4(&mut self, target: [f32; 3], pack: Piecepack) {
	    self.rotateVertex(target, pack.axis1, pim(8.));
	    self.EdgeGrp2(target, pack);
	}
	pub fn EdgeGrp5(&mut self, target: [f32; 3], pack: Piecepack) {
	    pack.multi += 1;
	    self.rotateVertex(target, pack.axis1, pim(2.));
	    self.rotateVertex(target, pack.axis2, sideangle!());
	    self.axis1multi(target, pack);
	}
	pub fn EdgeGrp6(&mut self, target: [f32; 3], pack: Piecepack) {
	    self.rotateVertex(target, pack.axis2, pi!());
	    self.axis1multi(target, pack);
	}
  }
}
