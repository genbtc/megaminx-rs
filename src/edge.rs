//2024 megaminx-rs edge.rs , by genr8eofl - LICENSED APGL3
pub mod edge {
  use crate::piece::piece::*;
  use crate::piece_color::PieceColor::G_EDGEPIECESCOLORS;
  //Edge functions
  pub trait Edge {
    fn init(&mut self, piecenum: usize, do_axes: bool);
    fn render(&self) -> Vec<VertexPositionColor>;
    fn render_lines(&self, n: i8) -> Vec<VertexPositionColor>;
    fn flip_twice(&mut self);
    fn flip(&mut self);
    fn getself(&self) -> &Self where Self: Sized; //so it does not apply to trait objects. returns back a Piece ref
    //fn getdata(&self) -> &PieceData;    
  }
  impl Edge for Piece {
    fn getself(&self) -> &Piece {
        self
    }    
//    fn getdata(&self) -> &PieceData {
//        &self.data
//    }//for faces.rs:swap_pieces()
    /**
     * \brief Inits a EdgeCorner piece
     * \note  (calls createAxis and initColor)
     * \param n the number of the EdgeCorner piece (piecenum)
     * \param doAxes True by default. First Time Initialization Only
     */
    fn init(&mut self, piecenum: usize, do_axes: bool) {
        self.new();
        if do_axes {
            for i in 0..6 {
                self.create_edge_axis(piecenum, i);
            }
        }
        self.initColor(G_EDGEPIECESCOLORS[piecenum], false);
        self.data.pieceNum = piecenum;
        self.defaultPieceNum = piecenum;
        self.points.new(self.vertex);
    }    
    /**
     * \brief Render Edge Node (CONST)
     */
    fn render(&self) -> Vec<VertexPositionColor> {
        //println!("DEBUG Edge[{}] self.vertex {:?}", self.defaultPieceNum, self.vertex);
        vec![
            VertexPositionColor { position: self.vertex[0], color: self.data.color.colorRGB[0] },
            VertexPositionColor { position: self.vertex[1], color: self.data.color.colorRGB[0] },
            VertexPositionColor { position: self.vertex[2], color: self.data.color.colorRGB[0] }, //tri1
            VertexPositionColor { position: self.vertex[0], color: self.data.color.colorRGB[0] },
            VertexPositionColor { position: self.vertex[2], color: self.data.color.colorRGB[0] },
            VertexPositionColor { position: self.vertex[3], color: self.data.color.colorRGB[0] }, //tri2
            VertexPositionColor { position: self.vertex[2], color: self.data.color.colorRGB[1] }, 
            VertexPositionColor { position: self.vertex[3], color: self.data.color.colorRGB[1] },
            VertexPositionColor { position: self.vertex[4], color: self.data.color.colorRGB[1] }, //tri3
            VertexPositionColor { position: self.vertex[5], color: self.data.color.colorRGB[1] }, 
            VertexPositionColor { position: self.vertex[4], color: self.data.color.colorRGB[1] },
            VertexPositionColor { position: self.vertex[2], color: self.data.color.colorRGB[1] }, //tri4
        ]
    }
    fn render_lines(&self, n: i8) -> Vec<VertexPositionColor> {
        match n {
            0 => { 
                vec![
                    VertexPositionColor { position: self.vertex[0], color: VERTEXZERO }, //black line
                    VertexPositionColor { position: self.vertex[1], color: VERTEXZERO },
                    VertexPositionColor { position: self.vertex[2], color: VERTEXZERO },
                    VertexPositionColor { position: self.vertex[3], color: VERTEXZERO }, //loop1
                ]
            },  //(Intersection Line is at 2/3)
            1 => {
                vec![
                    VertexPositionColor { position: self.vertex[2], color: VERTEXZERO }, //black line
                    VertexPositionColor { position: self.vertex[3], color: VERTEXZERO },
                    VertexPositionColor { position: self.vertex[4], color: VERTEXZERO },
                    VertexPositionColor { position: self.vertex[5], color: VERTEXZERO }, //loop2
                ]
            },
            _=> { Vec::<VertexPositionColor>::new() },
        }
    }
    //Flip - Changes colors. rotate/switches colors for current piece
    fn flip(&mut self) {
        self.data.color.colorRGB[0].rotate_left(3);
        self.data.color.colorNum.rotate_left(1);
        self.data.color.colorName.rotate_left(1);
        if self.data.flipStatus == 0 {
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
  }
}
