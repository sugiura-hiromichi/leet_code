#![allow(unused)]
use bvp::*;
const EPSILON: f64 = 3.0;
const DT: f64 = 0.05;
const MAX_STEP: usize = 1000;
const A: f64 = 0.7;
const B: f64 = 0.8;
const K: f64 = 0.3;
const NX: usize = 20;
const NY: usize = 20;
const I: f64 = -0.33;

mod bvp {
    use super::*;
    pub fn fx(x: f64, v: f64, xl: f64, xr: f64, xd: f64, xu: f64) -> f64 {
        // xl:left xr:right xd:down xu:up
        EPSILON * (v + x - x.powi(3) / 3.0 + I) + K * (x - xl - xr - xd - xu)
    }

    pub fn fv(x: f64, v: f64) -> f64 {
        -(x - A + B * v) / EPSILON
    }

    pub fn bo(n: i8, cap_n: usize) -> usize {
        if n < 0 {
            (n + cap_n as i8) as usize
        } else if n >= cap_n as i8 {
            n as usize - cap_n
        } else {
            n as usize
        }
    }
}

fn main() {
    let mut x = vec![vec![vec![0.0; NY]; NX]; 4];
    let mut v = vec![vec![vec![0.0; NY]; NX]; 4];
    x.insert(1, vec![vec![1.2; NY]; NX]);
    v.insert(1, vec![vec![-0.3; NY]; NX]);
    x[0][NX / 2][NY / 2] = 0.0;
    v[0][NX / 2][NY / 2] = -0.8;

    for step in 0..=MAX_STEP {
        for i in 0..NX {
            for j in 0..NY {
                println!(
                    "{} {} {} {} {}",
                    step as f64 * DT,
                    i as f64 / NX as f64,
                    j as f64 / NY as f64,
                    x[0][i][j],
                    v[0][i][j]
                );
            }
            print!("\n");
        }

        for n in 1..=4 {
            for i in 0..NX {
                for j in 0..NY {
                    let rk = (n / 2) as f64 * 0.5; // runge_kutta's coefficient
                    let ia = bo(i as i8 + 1, NX); // ia=i_add
                    let is = bo(i as i8 - 1, NX); // is=i_sub
                    let ja = bo(j as i8 + 1, NY); // ja=j_add
                    let js = bo(j as i8 - 1, NY); // js=j_sub
                    x[n][i][j] = DT
                        * fx(
                            x[0][i][j] + rk * x[n - 1][i][j],
                            v[0][i][j] + rk * v[n - 1][i][j],
                            x[0][ia][j] + rk * x[n - 1][ia][j],
                            x[0][is][j] + rk * x[n - 1][is][j],
                            x[0][i][ja] + rk * x[n - 1][i][ja],
                            x[0][i][js] + rk * x[n - 1][i][js],
                        );
                    v[n][i][j] = DT
                        * fv(
                            x[0][i][j] + rk * x[n - 1][i][j],
                            v[0][i][j] + rk * v[n - 1][i][j],
                        )
                }
            }
        }

        for i in 0..NX {
            for j in 0..NY {
                x[0][i][j] += (x[1][i][j] + x[2][i][j] * 2.0 + x[3][i][j] * 2.0 + x[4][i][j]) / 6.0;
                v[0][i][j] += (v[1][i][j] + v[2][i][j] * 2.0 + v[3][i][j] * 2.0 + v[4][i][j]) / 6.0;
            }
        }
    }
}
