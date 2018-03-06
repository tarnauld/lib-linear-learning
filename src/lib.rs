extern crate rand;

struct Point {
    x: f64,
    y: f64,
    z: f64
}

fn predict(row : [f64; 2], w: [f64; 10]) -> f64{
	println!("{0}:{1}", row[0], row[1]);
	let mut activation = w[0];

	for i in 0..row.len(){
		activation += w[i + 1] * row[i];
	}

	if activation >= 0.0 {
		return 1.0;
	}
	return 0.0;
}

fn generateWeigth(dim: u64) -> [f64; 10]{
	let mut w: [f64; 10] = [0.0; 10];

    for i in 0..dim{
        let x = rand::random::<f64>();
        w[i as usize] = (x * 2.0) - 1.0;
        println!("{}", w[i as usize]);
    }
    return w;
}

#[no_mangle]
pub extern fn linear_learning(step: f64, points : &[f64], dim: u64, nb: u64) -> bool{
    let w = generateWeigth(dim);

    return true;
}

#[test]
fn shouldGenerateRandomNumber(){
   assert!(linear_learning(0.1, &[0.0; 10], 10, 10));
}

#[test]
fn shouldPredictCorrectly(){
	let data_set = [[2.7810836,2.550537003],
	[1.465489372,2.362125076],
	[3.396561688,4.400293529],
	[1.38807019,1.850220317],
	[3.06407232,3.005305973],
	[7.627531214,2.759262235],
	[5.332441248,2.088626775],
	[6.922596716,1.77106367],
	[8.675418651,-0.242068655],
	[7.673756466,3.508563011]];

	let w = generateWeigth(10);

	for (i, row) in data_set.iter().enumerate(){
		predict(*row, w);
	}
}

#[test]
fn shouldGenerateArray(){
	let w = generateWeigth(10);
	assert!(w.len() == 10);
}