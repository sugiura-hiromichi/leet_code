///fx=dx/dt
fn fx(v: f64) -> f64 {
   v
}

///fv=dv/dt
fn fv(x: f64) -> f64 {
   let k = 1.0;
   let m = 1.0;
   -k * x / m
}

fn main() {
   let dt = 0.05;
   let mut x = vec![1.0, 0.0, 0.0, 0.0, 0.0];
   let mut v = vec![0.0, 0.0, 0.0, 0.0, 0.0];

   let max_step = 1000;
   for step in 0..max_step {
      println!("{} {} {}", step as f64 * dt, x[0], v[0]);

      x[1] = fx(v[0]) * dt;
      x[2] = fx(v[0] + v[1] / 2.0) * dt;
      x[3] = fx(v[0] + v[2] / 2.0) * dt;
      x[4] = fx(v[0] + v[3]) * dt;

      v[1] = fv(x[0]) * dt;
      v[2] = fv(x[0] + x[1] / 2.0) * dt;
      v[3] = fv(x[0] + x[2] / 2.0) * dt;
      v[4] = fv(x[0] + x[3]) * dt;

      x[0] += (x[1] + x[2] * 2.0 + x[3] * 2.0 + x[4]) / 6.0;
      v[0] += (v[1] + v[2] * 2.0 + v[3] * 2.0 + v[4]) / 6.0;
   }
}
