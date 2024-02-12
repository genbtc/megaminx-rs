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

//scaffolding Struct, a shell for a color, for g_colorRGBs
struct ColorPack {
	i: u16,
	r: f64,
    g: f64,
    b: f64,
	name: String
}

//scaffolding Struct, a shell for a piece, holds 1-3 Colors for a Center/Edge/Corner definition
struct ColorPiece {
	a: MegaminxColor,
    b: MegaminxColor,
    c: MegaminxColor,
}

//scaffolding struct that holds relative position/direction color info, for g_faceNeighbors below
struct ColorDirs {
    //order: Start from front face, then 9-oclock and going CW right around and down
    front: MegaminxColor,
    left: MegaminxColor,
    up: MegaminxColor,
    right: MegaminxColor,
    downr: MegaminxColor,
    downl: MegaminxColor,
    bottom: MegaminxColor
}
