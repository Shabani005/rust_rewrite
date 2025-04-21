#[allow(unused_imports)]
#[allow(dead_code)]
use statrs::distribution::{Normal, Continuous, Uniform};
use statrs::statistics::{Distribution, Statistics};
use rand::{rng, thread_rng, Rng};
use math::*;
mod math;


fn func(inp: f64) -> f64{
    return 0.5*(-custom_exp(abs_f64(inp)))//placeholder for abs and exp
}

fn metropolis_hasting(x_0: f64, n: i32, s: f64) -> Vec<f64>{
    let mut x_values: Vec<f64> = vec![x_0];
    let mut accepted: i32 = 0;
    for _ in 1..n{
        let xi_minus1: f64  = *x_values.last().unwrap();
        
        let n = Normal::new(xi_minus1, s).unwrap();
        let x_star: f64 = n.pdf(1.0);

        let r: f64 = func(x_star) / func(xi_minus1);
        let uni = Uniform::new(0.0, 1.0).unwrap();
        let u: f64 = uni.pdf(1.0);
        let mut rng = rng();
        let gg: f64 = 1.0; // u will be a random uniform number between 0.0 and 1.0

        if gg < r{
            x_values.push(x_star);
            accepted += 1;
        } 
        else {
            x_values.push(xi_minus1);    
        }
    }
    return x_values;
}

fn multi_metropolis_hasting(j: i32, x_0: f64, n: i32, s: f64) -> Vec<f64>{
    let mut x_values2: Vec<f64> = vec![x_0];
    for _ in 1..j{
        let x_value: f64 = metropolis_hasting(0.0, 2000, 0.001)[0];
        x_values2.push(x_value);
    }
    return x_values2
}

fn main() {
    let x_values:  Vec<f64> = metropolis_hasting(0.0, 10000, 1.0);
    let x_mean: f64 = x_values.mean();
    println!("{:?}", x_mean)
}
