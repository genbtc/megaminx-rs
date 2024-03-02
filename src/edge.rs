//2024 megaminx-rs edge.rs , by genr8eofl - LICENSED APGL3
pub mod edge {
  use crate::piece::piece::EdgeCornerMath;
  use crate::piece::piece::EdgeCornerInit;
  use crate::piece::piece::Piece;
  use crate::piece::piece::VertexPositionColor;
  //Edge functions
  pub trait Edge : EdgeCornerInit {
    fn render(&self) -> Vec<VertexPositionColor>;
  }
  impl Edge for Piece  {
    fn render(&self) -> Vec<VertexPositionColor> {
        EdgeCornerMath::render(self)
    }    
  }
}
