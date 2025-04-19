




fn func(inp: f32) -> f32{
    return 0.5*inp //placeholder for abs and exp
}

fn metropolis_hasting(x_0: f32, n: i32, s: f32) -> Vec<f32>{
    let mut x_values: Vec<f32> = vec![x_0];
    let mut accepted: i32 = 0;
    for _ in 1..n{
        let xi_minus1: f32  = *x_values.last().unwrap();
        let x_star: f32 = 3.0; //placeholder for random normal

        let r: f32 = func(x_star) / func(xi_minus1);
        let u: f32 = 1.0; //placeholder for random uniform

        if u < r{
            x_values.push(x_star);
            accepted += 1;
        } 
        else {
            x_values.push(xi_minus1);    
        }
    }
    return x_values;
}

fn multi_metropolis_hasting(j: i32, x_0: f32, n: i32, s: f32) -> Vec<f32>{
    let mut x_values2: Vec<f32> = vec![x_0];
    for _ in 1..j{
        let x_value: f32 = metropolis_hasting(0.0, 2000, 0.001)[0];
        x_values2.push(x_value);
    }
    return x_values2
}

fn main() {
    println!("Hello, world!");
}
