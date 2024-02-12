// Megaminx-rs/piece.rs - LICENSE - AGPL3 - genr8eofl @ genBTC - for megaminx-rs (2024)
 
// Piece data-members we can swap out all at once
struct PieceData {
    double _color[3][3];
    int _colorNum[3];
    const char* _colorName[3];
    int pieceNum;
    int flipStatus;
    //Highlight in bright green while algorithming (called in .render())
    bool hotPieceMoving = false;
};

// Piece struct
class Piece {                                                                                                                                     
    //Coords for GL vertex (up to 7, not all used)
    double _vertex[7][3] = {};
    //Keeps the default number in the piece. do not swap.
    int _defaultPieceNum;
    //Center has 1, Edge has 2, Corner has 3
    int numSides;
	//Data Struct
	PieceData data;
};
