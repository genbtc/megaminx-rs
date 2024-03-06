// megaminx-rs - a rust and SDL2 version of Megaminx - previously a C++ and OpenGL Dodecahedron Cube                                                                                                                                                                          
// Megaminx-rs/piece.rs - LICENSE - AGPL3 - genr8eofl @ genBTC - for megaminx-rs (2024)
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_variables)]
pub mod piece {
use crate::piece_color::PieceColor::{ColorData, ColorPack, ColorPiece, G_COLORRGBS};
use glm::* ; //{Vector3,Matrix4};

//Vertex 3 Position Definitions
#[derive(Copy, Clone, Default)]
pub struct VertexPosition {
    pub position: [f32; 3],
}   // (for glium)
use glium::implement_vertex;
implement_vertex!(VertexPosition, position);
//Vertex 3 Position/Color Definitions
#[derive(Copy, Clone, Default)]
pub struct VertexPositionColor {
    pub position: [f32; 3],
    pub color: [f32; 3],
}   // (for glium)
implement_vertex!(VertexPositionColor, position, color);

//typedef for Regular Vertex 3 and 3;7
pub type Vertex3 = [f32; 3];
pub type Vertex3x7 = [Vertex3; 7];

//Default initializer data for vertex
pub const VERTEXZERO: Vertex3 = [0.0,0.0,0.0];
pub const VERTEXDATAZERO: Vertex3x7 = [VERTEXZERO; 7];
pub const COLORGRAY: Vertex3 = [0.5,0.5,0.5];

//Piece Block
#[derive(Copy, Clone, Default, PartialEq)]
pub struct PieceData {
    pub pieceNum: usize,
    pub flipStatus: usize,
    pub hotPieceMoving: bool,
    pub color: ColorData,
    //Shape enum - same as numSides = NEW
    pub shape: Shape,
}  //data-members - (can swap out all at once)

//Piece Object (main)
#[derive(Copy, Clone, Default, PartialEq)]
pub struct Piece {
    //Coords for GL vertex*3 (up to 7, not all used)
    pub vertex: Vertex3x7,
    //Keeps the default number in the piece. do not swap.
    pub defaultPieceNum: usize,
    //Center has 1, Edge has 2, Corner has 3
    pub numSides: usize,
    //Data Struct (can swap out)
    pub data: PieceData,
    //alternate form of Vertex
    pub points: Points,
}
//Initialize constructor
impl Piece {
    pub fn new(defaultPieceNum: usize) -> Self {
      Self {
        vertex: Default::default(),
        numSides: 0,
        defaultPieceNum,
        data: Default::default(),
        points: Default::default(),
      }
    }
    pub fn swapdata(&mut self, data: &mut PieceData) {
        std::mem::swap(&mut self.data, data);
    }
    pub fn getvertex(&self) -> Vertex3x7 {
        self.vertex
    }
    pub fn getdata(&self) -> PieceData {
        self.data
    }
    pub fn getcolor(&self) -> ColorData {
        self.data.color
    }
    pub fn getpoints(&self) -> Points {
        self.points
    }
}
#[derive(Copy, Clone , Default, PartialEq)]
pub enum Shape {
    #[default]
    EmptyPiece  = 0,
    CenterPiece = 1,
    EdgePiece   = 2,
    CornerPiece = 3,
}
pub use Shape::*;

//MATHEMATICAL CONSTANTS: (as macros, since float math functions cant be declared const/static)
//arbitrary size of dodecahedron - default size in 3d coords for main megaminx
macro_rules! dodesize { () => {   85f32   }; }  //smaller size than original 100
//common geometric constants                                                        (exact representation ©2024 Wolfram Alpha LLC)
macro_rules! pi { () => {  (-1f32).acos()  }; }                //3.1415927410125732 (3.1415926535897932384626433832795028841971693993751058209749445923)
//golden ratio (phi) (also the ratio between the side length of a regular pentagon and one of its diagonals.)
macro_rules! pent { () => { (5f32).sqrt() }; }
macro_rules! phi { () => {  (1. + pent!() ) / 2.  }; }         //1.6180340051651001 (1.6180339887498948482045868343656381177203091798057628621354486227)
macro_rules! sideangle { () => {  2. * phi!().atan()  }; }     //2.0344439448698051 (2.0344439357957027354455779231009658441271217539736731742984053848)
macro_rules! pisideangle { () => {  pi!() - sideangle!() }; }  //1.107148796        (1.1071487177940905030170654601785370400700476454014326466765392074)
//inscribed sphere radius ( ri: f32 = a / 2 * √ ( 25 + 11 * √5 ) / 10 )
macro_rules! inssphererad { () => { dodesize!() * (10. + 22. / pent!()).sqrt() / 4.   }; }    //111.35163307189941
macro_rules! inscirclerad { () => { dodesize!() / ((5. - pent!()) / 2.).sqrt()   }; }         // 85.065082037033278
//megaminx vertex math shortcuts
macro_rules! twofifths { () => { 0.4  }; }
fn pim(x: f32) -> f32 { x*pi!()/5. }
macro_rules! edgefifth { () => { dodesize!() / pim(2.).sin()   }; }         //105.14622122913930
macro_rules! cospim35 { () => { inscirclerad!() * pim(3.5).cos()   }; }     //-50.000004917867173
macro_rules! cospim15 { () => { inscirclerad!() * pim(1.5).cos()   }; }      //49.999998901510480
macro_rules! sinpim35 { () => { inscirclerad!() * pim(3.5).sin()   }; }      //68.819093936061520

#[derive(Copy, Clone, Default, PartialEq, Debug)]
pub struct Points {
    pub a: Vertex3,
    pub b: Vertex3,
    pub c: Vertex3,
    pub d: Vertex3,
    pub e: Vertex3,
    pub f: Vertex3,
    pub g: Vertex3,
}
impl Points {
    pub fn length(&self, vec: Vertex3) -> f32 {
      return {
        let mut accum = 0.0;
        for v in vec {
            accum += v*v;
        }
        accum.sqrt()
      }
    }
    pub fn dot(&self, v1: Vertex3, v2: Vertex3) -> f32 {
        v1[0]*v2[0] + v1[1]*v2[1] + v1[2]*v2[2]
    }
    pub fn multiply(&self, vertex: Vertex3, m: [[f32;4];4]) -> Vector3<f32> {
        let       v =  Vector3::<f32>::from_array(&vertex);
        let mut dst =  Vector3::<f32>::from(*v);
        dst.x = v.x*m[0][0] + v.y*m[1][0] + v.z*m[2][0] + 0.0;
        dst.y = v.x*m[0][1] + v.y*m[1][1] + v.z*m[2][1] + 0.0;
        dst.z = v.x*m[0][2] + v.y*m[1][2] + v.z*m[2][2] + 0.0;
        //dst.w = v.x*m[0][3] + v.y*m[1][3] + v.z*m[2][3] + 0.0;
        dst
    } 
    // error[E0277]: cannot multiply `f32` by `Vector4<f32>`
    //  |                    ^ no implementation for `f32 * Vector4<f32>`
    //  = help: the trait `std::ops::Mul<Vector4<f32>>` is not implemented for `f32`    
}
//Piece Pack struct to rotateVertexXYZ
#[derive(Copy, Clone, Default)]
pub struct PiecePack {
    pub axis1: char,
    pub axis2: char,
    pub multi: usize
}
//Math & init Piece functions:
pub trait PieceInit {
    fn cornerInit(&mut self) -> &Vertex3x7;
    fn edgeInit(&mut self) -> &Vertex3x7;
    fn centerInit(&mut self) -> &Vertex3x7;
    fn faceInit(&mut self) -> &Vertex3x7;
}
pub trait PieceMath {    
    fn rotateVertexXYZ(&mut self, index: usize, axis: char, angle: f32);
    fn axis1multi(&mut self, index: usize, pack: PiecePack);
    fn CenterSide1(&mut self, index: usize, pack: PiecePack);
    fn CenterCenter(&mut self, index: usize, pack: PiecePack);
    fn CenterSide2(&mut self, index: usize, pack: PiecePack);
    fn CornerGrp3(&mut self, index: usize, pack: PiecePack);
    fn CornerGrp4(&mut self, index: usize, pack: PiecePack);
    fn EdgeGrp2(&mut self, index: usize, pack: PiecePack);
    fn EdgeGrp3(&mut self, index: usize, pack: PiecePack);
    fn EdgeGrp4(&mut self, index: usize, pack: PiecePack);
    fn EdgeGrp5(&mut self, index: usize, pack: PiecePack);
    fn EdgeGrp6(&mut self, index: usize, pack: PiecePack);
}
impl PieceInit for Piece {
    fn cornerInit(&mut self) -> &Vertex3x7 {
        //println!("cornerInit({})", self.defaultPieceNum);
        self.numSides = 3;
        for i in 0..7 {
            self.vertex[i][2] = -inssphererad!();
        }
        self.vertex[0][0] = cospim35!() * twofifths!(); //inside corner (aka outside center)
        self.vertex[0][1] = sinpim35!() * twofifths!();
        self.points.a = self.vertex[0];
        self.vertex[1][0] = cospim35!() + edgefifth!() * twofifths!(); //corner inside edge a
        self.vertex[1][1] = sinpim35!();
        self.points.b = self.vertex[1];
        self.vertex[2][0] = cospim35!();     //outside corner
        self.vertex[2][1] = self.vertex[1][1];
        self.points.c = self.vertex[2];
        self.vertex[3][0] = cospim15!() - edgefifth!() * twofifths!(); //corner inside edge b
        self.vertex[3][1] = self.vertex[1][1];
        self.rotateVertexXYZ(3, 'z', pim(2.));
        self.points.d = self.vertex[3];
        self.vertex[4][0] = cospim15!() * twofifths!(); //brother = 6 + 0
        self.vertex[4][1] = self.vertex[0][1];
        self.rotateVertexXYZ(4, 'z', pim(-3.));
        self.rotateVertexXYZ(4, 'x', pisideangle!());
        self.rotateVertexXYZ(4, 'z', pim(2.));
        self.points.e = self.vertex[4];
        self.vertex[5][0] = cospim15!() - edgefifth!() * twofifths!(); //brother = 3 + 1
        self.vertex[5][1] = self.vertex[1][1];
        self.rotateVertexXYZ(5, 'z', pim(-3.));
        self.rotateVertexXYZ(5, 'x', pisideangle!());
        self.rotateVertexXYZ(5, 'z', pim(2.));
        self.points.f = self.vertex[5];
        self.vertex[6][0] = cospim15!() * twofifths!(); //brother = 4 + 0
        self.vertex[6][1] = self.vertex[0][1];
        self.rotateVertexXYZ(6, 'z', pim(-5.));
        self.rotateVertexXYZ(6, 'x', pisideangle!());
        self.points.g = self.vertex[6];
        &self.vertex
    }
    //Creates the common starting vertexes for all pieces that are EDGES
    fn edgeInit(&mut self) -> &Vertex3x7 {
        //println!("edgeInit({})", self.defaultPieceNum);
        self.numSides = 2;
        for i in 0..6 {
            self.vertex[i][2] = -inssphererad!();
        }

        self.vertex[0][0] = cospim35!() * twofifths!();
        self.vertex[0][1] = sinpim35!() * twofifths!();
        self.points.a = self.vertex[0];
        self.vertex[1][0] = cospim15!() * twofifths!();
        self.vertex[1][1] = self.vertex[0][1];
        self.points.b = self.vertex[1];
        self.vertex[2][0] = cospim15!() - edgefifth!() * twofifths!();
        self.vertex[2][1] = sinpim35!();
        self.points.c = self.vertex[2];
        // self.vertex[3][0] = cospim35!() + edgefifth!() * twofifths!();
        self.vertex[3][0] = self.vertex[2][0];
        self.vertex[3][1] = self.vertex[2][1];
        self.rotateVertexXYZ(3, 'z', pi!());
        self.rotateVertexXYZ(3, 'x', pisideangle!());
        self.points.d = self.vertex[3];
        self.vertex[4][0] = self.vertex[1][0];
        self.vertex[4][1] = self.vertex[1][1];
        self.rotateVertexXYZ(4, 'z', pi!());
        self.rotateVertexXYZ(4, 'x', pisideangle!());
        self.points.e = self.vertex[4];
        self.vertex[5][0] = self.vertex[0][0];
        self.vertex[5][1] = self.vertex[0][1];
        self.rotateVertexXYZ(5, 'z', pi!());
        self.rotateVertexXYZ(5, 'x', pisideangle!());
        self.points.f = self.vertex[5];
        &self.vertex
    }
    //Creates the common starting vertexes for all pieces that are CENTERS
    fn centerInit(&mut self) -> &Vertex3x7 {
        //println!("centerInit({})", self.defaultPieceNum);
        self.numSides = 1;
        for i in 0..5 {
            self.vertex[i][0] = inscirclerad!() * (pim(2.) * (i as f32) + pim(1.5)).cos() * twofifths!();
            self.vertex[i][1] = inscirclerad!() * (pim(2.) * (i as f32) + pim(1.5)).sin() * twofifths!();
            self.vertex[i][2] = -inssphererad!();
        }
        self.points.a = self.vertex[0];
        self.points.b = self.vertex[1];
        self.points.c = self.vertex[2];
        self.points.d = self.vertex[3];
        self.points.e = self.vertex[4];
        &self.vertex
    }    
    //Creates the common starting vertexes for all pieces that are FACES
    fn faceInit(&mut self) -> &Vertex3x7 {
        //println!("faceInit({})", self.defaultPieceNum);
        self.numSides = 0;
        for i in 0..5 {
            //This puts it on the back face
            self.vertex[i][0] = cospim35!() + edgefifth!() * twofifths!();
            self.vertex[i][1] = -sinpim35!();
            self.vertex[i][2] = -inssphererad!();
            self.rotateVertexXYZ(i, 'z', pim(2.));
            self.rotateVertexXYZ(i, 'x', pisideangle!());
            self.rotateVertexXYZ(i, 'z', (i as f32) * pim(2.));
        }
        self.points.a = self.vertex[0];
        self.points.b = self.vertex[1];
        self.points.c = self.vertex[2];
        self.points.d = self.vertex[3];
        self.points.e = self.vertex[4];        
        &self.vertex
    }
}
//Attach these Math traits to Piece object
impl PieceMath for Piece {
    //Vertex Transformation Functions
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
    //main transform: used in almost every other algo
    fn axis1multi(&mut self, index: usize, pack: PiecePack) {
        self.rotateVertexXYZ(index, pack.axis1, pim(pack.multi as f32));
    }
    fn CenterSide1(&mut self, index: usize, pack: PiecePack) {
        self.rotateVertexXYZ(index, pack.axis1, pim(1.));
        self.rotateVertexXYZ(index, pack.axis2, pisideangle!());
        self.axis1multi(index, pack);
    }
    fn CenterCenter(&mut self, index: usize, pack: PiecePack) {
        self.rotateVertexXYZ(index, pack.axis1, pi!());
    }
    fn CenterSide2(&mut self, index: usize, pack: PiecePack) {
        self.CenterCenter(index, pack);
        self.rotateVertexXYZ(index, pack.axis2, pisideangle!());
        self.rotateVertexXYZ(index,  'z', pim(pack.multi as f32));
        //note: always z, because axis1/2 are y/x. the z gets re-used by face.
    }
    fn CornerGrp3(&mut self, index: usize, pack: PiecePack) {
        self.CenterSide1(index, pack);
        self.rotateVertexXYZ(index, pack.axis2, pi!());
    }
    fn CornerGrp4(&mut self, index: usize, pack: PiecePack) {
        self.CenterCenter(index, pack);
        self.rotateVertexXYZ(index, pack.axis2, pim(pack.multi as f32));
    }
    fn EdgeGrp2(&mut self, index: usize, pack: PiecePack) {
        self.rotateVertexXYZ(index, pack.axis1, pim(3.));
        self.rotateVertexXYZ(index, pack.axis2, pisideangle!());
        self.axis1multi(index, pack);
    }
    fn EdgeGrp3(&mut self, index: usize, pack: PiecePack) {
        self.rotateVertexXYZ(index, pack.axis1, pim(6.));
        self.EdgeGrp2(index, pack);
    }
    fn EdgeGrp4(&mut self, index: usize, pack: PiecePack) {
        self.rotateVertexXYZ(index, pack.axis1, pim(8.));
        self.EdgeGrp2(index, pack);
    }
    fn EdgeGrp5(&mut self, index: usize, mut pack: PiecePack) {
        pack.multi += 1;
        self.rotateVertexXYZ(index, pack.axis1, pim(2.));
        self.rotateVertexXYZ(index, pack.axis2, sideangle!());
        self.axis1multi(index, pack);
    }
    fn EdgeGrp6(&mut self, index: usize, pack: PiecePack) {
        self.rotateVertexXYZ(index, pack.axis2, pi!());
        self.axis1multi(index, pack);
    }
}
//Piece Color Implementations
pub trait PieceColor {
    fn flipColor(&mut self);    
    fn setColor(&mut self, i: usize, c: ColorPack);
    fn initColorIndex(&mut self, idx: usize, k: usize);
    fn initColorA(&mut self, a: usize);
    fn initColor(&mut self, color: ColorPiece, corner: bool);
    fn matchesColor(&self, color: usize) -> bool;
    fn anyColor(&self, color: usize) -> bool;
    fn match_color(&self, color: usize) -> bool;
    fn isShape(&mut self) -> Shape;
}
impl PieceColor for Piece {
    //Flip - Changes colors. rotate/switches colors for current piece
    fn flipColor(&mut self) {
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
    //Set / Initializer for entire
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
    //Uses the two arrays g_cornerPiecesColors and g_edgePiecesColors to populate.
    fn initColor(&mut self, color: ColorPiece, corner: bool ) {
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
    //check if color-num (int) matches ANY colors (3 sided)
    // currently stored in struct data
    fn matchesColor(&self, color: usize) -> bool {
        return self.data.color.colorNum[0] == color ||
               self.data.color.colorNum[1] == color ||
               self.data.color.colorNum[2] == color;
    }
    fn anyColor(&self, color: usize) -> bool {
        self.data.color.colorNum.iter().any(|&rgb| rgb == color)
    }
    fn match_color(&self, color: usize) -> bool {
        match self.data.color.colorNum {
            |[_,_,_] => {true},
        }
    }
    fn isShape(&mut self) -> Shape {
        match self.numSides {
            2 => { EdgePiece  },
            3 => { CornerPiece},
            1 => { CenterPiece},
            0|_ => { EmptyPiece },
        }
    }    
  }
  pub struct NumDir {                                                                                                                                                                                                                                         
    pub num: i8,
    pub dir: i8,
    pub algo: i8,
  }
  pub trait EdgeCornerInit : PieceInit {
    fn new(&mut self);
    fn newEnum(&mut self, shapeType: Shape);
    fn init_data(&mut self, vertex_base: [Vertex3; 7]);
    fn init_edge_data(&mut self, piecenum: usize, vertex_base: [Vertex3; 7]);
    fn init_corner_data(&mut self, piecenum: usize, vertex_base: [Vertex3; 7]);
    fn create_edge_axis(&mut self, piecenum: usize, index: usize);
    fn create_corner_axis(&mut self, piecenum: usize, index: usize);
    fn create_center_axis(&mut self, piecenum: usize, index: usize);
    fn switch_axis(&mut self, shapeType: Shape, piecenum: usize, index: usize);
  }
  impl EdgeCornerInit for Piece {
    fn new(&mut self) {
        match self.numSides {
            2 => { self.edgeInit();   self.data.shape = EdgePiece;  },
            3 => { self.cornerInit(); self.data.shape = CornerPiece;},
            1 => { self.centerInit(); self.data.shape = CenterPiece;},
            0 => { self.faceInit();   self.data.shape = EmptyPiece; },
            _ => {},
        }
        //self.init(self.defaultPieceNum, true);
    }
    //Starts a piece based on the Shape Enum passed in.
    fn newEnum(&mut self, shapeType: Shape) {
        match shapeType {
            EdgePiece   => { self.edgeInit();   },
            CornerPiece => { self.cornerInit(); },
            CenterPiece => { self.centerInit(); },
            EmptyPiece  => { self.faceInit();   },
        }
    }
    /**
     * \brief Inits the piece with a pre-existing Vertex Array
     * \param vertexBase the starting points to be memcpy'ed in
     */
    fn init_data(&mut self, vertex_base: [Vertex3; 7]) {
        self.vertex = vertex_base;
    }
    fn init_edge_data(&mut self, piecenum: usize, vertex_base: [Vertex3; 7]) {
        self.vertex = vertex_base;
        crate::edge::edge::Edge::init(self, piecenum, true);
    }
    fn init_corner_data(&mut self, piecenum: usize, vertex_base: [Vertex3; 7]) {
        self.vertex = vertex_base;
        crate::corner::corner::Corner::init(self, piecenum, true)
    }
    /**
     * \brief createAxis sets up the x,y,z Axes that the EdgeCorner pieces ride on
     * \note (called by init on startup)
     * \param n - the number of the piece (piecenum)
     */
    fn create_edge_axis(&mut self, piecenum: usize, index: usize) {
        let pack: PiecePack = PiecePack { axis1: 'z', axis2:'x', multi: (piecenum * 2 % 10) };
        match piecenum + 1 {
        1..=5 => {
            self.axis1multi(index, pack); },
        6..=10 => {
            self.EdgeGrp2(index, pack); },
        11..=15 => {
            self.EdgeGrp3(index, pack); },
        16..=20 => {
            self.EdgeGrp4(index, pack); },
        21..=25 => {
            self.EdgeGrp5(index, pack); },
        26..=30 => {
            self.EdgeGrp6(index, pack); },
        _ => println!("Must be within 1-30"),
        }
    }
    fn create_corner_axis(&mut self, piecenum: usize, index: usize) {
        let mut pack: PiecePack = PiecePack { axis1: 'z', axis2:'x', multi: (piecenum * 2 % 10) };
        match piecenum + 1 {
        1=> { },
        2..=5 => {
            self.axis1multi(index, pack); },
        6..=10 => {
            self.CenterSide1(index, pack); },
        11..=15 => {
            self.CornerGrp3(index, pack); },
        16..=20 => {
            pack.axis1 = 'x'; pack.axis2 = 'z';
            self.CornerGrp4(index, pack); },
        _ => println!("Must be within 1-20"),
        }
    }
    fn create_center_axis(&mut self, piecenum: usize, index: usize) {
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
    //Starts a piece based on the Shape Enum passed in.
    fn switch_axis(&mut self, shapeType: Shape, piecenum: usize, index: usize) {
        match shapeType {
            EdgePiece   => { self.create_edge_axis(piecenum, index);   },
            CornerPiece => { self.create_corner_axis(piecenum, index); },
            CenterPiece => { self.create_center_axis(piecenum, index); },
            EmptyPiece  => { },
        }
    }    
  }
}
