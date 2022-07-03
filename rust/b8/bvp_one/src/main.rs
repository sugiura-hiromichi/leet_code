mod bvp {
    const EPSILON: f64 = 3.0;
    const I: f64 = 0.0;
    const A: f64 = 0.7;
    const B: f64 = 0.8;

    ///fx=dx/dt
    pub fn fx(x: f64, v: f64) -> f64 {
        EPSILON * (v + x - x * x * x / 3.0 + I)
    }

    ///fv=dv/dt
    pub fn fv(x: f64, v: f64) -> f64 {
        -(x - A + B * v) / EPSILON
    }
}

fn main() {
    use bvp::{fv, fx};
    let dt = 0.05;
    let mut x = vec![1.0, 0.0, 0.0, 0.0, 0.0];
    let mut v = vec![0.0, 0.0, 0.0, 0.0, 0.0];

    let max_step = 1000;
    for step in 0..max_step {
        println!("{} {} {}", step as f64 * dt, x[0], v[0]);

        x[1] = fx(x[0], v[0]) * dt;
        x[2] = fx(x[0], v[0] + v[1] / 2.0) * dt;
        x[3] = fx(x[0], v[0] + v[2] / 2.0) * dt;
        x[4] = fx(x[0], v[0] + v[3]) * dt;

        v[1] = fv(x[0], v[0]) * dt;
        v[2] = fv(x[0] + x[1] / 2.0, v[0]) * dt;
        v[3] = fv(x[0] + x[2] / 2.0, v[0]) * dt;
        v[4] = fv(x[0] + x[3], v[0]) * dt;

        x[0] += (x[1] + x[2] * 2.0 + x[3] * 2.0 + x[4]) / 6.0;
        v[0] += (v[1] + v[2] * 2.0 + v[3] * 2.0 + v[4]) / 6.0;
    }
}
