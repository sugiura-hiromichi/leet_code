const DT: f64 = 0.05;
const MAX_STEP: i32 = 1000;
const M: f64 = 1.0;
const K: f64 = 1.0;
//const GAMMA: f64 = 0.0;

fn fx(v: f64) -> f64 {
   v
}

fn fv(x: f64) -> f64 {
   -K * x / M
}

fn main() {
   let mut x = 1.0;
   let mut v = 0.0;

   for step in 0..MAX_STEP {
      println!("{} {x} {v}", step as f64 * DT);

      x += fx(v) * DT;
      v += fv(x) * DT;
   }
}
