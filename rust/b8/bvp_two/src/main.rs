mod bvp {
    const EPSILON: f64 = 3.0;
    const I: f64 = -0.4;
    const A: f64 = 0.7;
    const B: f64 = 0.8;
    const K: f64 = 0.5; // static enables K to be mutable???

    //fx=dx/dt
    pub fn fx(x1: f64, x2: f64, v1: f64) -> f64 {
        EPSILON * (v1 + x1 - x1 * x1 * x1 / 3.0 + I) + K * (x2 - x1)
    }

    //fv=dv/dt
    pub fn fv(x1: f64, v1: f64) -> f64 {
        (x1 - A + B * v1) / EPSILON
    }
}

fn main() {
    use bvp::{fv, fx};
    let dt = 0.05;
    let mut x1 = vec![1.5, 0.0, 0.0, 0.0, 0.0];
    let mut v1 = vec![0.2, 0.0, 0.0, 0.0, 0.0];
    let mut x2 = vec![-1.2, 0.0, 0.0, 0.0, 0.0];
    let mut v2 = vec![1.0, 0.0, 0.0, 0.0, 0.0];

    let max_step = 1000;
    for step in 0..max_step {
        println!(
            "{} {} {} {} {}",
            step as f64 * dt,
            x1[0],
            v1[0],
            x2[0],
            v2[0]
        );

        x1[1] = fx(x1[0], x2[0], v1[0]) * dt;
        x1[2] = fx(x1[0], x2[0], v1[0] + v1[1] / 2.0) * dt;
        x1[3] = fx(x1[0], x2[0], v1[0] + v1[2] / 2.0) * dt;
        x1[4] = fx(x1[0], x2[0], v1[0] + v1[3]) * dt;

        v1[1] = fv(x1[0], v1[0]) * dt;
        v1[2] = fv(x1[0] + x1[1] / 2.0, v1[0]) * dt;
        v1[3] = fv(x1[0] + x1[2] / 2.0, v1[0]) * dt;
        v1[4] = fv(x1[0] + x1[3], v1[0]) * dt;

        x2[1] = fx(x2[0], x1[0], v2[0]) * dt;
        x2[2] = fx(x2[0], x1[0], v2[0] + v2[2] / 2.0) * dt;
        x2[3] = fx(x2[0], x1[0], v2[0] + v2[2] / 2.0) * dt;
        x2[4] = fx(x2[0], x1[0], v2[0] + v2[3]) * dt;

        v2[1] = fv(x2[0], v2[0]) * dt;
        v2[2] = fv(x2[0] + x2[1] / 2.0, v2[0]) * dt;
        v2[3] = fv(x2[0] + x2[2] / 2.0, v2[0]) * dt;
        v2[4] = fv(x2[0] + x2[3], v2[0]) * dt;

        //insert result
        x1[0] += (x1[1] + x1[2] * 2.0 + x1[3] * 2.0 + x1[4]) / 6.0;
        v1[0] += (v1[1] + v1[2] * 2.0 + v1[3] * 2.0 + v1[4]) / 6.0;

        x2[0] += (x2[1] + x2[2] * 2.0 + x2[3] * 2.0 + x2[4]) / 6.0;
        v2[0] += (v2[1] + v2[2] * 2.0 + v2[3] * 2.0 + v2[4]) / 6.0;
    }
}
