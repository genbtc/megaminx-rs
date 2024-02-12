// Megaminx-rs/piece.rs - LICENSE - AGPL3 - genr8eofl @ genBTC - for megaminx-rs (2024)
 
// Piece data-members we can swap out all at once
struct PieceData {
    _color: [f32; 9],
    _colorNum: [i8; 3],
    _colorName: [&'static str; 3],
    pieceNum: i8,
    flipStatus: i8,
    hotPieceMoving: bool,
}

// Piece struct
struct Piece {                                                                                                                                     
    //Coords for GL vertex (up to 7, not all used)
    vertex: [f32; 21], // = {};
    //Keeps the default number in the piece. do not swap.
    _defaultPieceNum: i8,
    //Center has 1, Edge has 2, Corner has 3
    numSides: i8,
	//Data Struct
	data: PieceData,
}
