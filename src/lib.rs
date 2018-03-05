extern crate rand;

struct Point {
    x: f64,
    y: f64,
    z: f64
}

#[no_mangle]
pub extern fn linear_learning(step: f64, points : &[f64], dim: u64, nb: u64) -> bool{
    let mut w: [f64; 10] = [0.0; 10];

    for i in 0..dim{
        let x = rand::random::<f64>();
        w[i as usize] = (x * 2.0) - 1.0;
        println!("{}", w[i as usize]);
    }

    return true;
}

#[test]
fn shouldGenerateRandomNumber(){
   assert!(linear_learning(0.1, &[0.0; 10], 10, 10));
}
