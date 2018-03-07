extern crate rand;

struct Point {
    x: f64,
    y: f64,
    z: f64
}

fn convert_to_point(row: &[f64; 3]) -> Point{
    Point {x: row[0], y: row[1], z: row[2]}
}

fn convert_to_raw_data(point: &Point) -> [f64; 3]{
    [
        point.x,
        point.y,
        point.z
    ]
}

fn predict(row : &Point, w: [f64; 4]) -> f64{
	let mut activation = w[0];

	activation += w[1] * row.x;
    activation += w[2] * row.y;
    activation += w[3] * row.z;

	if activation >= 0.0 {
		return 1.0;
	}
	return 0.0;
}

fn weights_training(data_set: &mut[Point], step: f64, nb: u32) -> [f64; 4]{
    let mut weights = generate_weigth();

    for i in 0..nb{
        for (i, point) in data_set.iter().enumerate() {
            let row = convert_to_raw_data(point);
            let prediction = predict(point, weights);
            let error = row[2] - prediction;
            weights[0] = weights[0] + step * error;
            for i in 0..row.len() - 1{
                weights[i + 1] = weights[i + 1] + step * error * row[i]
            }
        }
    }
    return weights;
}

fn generate_weigth() -> [f64; 4]{
	let mut w: [f64; 4] = [0.0; 4];

    for i in 0..4{
        let x = rand::random::<f64>();
        w[i as usize] = (x * 2.0) - 1.0;
    }
    return w;
}

fn convert_raw_data_set_to_point_dataset(p : &[f64], nb: u64) -> Vec<Point> {
    let mut points : Vec<Point> = Vec::new();
    let mut i = 0;

    while i < nb {
        let tmp : [f64;3] = [p[i as usize], p[(i+1) as usize], p[(i+2) as usize]];
        let mut point = convert_to_point(&tmp);
        points.push(point);
        i += 3;
    }
    points
}

#[no_mangle]
pub extern fn linear_classification(step: f64, p : &[f64], nb: u64) -> bool{
    let mut points = convert_raw_data_set_to_point_dataset(p, nb);

    weights_training(points.as_mut_slice(), step, 5);

    return true;
}

// ((X^tX)^-1Xt)Y
fn calculate_weights(points : &[Point]) -> [f64;10] {
    let mut res : [f64;10] = [0.0 as f64;10];
    return res;
}

#[no_mangle]
pub extern fn linear_regression(step: f64, p : &[f64], dim: u64, nb: u64) -> bool{
    let mut points = convert_raw_data_set_to_point_dataset(p, nb);

    let w = calculate_weights(points.as_mut_slice());

    return true;
}

#[test]
fn should_generate_random_weigths(){
    let mut w = generate_weigth();
    assert!(w.len() == 4);
}

#[test]
fn should_predict_correctly(){
	let data_set = [[2.7810836,2.550537003, -0.100000000],
	[1.465489372,2.362125076, -0.100000000],
	[3.396561688,4.400293529, -0.100000000],
	[1.38807019,1.850220317, -0.100000000],
	[3.06407232,3.005305973, -0.100000000],
	[7.627531214,2.759262235, -0.100000000],
	[5.332441248,2.088626775, -0.100000000],
	[6.922596716,1.77106367, -0.100000000],
	[8.675418651,-0.242068655, -0.100000000],
	[7.673756466,3.508563011, -0.100000000]];

    let mut p : Vec<Point>= Vec::new();

    for i in 0..data_set.len(){
        p.push(convert_to_point(&data_set[i]));
    }
    let mut w = weights_training(p.as_mut_slice(), 0.1, 15);
    for (i, val) in w.iter().enumerate(){
        println!("{:?}", val);
    }
}
