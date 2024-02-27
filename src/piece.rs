// megaminx-rs - a rust and SDL2 version of Megaminx - previously a C++ and OpenGL Dodecahedron Cube                                                                                                                                                                          
// Megaminx-rs/piece.rs - LICENSE - AGPL3 - genr8eofl @ genBTC - for megaminx-rs (2024)
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_imports)]
pub mod piece {
use crate::piece_color::PieceColor::{ColorPack,ColorPiece, G_COLORRGBS};

//Vertex 3 Position Definitions
#[derive(Copy, Clone, Default)]
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

//Color Block
#[derive(Copy, Clone, Default)]
pub struct ColorData {
    pub colorNum: [usize; 3],
    pub colorName: [&'static str; 3],
    pub colorRGB: [Vertex3; 3],
    pub pack: ColorPack,
}
// Piece Block
#[derive(Copy, Clone, Default)]
pub struct PieceData {
    pub pieceNum: usize,
    pub flipStatus: usize,
    pub hotPieceMoving: bool,
    pub color: ColorData,
} //data-members - we can swap out all at once.

//Piece Pack struct to rotateVertexXYZ
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
    //Coords for GL vertex*3 (up to 7, not all used)
    pub vertex: [Vertex3; 7],
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
        vertex: Default::default(),
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
    fn flip(&mut self);
    fn flip_twice(&mut self);
    fn swapdata(&mut self, data: &mut PieceData);
}
//Attach these Math functions to Piece object
impl PieceMath for Piece {
    fn cornerInit(&mut self) -> &[Vertex3; 7] {
        //println!("cornerInit({})", self.defaultPieceNum);
        self.numSides = 3;
        for i in 0..7 {
            self.vertex[i][2] = -inssphererad!();
        }

        self.vertex[0][0] = cospim35!() * twofifths!(); //inside corner (aka outside center)
        self.vertex[0][1] = sinpim35!() * twofifths!();

        self.vertex[1][0] = cospim35!() + edgefifth!() * twofifths!(); //corner inside edge a
        self.vertex[1][1] = sinpim35!();

        self.vertex[2][0] = cospim35!();     //outside corner
        self.vertex[2][1] = sinpim35!();

        self.vertex[3][0] = cospim15!() - edgefifth!() * twofifths!(); //corner inside edge b
        self.vertex[3][1] = sinpim35!();
        self.rotateVertexXYZ(3, 'z', pim(2.));

        self.vertex[4][0] = cospim15!() * twofifths!(); //brother = 0 or 6
        self.vertex[4][1] = sinpim35!() * twofifths!();
        self.rotateVertexXYZ(4, 'z', pim(-3.));
        self.rotateVertexXYZ(4, 'x', pi!() - sideangle!());
        self.rotateVertexXYZ(4, 'z', pim(2.));

        self.vertex[5][0] = cospim15!() - edgefifth!() * twofifths!(); //brother = 3 or 1
        self.vertex[5][1] = sinpim35!();
        self.rotateVertexXYZ(5, 'z', pim(-3.));
        self.rotateVertexXYZ(5, 'x', pi!() - sideangle!());
        self.rotateVertexXYZ(5, 'z', pim(2.));

        self.vertex[6][0] = cospim15!() * twofifths!(); //brother = 0 or 4
        self.vertex[6][1] = sinpim35!() * twofifths!();
        self.rotateVertexXYZ(6, 'z', pim(-5.));
        self.rotateVertexXYZ(6, 'x', pi!() - sideangle!());
        return &self.vertex;
    }
    //Creates the common starting vertexes for all pieces that are EDGES
    fn edgeInit(&mut self) -> &[Vertex3; 7] {
        //println!("edgeInit({})", self.defaultPieceNum);
        self.numSides = 2;
        for i in 0..6 {
            self.vertex[i][2] = -inssphererad!();
        }

        self.vertex[0][0] = cospim35!() * twofifths!();
        self.vertex[0][1] = sinpim35!() * twofifths!();

        self.vertex[1][0] = cospim15!() * twofifths!();
        self.vertex[1][1] = sinpim35!() * twofifths!();

        self.vertex[2][0] = cospim15!() - edgefifth!() * twofifths!();
        self.vertex[2][1] = sinpim35!();

        self.vertex[3][0] = cospim35!() + edgefifth!() * twofifths!();
        self.vertex[3][1] = sinpim35!();

        self.vertex[4][0] = self.vertex[1][0];
        self.vertex[4][1] = self.vertex[1][1];
        self.rotateVertexXYZ(4, 'z', pi!());
        self.rotateVertexXYZ(4, 'x', pi!() - sideangle!());

        self.vertex[5][0] = self.vertex[0][0];
        self.vertex[5][1] = self.vertex[0][1];
        self.rotateVertexXYZ(5, 'z', pi!());
        self.rotateVertexXYZ(5, 'x', pi!() - sideangle!());
        return &self.vertex;
    }
    //Creates the common starting vertexes for all pieces that are CENTERS
    fn centerInit(&mut self) -> &[Vertex3; 7] {
        //println!("centerInit({})", self.defaultPieceNum);
        self.numSides = 1;
        for i in 0..5 {
            self.vertex[i][0] = inscirclerad!() * (pim(2.) * (i as f32) + pim(1.5)).cos() * twofifths!();
            self.vertex[i][1] = inscirclerad!() * (pim(2.) * (i as f32) + pim(1.5)).sin() * twofifths!();
            self.vertex[i][2] = -inssphererad!();
        }
        return &self.vertex;
    }    
    //Creates the common starting vertexes for all pieces that are FACES
    fn faceInit(&mut self) -> &[Vertex3; 7] {
        //println!("faceInit({})", self.defaultPieceNum);
        self.numSides = 0;
        for i in 0..5 {
            //This puts it on the back face
            self.vertex[i][0] = cospim35!() + edgefifth!() * twofifths!();
            self.vertex[i][1] = -sinpim35!();
            self.vertex[i][2] = -inssphererad!();
            self.rotateVertexXYZ(i, 'z', pim(2.));
            self.rotateVertexXYZ(i, 'x', pi!() - sideangle!());
            self.rotateVertexXYZ(i, 'z', (i as f32) * pim(2.));
        }
        return &self.vertex;
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
        let vx: f32 = self.vertex[index][vxIndex];
        let vy: f32 = self.vertex[index][vyIndex];
        let r: f32 = (vx * vx + vy * vy).sqrt();
        let mut a: f32 = if vy > 0. { (vx / r).acos() } else { 2. * pi!() - (vx / r).acos() };
        a += angle;
        self.vertex[index][vxIndex] = r * a.cos();
        self.vertex[index][vyIndex] = r * a.sin();
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

    //Changes colors. Flip/rotate/switches colors for current piece
    fn flip(&mut self) {
        self.data.color.colorRGB[0].rotate_left(3);
        self.data.color.colorNum.rotate_left(1);
        self.data.color.colorName.rotate_left(1);
        let isCorner: bool = self.numSides == 3;
        if (isCorner && self.data.flipStatus < 2) ||
          (!isCorner && self.data.flipStatus == 0) {
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
    fn swapdata(&mut self, data: &mut PieceData) {
        std::mem::swap(&mut self.data, data);
    }
}
//Piece Color Implementations
pub trait PieceColor {
    fn setColor(&mut self, i: usize, c: ColorPack);
    fn initColorIndex(&mut self, idx: usize, k: usize);
    fn initColorA(&mut self, a: usize);
    fn initColorAB(&mut self, a: usize, b: usize);
    fn initColorABC(&mut self, a: usize, b: usize, c: usize);
    fn initColor(&mut self, color: ColorPiece, corner: bool);
    fn matchesColor(&mut self, color: usize) -> bool;
    
}
impl PieceColor for Piece {
    fn setColor(&mut self, i: usize, c: ColorPack) {
        self.data.color.colorRGB[i][0] = c.r;
        self.data.color.colorRGB[i][1] = c.g;
        self.data.color.colorRGB[i][2] = c.b;
        self.data.color.colorName[i] = c.name;
        self.data.color.colorNum[i] = c.i;
        self.data.color.pack = c; //NEW
        self.data.flipStatus = 0;
    }
    //interface function for setter
    fn initColorIndex(&mut self, idx: usize, k: usize) {
        let thecolor: ColorPack = G_COLORRGBS[k];
        self.setColor(idx, thecolor);
    }
    //store Center color
    fn initColorA(&mut self, a: usize) {
        self.initColorIndex(0, a);
        self.numSides = 1;
    }
    //store Edge colors
    //[[deprecated]]
    fn initColorAB(&mut self, a: usize, b: usize) {
        self.initColorIndex(0, a);
        self.initColorIndex(1, b);
        //set non-existant 3rd side of edge to 0=Black
        // aka not undefined so we can re-use corner.
        self.initColorIndex(2, 0);
        self.numSides = 2;
    }
    //store Corner colors
    //[[deprecated]]
    fn initColorABC(&mut self, a: usize, b: usize, c: usize) {
        self.initColorIndex(0, a);
        self.initColorIndex(1, b);
        self.initColorIndex(2, c);
        self.numSides = 3;
    }
    //Uses the two arrays g_cornerPiecesColors and g_edgePiecesColors to populate.
    fn initColor(&mut self, color: ColorPiece, corner: bool /*false*/) {
        self.initColorIndex(0, color.0 as usize);
        self.initColorIndex(1, color.1 as usize);
        if corner {
            self.initColorIndex(2, color.2 as usize);
            self.numSides = 3;
        }
        else { //edge
            self.initColorIndex(2, 0);   //set 3rd side to black.
            self.numSides = 2;
        }
    }
    //check if color-num (int) matches any colors
    // currently stored in struct data (3 sided)
    fn matchesColor(&mut self, color: usize) -> bool {
        return  self.data.color.colorNum[0] == color ||
                self.data.color.colorNum[1] == color ||
                self.data.color.colorNum[2] == color;
    }
   
    //Function to Reverse the c-Array.
    //template <typename T>
    //void arrayReverse(T arr[], int r, int n)

}

}
