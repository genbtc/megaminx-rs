// megaminx-rs/piece_static.rs - LICENSE = AGPL3 - genr8eofl @ genbtc, (for megaminx-rs) (c) 2024

mod PieceStatic {
  use std::f32::consts::PI;

  fn math() {
	//default size in 3d coords for main megaminx
	let dodesize: f32 = 100f32;
	//common geometric constants
	let pi: f32 = (-1f32).acos();           	 //3.1415927410125732
	//golden ratio (phi) (also the ratio between the side length of a regular pentagon and one of its diagonals.)
	let phi: f32 = (1. + (5f32).sqrt()) / 2f32;  //1.6180340051651001
	let sideangle: f32 = 2. * phi.atan(); 		 //2.0344439448698051
	//inscribed sphere radius ( ri: f32 = a / 2 * √ ( 25 + 11 * √5 ) / 10 )
	let inssphererad: f32 = dodesize * (10. + 22. / (5f32).sqrt()).sqrt() / 4.;   //111.35163307189941
	let inscirclerad: f32 = dodesize / ((5. - (5f32).sqrt()) / 2.).sqrt();     // 85.065082037033278
	fn pim(x: f32) -> f32 {	x*PI/5f32 }
	//megaminx vertex math shortcuts
	let twofifths: f32 = 2./5.;
	let edgefifth: f32 = dodesize / pim(2.).sin();           //105.14622122913930
	let cospim35: f32 = inscirclerad * pim(3.5).cos();     //-50.000004917867173
	let cospim15: f32 = inscirclerad * pim(1.5).cos();     //49.999998901510480
	let sinpim35: f32 = inscirclerad * pim(3.5).sin();     //68.819093936061520
  }
}
