// Megaminx-rs/enum.rs - LICENSE - AGPL3 - genr8eofl @ genBTC - for megaminx-rs (2024)

//Megaminx standard color names defined in numerical int order 
enum MegaminxColor {
    Black,
    White,
    DarkBlue,
    Red,
    DarkGreen,
    Purple,
    Yellow,
    Gray,
    LightBlue,
    Orange,
    LightGreen,
    Pink,
    Beige,
    MaxColorStates
}
use MegaminxColor::*;

//scaffolding Struct, a shell for a color, for G_COLORRGBS
struct ColorPack {
    i: u16,
    r: f32,
    g: f32,
    b: f32,
    name: &'static str
}

/*
struct RGBPack {
	ase: RGB,
    name: &'static str
}*/

//Main list of the index 12 colors in R,G,B form (0-255) = (0.0 - 1.0), Name string for Enum
const MAX_COLOR_STATES: usize = MaxColorStates as usize;
static G_COLORRGBS: [ColorPack; MAX_COLOR_STATES] = [
    ColorPack{ i:0, r:0.0, g:0.0, b:0.0, name:"BLACK" },
    ColorPack{ i:1, r:1.0, g:1.0, b:1.0, name:"WHITE" },
    ColorPack{ i:2, r:0.0, g:0.0, b:1.0, name:"DARK_BLUE" },
    ColorPack{ i:3, r:1.0, g:0.0, b:0.0, name:"RED" },
    ColorPack{ i:4, r:0.0, g:0.4, b:0.0, name:"DARK_GREEN" },
    ColorPack{ i:5, r:0.5, g:0.0, b:1.0, name:"PURPLE" },
    ColorPack{ i:6, r:1.0, g:1.0, b:0.0, name:"YELLOW" },
    ColorPack{ i:7, r:0.5, g:0.5, b:0.5, name:"GRAY" },
    ColorPack{ i:8, r:0.2, g:0.5, b:1.0, name:"LIGHT_BLUE" },
    ColorPack{ i:9, r:1.0, g:0.4, b:0.1, name:"ORANGE" },
    ColorPack{ i:10, r:0.4, g:1.0, b:0.4, name:"LIGHT_GREEN" },
    ColorPack{ i:11, r:0.9, g:0.4, b:1.0, name:"PINK" },
    ColorPack{ i:12, r:1.0, g:0.9, b:0.65, name:"BEIGE" },
];

const NUM_FACES:   i16 = 12;
const NUM_CORNERS: i16 = 20;
const NUM_EDGES:   i16 = 30;

//scaffolding Struct, a shell for a piece, holds 1-3 Colors for a Center/Edge/Corner definition
struct ColorPiece(MegaminxColor,MegaminxColor,MegaminxColor);

static G_EDGEPIECESCOLORS: [ColorPiece; NUM_EDGES as usize] = [
    // 0 - 4
    ColorPiece( White, DarkBlue,  Black),
    ColorPiece( White, Red,       Black),
    ColorPiece( White, DarkGreen, Black),
    ColorPiece( White, Purple,    Black),
    ColorPiece( White, Yellow,    Black),
    // 5 - 9
    ColorPiece( DarkBlue, Red,     Black),
    ColorPiece( Red, DarkGreen,    Black),
    ColorPiece( DarkGreen, Purple, Black),
    ColorPiece( Purple, Yellow,    Black),
    ColorPiece( Yellow, DarkBlue,  Black),
    // 10 - 14
    ColorPiece( DarkBlue, LightGreen, Black),
    ColorPiece( Red, Pink,            Black),
    ColorPiece( DarkGreen, Beige,     Black),
    ColorPiece( Purple, LightBlue,    Black),
    ColorPiece( Yellow, Orange,       Black),
    // 15 - 19
    ColorPiece( DarkBlue, Pink,       Black),
    ColorPiece( Red, Beige,           Black),
    ColorPiece( DarkGreen, LightBlue, Black),
    ColorPiece( Purple, Orange,       Black),
    ColorPiece( Yellow, LightGreen,   Black),
    // 20 - 24
    ColorPiece( Pink, Beige,        Black),
    ColorPiece( Beige, LightBlue,   Black),
    ColorPiece( LightBlue, Orange,  Black),
    ColorPiece( Orange, LightGreen, Black),
    ColorPiece( LightGreen, Pink,   Black),
    // 25 - 29
    ColorPiece( Gray, LightBlue,  Black),
    ColorPiece( Gray, Orange,     Black),
    ColorPiece( Gray, LightGreen, Black),
    ColorPiece( Gray, Pink,       Black),
    ColorPiece( Gray, Beige,      Black)
];

static G_CORNERPIECESCOLORS: [ColorPiece; NUM_CORNERS as usize] = [
    // 0 - 4
    ColorPiece( White, Red, DarkBlue),
    ColorPiece( White, DarkGreen, Red),
    ColorPiece( White, Purple, DarkGreen),
    ColorPiece( White, Yellow, Purple),
    ColorPiece( White, DarkBlue, Yellow),
    // 5 - 9
    ColorPiece( DarkBlue, Red, Pink),   //D_Blue and Red faces share corner #3
    ColorPiece( Red, DarkGreen, Beige),
    ColorPiece( DarkGreen, Purple, LightBlue),
    ColorPiece( Purple, Yellow, Orange),
    ColorPiece( Yellow, DarkBlue, LightGreen),
    // 10 - 14
    ColorPiece( LightBlue, Beige, DarkGreen),  //L_Blue and Beige faces share corner #2
    ColorPiece( Beige, Pink, Red),
    ColorPiece( Pink, LightGreen, DarkBlue),
    ColorPiece( LightGreen, Orange, Yellow),
    ColorPiece( Orange, LightBlue, Purple),
    // 15 - 19
    ColorPiece( Gray, Beige, LightBlue),
    ColorPiece( Gray, LightBlue, Orange),
    ColorPiece( Gray, Orange, LightGreen),
    ColorPiece( Gray, LightGreen, Pink),
    ColorPiece( Gray, Pink, Beige)
];

//scaffolding struct that holds relative position/direction color info, for g_faceNeighbors below
struct ColorDirection {
    //order: Start from front face, then 9-oclock and going CW right around and down
    front: MegaminxColor,
    left: MegaminxColor,
    up: MegaminxColor,
    right: MegaminxColor,
    downr: MegaminxColor,
    downl: MegaminxColor,
    bottom: MegaminxColor
}
//TupleStruct
struct ColorDirs([MegaminxColor; 7]);

//Defines which faces are touching each other. Entire relational color map.
// For Human Algo Rotate., used by RotateAlgo, ParseAlgorithmID and param to ParseAlgorithmString
static G_FACENEIGHBORS: [ColorDirs; MAX_COLOR_STATES] = [
// #case converted with https://www.better-converter.com/Case-Converters/Camel-Case-Converter
    //initializeThe0SlotToBlackInvalid
    ColorDirs([ Black, Black, Black, Black, Black, Black, Black ]),
    //Bottom/White1-6
    ColorDirs([ White, DarkBlue, Red, DarkGreen, Purple, Yellow, Orange ]),
    ColorDirs([ DarkBlue, Red, White, Yellow, LightGreen, Pink, Gray ]),
    ColorDirs([ Red, DarkGreen, White, DarkBlue, Pink, Beige, Gray ]),
    ColorDirs([ DarkGreen, Purple, White, Red, Beige, LightBlue, Gray ]),
    ColorDirs([ Purple, Yellow, White, DarkGreen, LightBlue, Orange, Gray ]),
    ColorDirs([ Yellow, DarkBlue, White, Purple, Orange, LightGreen, Gray ]),
    //Top/Gray7-12
    ColorDirs([ Gray, Pink, LightGreen, Orange, LightBlue, Beige, DarkGreen ]),
    ColorDirs([ LightBlue, Beige, Gray, Orange, Purple, DarkGreen, White ]),
    ColorDirs([ Orange, LightBlue, Gray, LightGreen, Yellow, Purple, White ]),
    ColorDirs([ LightGreen, Orange, Gray, Pink, DarkBlue, Yellow, White ]),
    ColorDirs([ Pink, LightGreen, Gray, Beige, Red, DarkBlue, White ]),
    ColorDirs([ Beige, Pink, Gray, LightBlue, DarkGreen, Red, White ])
];

//Determine which direction those faces need to rotate to land the Edge on the white
struct RotationDirs([i16; 5]);

//Decides which direction, up or down, for the pieces to passively float to their original home
//Spatial awareness vision introspection
static DIRTOWHITEFACE: [RotationDirs; MAX_COLOR_STATES] = [
    RotationDirs( [ 0, 0, 0, 0, 0 ]),
    RotationDirs( [ 0, 0, 0, 0, 0 ]),
    RotationDirs( [ -1, -1, 1, 1, -1 ]),  //e2&3 swapped @ D.Blue
    RotationDirs( [ -1, 1, -1, 1, -1 ]),
    RotationDirs( [ -1, 1, -1, 1, -1 ]),
    RotationDirs( [ -1, 1, -1, 1, -1 ]),
    RotationDirs( [ -1, 1, -1, 1, -1 ]),
    RotationDirs( [ 0, 0, 0, 0, 0 ]),
    RotationDirs( [ -1, 1, 1, -1, -1 ]),
    RotationDirs( [ -1, 1, 1, -1, -1 ]),
    RotationDirs( [ -1, 1, 1, -1, -1 ]),
    RotationDirs( [ -1, 1, -1, 1, -1 ]), //e3&4 swapped @ Pink
    RotationDirs( [ -1, 1, 1, -1, -1 ])
];

//edge self-solve bitmap ( [introspection])
//Edges that have their solved-Face-Color in the color[1] index. ( [marked by 1's]),
//                                instead of the color[0] index. ( [marked by 0's])
//( [Which half of the edge would the solved face be on])
//these bits must be set, equal to the color data index, to consider as solved
static EDGESOLVEFACEBITMAP: [RotationDirs; MAX_COLOR_STATES] = [
    RotationDirs( [ 0, 0, 0, 0, 0 ]),
    RotationDirs( [ 0, 0, 0, 0, 0 ]),
    RotationDirs( [ 1, 0, 1, 0, 0 ]),
    RotationDirs( [ 1, 1, 0, 0, 0 ]),
    RotationDirs( [ 1, 1, 0, 0, 0 ]),
    RotationDirs( [ 1, 1, 0, 0, 0 ]),
    RotationDirs( [ 1, 1, 0, 0, 0 ]),
    RotationDirs( [ 0, 0, 0, 0, 0 ]),
    RotationDirs( [ 1, 1, 1, 0, 1 ]),
    RotationDirs( [ 1, 1, 1, 0, 1 ]),
    RotationDirs( [ 1, 1, 1, 0, 1 ]),
    RotationDirs( [ 1, 1, 0, 1, 1 ]),
    RotationDirs( [ 1, 1, 1, 0, 1 ])
];
