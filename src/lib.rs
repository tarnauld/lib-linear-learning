#![feature(rustc_private)]
extern crate rand;
extern crate nalgebra;
use nalgebra::core::dimension::Dynamic as Dynamic;
use nalgebra::core::MatrixMN as MatrixMN;

use rand::{Rng};

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

#[no_mangle]
pub unsafe extern fn classify(row :*mut [f64; 2], w:*mut [f64; 3]) -> f64{
    let point = convert_to_point(&[(*row)[0], 0.0, (*row)[1]]);
	predict(&point, *w)
}

fn predict(row : &Point, w: [f64; 3]) -> f64{
	let mut activation = w[0];

	activation += w[1] * row.x;
    activation += w[2] * row.z;

	if activation >= 0.0 {
		return 1.0;
	}
	return -1.0;
}

fn export_external(w: &[f64]) -> *mut std::os::raw::c_void{
    Box::into_raw(Box::new(w)) as *mut std::os::raw::c_void
}

fn import_external<'a>(array: *mut std::os::raw::c_void, length : u32) -> &'a[f64] {
    unsafe {
        std::slice::from_raw_parts(array as *mut f64, length as usize)
    }
}

#[no_mangle]
pub extern fn weights_training(weights: *mut[f64; 3], data_set: *mut [f64; 9]) -> *mut[f64; 3]{
    let nb = 10000000;
    let step:f64 = 0.01;

    let mut points : Vec<Point> = Vec::new();
    let mut i = 0;
    unsafe{
        loop{
            if i == 9{break;}
            let mut tmp : [f64;3] = [(*data_set)[i as usize], (*data_set)[(i + 1) as usize], (*data_set)[(i + 2) as usize]];
            points.push(convert_to_point(&tmp));
            i += 3;
        }
    }

    for i in 0..nb{
        for (j, point) in points.iter().enumerate() {
            let row = convert_to_raw_data(point);
            unsafe{
                let prediction = predict(point, *weights);
                let error = (row[1] - prediction) as f64;
                (*weights)[0] = (*weights)[0] + step * error;
                (*weights)[1] = (*weights)[1] + step * error * row[0];
                (*weights)[2] = (*weights)[2] + step * error * row[2];
                // for k in 0..row.len() - 1{
                //     (*weights)[k + 1] = (*weights)[k + 1] + step * error * row[k];
                // }
            }
        }
    }
    weights
}

#[no_mangle]
pub unsafe extern "C" fn generate_weight() -> *mut[f64; 3]{
	let mut w: [f64; 3] = [0.0; 3];

    for i in 0..3{
        w[i as usize] = rand::thread_rng().gen_range(-1., 1.);
    }
   	Box::into_raw(Box::new(w))
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

// ((X^tX)^-1Xt)Y
fn calculate_weights(points : &[Point]) -> [f64;10] {
    let mut res : [f64;10] = [0.0 as f64;10];
    return res;
}

#[no_mangle]
pub extern fn linear_regression(step: f64, p : &[f64], dim: u64, nb: u64) -> bool{

    let m = Dynamic::new(nb as usize);
    let n = Dynamic::new(dim as usize);
    let matrix : MatrixMN<f64, Dynamic, Dynamic> = MatrixMN::from_row_slice_generic(m, n, p);
    let inversedMatrix = matrix.pseudo_inverse(1e-9);


    //let w = calculate_weights(points.as_mut_slice());

    return true;
}



/*#[test]
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
}*/
