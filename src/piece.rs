// megaminx-rs - a rust and SDL2 version of Megaminx - previously a C++ and OpenGL Dodecahedron Cube                                                                                                                                                                          
// Megaminx-rs/piece.rs - LICENSE - AGPL3 - genr8eofl @ genBTC - for megaminx-rs (2024)
#![allow(non_snake_case)]
#![allow(dead_code)]
pub mod piece {
use crate::piece_color::PieceColor::{ColorData, ColorPack, ColorPiece, G_COLORRGBS, G_EDGEPIECESCOLORS};

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
}
//Initialize constructor
impl Piece {
    pub fn new(defaultPieceNum: usize) -> Self {
      Self {
        vertex: Default::default(),
        numSides: 0,
        defaultPieceNum,
        data: Default::default(),
      }
    }
    pub fn swapdata(&mut self, data: &mut PieceData) {
        std::mem::swap(&mut self.data, data);
    }
    pub fn getpos(&self) -> Vertex3x7 {
        self.vertex
    }
    pub fn getdata(&self) -> PieceData {
        self.data
    }
    pub fn getcolor(&self) -> ColorData {
        self.data.color
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
fn pim(x: f32) -> f32 { x*pi!()/5. }
macro_rules! edgefifth { () => { dodesize!() / pim(2.).sin()   }; }         //105.14622122913930
macro_rules! cospim35 { () => { inscirclerad!() * pim(3.5).cos()   }; }     //-50.000004917867173
macro_rules! cospim15 { () => { inscirclerad!() * pim(1.5).cos()   }; }      //49.999998901510480
macro_rules! sinpim35 { () => { inscirclerad!() * pim(3.5).sin()   }; }      //68.819093936061520

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
            self.rotateVertexXYZ(i, 'x', pi!() - sideangle!());
            self.rotateVertexXYZ(i, 'z', (i as f32) * pim(2.));
        }
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
        self.rotateVertexXYZ(index, pack.axis2, pi!() - sideangle!());
        self.axis1multi(index, pack);
    }
    fn CenterCenter(&mut self, index: usize, pack: PiecePack) {
        self.rotateVertexXYZ(index, pack.axis1, pi!());
    }
    fn CenterSide2(&mut self, index: usize, pack: PiecePack) {
        self.CenterCenter(index, pack);
        self.rotateVertexXYZ(index, pack.axis2, pi!() - sideangle!());
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
        self.rotateVertexXYZ(index, pack.axis2, pi!() - sideangle!());
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
    //check if color-num (int) matches any colors
    // currently stored in struct data (3 sided)
    fn matchesColor(&self, color: usize) -> bool {
        return self.data.color.colorNum[0] == color ||
               self.data.color.colorNum[1] == color ||
               self.data.color.colorNum[2] == color;
    }
  }
  pub struct NumDir {                                                                                                                                                                                                                                         
    pub num: i8,
    pub dir: i8,
    pub algo: i8,
  }
  pub trait EdgeCornerInit : PieceInit {
    fn new(&mut self);
    fn init_data(&mut self, piecenum: usize, vertex_base: [Vertex3; 7]);
    fn init(&mut self, piecenum: usize, do_axes: bool);
    fn create_axis(&mut self, piecenum: usize, index: usize);    
  }
  impl EdgeCornerInit for Piece {
    fn new(&mut self) {
        self.edgeInit();
        self.init(self.defaultPieceNum, true);
    }
    /**
     * \brief Inits the piece with a pre-existing Vertex Array
     * \param edgeVertexBase the starting points to be memcpy'ed in
     */
    fn init_data(&mut self, piecenum: usize, edge_vertex_base: [Vertex3; 7]) {
        self.vertex = edge_vertex_base;
        self.init(piecenum, true);
    }
    /**
     * \brief Inits a Edge piece
     * \note  (calls createAxis and initColor)
     * \param n the number of the Edge piece (piecenum)
     * \param doAxes True by default. First Time Initialization Only
     */
    fn init(&mut self, piecenum: usize, do_axes: bool) {
        if do_axes {
            for i in 0..6 {
                self.create_axis(piecenum, i);
            }
        }
        self.initColor(G_EDGEPIECESCOLORS[piecenum], false);
        self.data.pieceNum = piecenum;
        self.defaultPieceNum = piecenum;
    }
    fn create_axis(&mut self, piecenum: usize, index: usize) {
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
  }
}
