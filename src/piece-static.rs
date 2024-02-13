// Megaminx-rs/piece-static.rs - LICENSE - AGPL3 - genr8eofl @ genBTC - for megaminx-rs (2024)
mod PieceMath {
  use std::f32::consts::PI;

  fn PieceMath() {
	//default size in 3D coords for main megaminx
	let DODESIZE: f32 = 100f32;
	//common geometric constants
//	let PI: f32 = (-1f32).acos();           	 //3.1415927410125732
	//Golden Ratio (Phi) (also The ratio between the side length of a regular pentagon and one of its diagonals.)
	let PHI: f32 = (1. + (5f32).sqrt()) / 2f32;  //1.6180340051651001
	let SIDE_ANGLE: f32 = 2. * PHI.atan(); 		 //2.0344439448698051
	//inscribed sphere radius ( ri: f32 = a / 2 * √ ( 25 + 11 * √5 ) / 10 )
	let INS_SPHERE_RAD: f32 = DODESIZE * (10. + 22. / (5f32).sqrt()).sqrt() / 4.;   //111.35163307189941
	let INS_CIRCLE_RAD: f32 = DODESIZE / ((5. - (5f32).sqrt()) / 2.).sqrt();     // 85.065082037033278
	fn pim(x: f32) -> f32 {	x*PI/5f32 }
	//megaminx vertex math shortcuts
	let TWOFIFTHS: f32 = 2./5.;
	let EDGEFIFTH: f32 = DODESIZE / pim(2.).sin();           //105.14622122913930
	let COSPIM35: f32 = INS_CIRCLE_RAD * pim(3.5).cos();     //-50.000004917867173
	let COSPIM15: f32 = INS_CIRCLE_RAD * pim(1.5).cos();     //49.999998901510480
	let SINPIM35: f32 = INS_CIRCLE_RAD * pim(3.5).sin();     //68.819093936061520
  }
}
