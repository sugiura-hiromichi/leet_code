use crate::bvp::{bo, fv, fx};

const EPSILON: f64 = 3.0;
const DT: f64 = 0.05;
const MAX_STEP: usize = 1000;
const A: f64 = 0.7;
const B: f64 = 0.8;
const K: f64 = 0.5;
const N: usize = 20;
const I: f64 = -0.33;

mod bvp {
    use super::*;
    /// xl==left side osc, xr==right side osc
    pub fn fx(x: f64, v: f64, xl: f64, xr: f64) -> f64 {
        EPSILON * (v + x - x.powi(3) / 3.0 + I) + K * (x - xl - xr)
    }

    /// check bvp_* crates
    pub fn fv(x: f64, v: f64) -> f64 {
        -(x - A + B * v) / EPSILON
    }

    pub fn bo(n: i8) -> usize {
        if n < 0 {
            (n + N as i8) as usize
        } else if n >= N as i8 {
            n as usize - N
        } else {
            n as usize
        }
    }
}

fn main() {
    let mut x = Vec::new();
    let mut xn1 = vec![0.0; N];
    let mut xn2 = vec![0.0; N];
    let mut xn3 = vec![0.0; N];
    let mut xn4 = vec![0.0; N];

    let mut v = Vec::new();
    let mut vn1 = vec![0.0; N];
    let mut vn2 = vec![0.0; N];
    let mut vn3 = vec![0.0; N];
    let mut vn4 = vec![0.0; N];

    for _i in 0..N {
        x.push(1.2);
        v.push(-0.3);
    }

    x[(N / 2) as usize] = 0.0;
    v[(N / 2) as usize] = -0.8;

    for step in 0..=MAX_STEP {
        /*        print!("{} {N} ", step as f64 * DT);
                for i in 0..N {
                    print!("{} ", x.get(i).unwrap());
                }
                for i in 0..N {
                    print!("{} ", v.get(i).unwrap());
                }
                println!("");
        */
        for i in 0..N {
            xn1[i] = DT * fx(x[i], v[i], x[bo(i as i8 - 1)], x[bo(i as i8 + 1)]);
            vn1[i] = DT * fv(x[i], v[i]);
        }

        for i in 0..N {
            xn2[i] = DT
                * fx(
                    x[i] + xn1[i] / 2.0,
                    v[i] + vn1[i] / 2.0,
                    x[bo(i as i8 - 1)] + xn1[bo(i as i8 - 1)] / 2.0,
                    x[bo(i as i8 + 1)] + xn1[bo(i as i8 + 1)] / 2.0,
                );
            vn2[i] = DT * fv(x[i] + xn1[i] / 2.0, v[i] + vn1[i] / 2.0);
        }

        for i in 0..N {
            xn3[i] = DT
                * fx(
                    x[i] + xn2[i] / 2.0,
                    v[i] + vn2[i] / 2.0,
                    x[bo(i as i8 - 1)] + xn2[bo(i as i8 - 1)] / 2.0,
                    x[bo(i as i8 + 1)] + xn2[bo(i as i8 + 1)] / 2.0,
                );
            vn3[i] = DT * fv(x[i] + xn2[i] / 2.0, v[i] + vn2[i] / 2.0);
        }

        for i in 0..N {
            xn4[i] = DT
                * fx(
                    x[i] + xn3[i],
                    v[i] + vn3[i],
                    x[bo(i as i8 - 1)] + xn3[bo(i as i8 - 1)],
                    x[bo(i as i8 + 1)] + xn3[bo(i as i8 + 1)],
                );
            vn4[i] = DT * fv(x[i] + xn3[i], v[i] + vn3[i]);
        }

        for i in 0..N {
            x[i] += (xn1[i] + xn2[i] * 2.0 + xn3[i] * 2.0 + xn4[i]) / 6.0;
            v[i] += (vn1[i] + vn2[i] * 2.0 + vn3[i] * 2.0 + vn4[i]) / 6.0;
        }
    }
}
