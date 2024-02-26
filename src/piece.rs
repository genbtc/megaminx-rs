// megaminx-rs - a rust and SDL2 version of Megaminx - previously a C++ and OpenGL Dodecahedron Cube                                                                                                                                                                          
// Megaminx-rs/piece.rs - LICENSE - AGPL3 - genr8eofl @ genBTC - for megaminx-rs (2024)
#![allow(non_snake_case)]
#![allow(dead_code)]
pub mod piece {

//Vertex 3 Position Definitions
#[derive(Copy, Clone)]
pub struct VertexPosition {
    pub position: [f32; 3],
}
use glium::implement_vertex;
implement_vertex!(VertexPosition, position);
pub use VertexPosition as Vertex;

//Regular Vertex 3 Array
pub type Vertex3 = [f32; 3];

//Default initializer data
pub const VERTEXZERO: Vertex3 = [0.0,0.0,0.0];
pub const VERTEXDATAZERO: [Vertex3; 7] = [VERTEXZERO; 7];
pub const COLORGRAY: Vertex3 = [0.5,0.5,0.5];

// Piece data-members we can swap out all at once
#[derive(Copy, Clone, Default)]
pub struct PieceData {
    pub _color: [Vertex3; 3],
    pub _colorNum: [usize; 3],
    pub _colorName: [&'static str; 3],
    pub pieceNum: usize,
    pub flipStatus: usize,
    pub hotPieceMoving: bool,
}
//Pack struct to rotateVertex
#[derive(Copy, Clone, Default)]
pub struct Piecepack {
    pub axis1: char,
    pub axis2: char,
    pub multi: usize
}
//Main Piece Object
#[derive(Copy, Clone, Default)]
pub struct Piece {
    // Piece struct
    //Coords for GL vertex (up to 7, not all used) * max possible sides 3
    pub _vertex: [Vertex3; 7],
    //Keeps the default number in the piece. do not swap.
    pub defaultPieceNum: usize,
    //Center has 1, Edge has 2, Corner has 3
    pub numSides: usize,
    //Data Struct (can swap out)
    pub data: PieceData,
}
//Initialize constructor
impl Piece {
    pub fn new(defaultPieceNum: usize) -> Self {
      Self {
        _vertex: Default::default(),
        numSides: 0,
        defaultPieceNum: defaultPieceNum,
        data: Default::default(),
      }
    }
}
//CONSTANTS:
//arbitrary size of dodecahedron - default size in 3d coords for main megaminx
macro_rules! dodesize { () => {   100f32   }; }
//common geometric constants
macro_rules! pi { () => {  (-1f32).acos()  }; }                     //3.1415927410125732
//golden ratio (phi) (also the ratio between the side length of a regular pentagon and one of its diagonals.)
macro_rules! phi { () => {  (1. + (5f32).sqrt()) / 2.  }; }         //1.6180340051651001
macro_rules! sideangle { () => {  2. * phi!().atan()  }; }          //2.0344439448698051
//inscribed sphere radius ( ri: f32 = a / 2 * √ ( 25 + 11 * √5 ) / 10 )
macro_rules! inssphererad { () => { dodesize!() * (10. + 22. / (5f32).sqrt()).sqrt() / 4.   }; }    //111.35163307189941
macro_rules! inscirclerad { () => { dodesize!() / ((5. - (5f32).sqrt()) / 2.).sqrt()   }; }         // 85.065082037033278
//megaminx vertex math shortcuts
macro_rules! twofifths { () => { 0.4  }; }
fn pim(x: f32) -> f32 { return x*pi!()/5. }
macro_rules! edgefifth { () => { dodesize!() / pim(2.).sin()   }; }         //105.14622122913930
macro_rules! cospim35 { () => { inscirclerad!() * pim(3.5).cos()   }; }     //-50.000004917867173
macro_rules! cospim15 { () => { inscirclerad!() * pim(1.5).cos()   }; }      //49.999998901510480
macro_rules! sinpim35 { () => { inscirclerad!() * pim(3.5).sin()   }; }      //68.819093936061520

//Math & init functions:
pub trait PieceMath {
    fn cornerInit(&mut self) -> &[Vertex3; 7];
    fn edgeInit(&mut self) -> &[Vertex3; 7];
    fn centerInit(&mut self) -> &[Vertex3; 7];
    fn faceInit(&mut self) -> &[Vertex3; 7];
    fn rotateVertexXYZ(&mut self, index: usize, axis: char, angle: f32);
    fn axis1multi(&mut self, index: usize, pack: Piecepack);
    fn CenterSide1(&mut self, index: usize, pack: Piecepack);
    fn CenterCenter(&mut self, index: usize, pack: Piecepack);
    fn CenterSide2(&mut self, index: usize, pack: Piecepack);
    fn CornerGrp3(&mut self, index: usize, pack: Piecepack);
    fn CornerGrp4(&mut self, index: usize, pack: Piecepack);
    fn EdgeGrp2(&mut self, index: usize, pack: Piecepack);
    fn EdgeGrp3(&mut self, index: usize, pack: Piecepack);
    fn EdgeGrp4(&mut self, index: usize, pack: Piecepack);
    fn EdgeGrp5(&mut self, index: usize, pack: Piecepack);
    fn EdgeGrp6(&mut self, index: usize, pack: Piecepack);
}
//Attach these Math functions to Piece object
impl PieceMath for Piece {
    fn cornerInit(&mut self) -> &[Vertex3; 7] {
        self.numSides = 3;
        for i in 0..7 {
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
        self.rotateVertexXYZ(3, 'z', pim(2.));

        self._vertex[4][0] = cospim15!() * twofifths!(); //brother = 0 or 6
        self._vertex[4][1] = sinpim35!() * twofifths!();
        self.rotateVertexXYZ(4, 'z', pim(-3.));
        self.rotateVertexXYZ(4, 'x', pi!() - sideangle!());
        self.rotateVertexXYZ(4, 'z', pim(2.));

        self._vertex[5][0] = cospim15!() - edgefifth!() * twofifths!(); //brother = 3 or 1
        self._vertex[5][1] = sinpim35!();
        self.rotateVertexXYZ(5, 'z', pim(-3.));
        self.rotateVertexXYZ(5, 'x', pi!() - sideangle!());
        self.rotateVertexXYZ(5, 'z', pim(2.));

        self._vertex[6][0] = cospim15!() * twofifths!(); //brother = 0 or 4
        self._vertex[6][1] = sinpim35!() * twofifths!();
        self.rotateVertexXYZ(6, 'z', pim(-5.));
        self.rotateVertexXYZ(6, 'x', pi!() - sideangle!());
        return &self._vertex;
    }
    //Creates the common starting vertexes for all pieces that are EDGES
    fn edgeInit(&mut self) -> &[Vertex3; 7] {
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
        self.rotateVertexXYZ(4, 'z', pi!());
        self.rotateVertexXYZ(4, 'x', pi!() - sideangle!());

        self._vertex[5][0] = self._vertex[0][0];
        self._vertex[5][1] = self._vertex[0][1];
        self.rotateVertexXYZ(5, 'z', pi!());
        self.rotateVertexXYZ(5, 'x', pi!() - sideangle!());
        return &self._vertex;
    }
    //Creates the common starting vertexes for all pieces that are CENTERS
    fn centerInit(&mut self) -> &[Vertex3; 7] {
        self.numSides = 1;
        for i in 0..5 {
            self._vertex[i][0] = inscirclerad!() * (pim(2.) * (i as f32) + pim(1.5)).cos() * twofifths!();
            self._vertex[i][1] = inscirclerad!() * (pim(2.) * (i as f32) + pim(1.5)).sin() * twofifths!();
            self._vertex[i][2] = -inssphererad!();
        }
        return &self._vertex;
    }    
    //Creates the common starting vertexes for all pieces that are FACES
    fn faceInit(&mut self) -> &[Vertex3; 7] {
        self.numSides = 0;
        for i in 0..5 {
            //This puts it on the back face
            self._vertex[i][0] = cospim35!() + edgefifth!() * twofifths!();
            self._vertex[i][1] = -sinpim35!();
            self._vertex[i][2] = -inssphererad!();
            self.rotateVertexXYZ(i, 'z', pim(2.));
            self.rotateVertexXYZ(i, 'x', pi!() - sideangle!());
            self.rotateVertexXYZ(i, 'z', (i as f32) * pim(2.));
        }
        return &self._vertex;
    }

    fn rotateVertexXYZ(&mut self, index: usize, axis: char, angle: f32) {
        let mut vxIndex: usize = 0;
        let mut vyIndex: usize = 0;
        match axis {
            'x' => { vxIndex=1; vyIndex=2; },
            'y' => vyIndex=2,
            'z' => vyIndex=1,
            _ => println!("Axis must be in x, y, z"),
        }
        let vx: f32 = self._vertex[index][vxIndex];
        let vy: f32 = self._vertex[index][vyIndex];
        let r: f32 = (vx * vx + vy * vy).sqrt();
        let mut a: f32 = if vy > 0. { (vx / r).acos() } else { 2. * pi!() - (vx / r).acos() };
        a += angle;
        self._vertex[index][vxIndex] = r * a.cos();
        self._vertex[index][vyIndex] = r * a.sin();
    }   
//Vertex Transformation Functions
    //main transform: used in almost every other algo
    fn axis1multi(&mut self, index: usize, pack: Piecepack) {
        self.rotateVertexXYZ(index, pack.axis1, pim(pack.multi as f32));
    }
    fn CenterSide1(&mut self, index: usize, pack: Piecepack) {
        self.rotateVertexXYZ(index, pack.axis1, pim(1.));
        self.rotateVertexXYZ(index, pack.axis2, pi!() - sideangle!());
        self.axis1multi(index, pack);
    }
    fn CenterCenter(&mut self, index: usize, pack: Piecepack) {
        self.rotateVertexXYZ(index, pack.axis1, pi!());
    }
    fn CenterSide2(&mut self, index: usize, pack: Piecepack) {
        self.CenterCenter(index, pack);
        self.rotateVertexXYZ(index, pack.axis2, pi!() - sideangle!());
        self.rotateVertexXYZ(index,  'z', pim(pack.multi as f32));
        //This is always z, because axis1/2 are usually y/x and
        //is re-used by face, where it is Z.
    }
    fn CornerGrp3(&mut self, index: usize, pack: Piecepack) {
        self.CenterSide1(index, pack);
        self.rotateVertexXYZ(index, pack.axis2, pi!());
    }
    fn CornerGrp4(&mut self, index: usize, pack: Piecepack) {
        self.CenterCenter(index, pack);
        self.rotateVertexXYZ(index, pack.axis2, pim(pack.multi as f32));
    }
    fn EdgeGrp2(&mut self, index: usize, pack: Piecepack) {
        self.rotateVertexXYZ(index, pack.axis1, pim(3.));
        self.rotateVertexXYZ(index, pack.axis2, pi!() - sideangle!());
        self.axis1multi(index, pack);
    }
    fn EdgeGrp3(&mut self, index: usize, pack: Piecepack) {
        self.rotateVertexXYZ(index, pack.axis1, pim(6.));
        self.EdgeGrp2(index, pack);
    }
    fn EdgeGrp4(&mut self, index: usize, pack: Piecepack) {
        self.rotateVertexXYZ(index, pack.axis1, pim(8.));
        self.EdgeGrp2(index, pack);
    }
    fn EdgeGrp5(&mut self, index: usize, mut pack: Piecepack) {
        pack.multi += 1;
        self.rotateVertexXYZ(index, pack.axis1, pim(2.));
        self.rotateVertexXYZ(index, pack.axis2, sideangle!());
        self.axis1multi(index, pack);
    }
    fn EdgeGrp6(&mut self, index: usize, pack: Piecepack) {
        self.rotateVertexXYZ(index, pack.axis2, pi!());
        self.axis1multi(index, pack);
    }
}

}
