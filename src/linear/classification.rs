use rand::{Rng};
use rand;

use linear::miscelanous::*;
use std;

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

#[no_mangle]
pub extern fn weights_training(weights: *mut[f64; 3], raw_data_set: *mut std::os::raw::c_void, nb_points: u64) -> *mut[f64; 3]{
    let nb = 1000000;
    let step:f64 = 0.01;
    let data_set = import_external(raw_data_set, (nb_points * 3) as usize);

    let mut points : Vec<Point> = Vec::new();
    for i in 0..nb_points{
        let current = i * 3;
        let tmp : [f64;3] = [(*data_set)[current as usize], (*data_set)[(current + 1) as usize], (*data_set)[(current + 2) as usize]];
        points.push(convert_to_point(&tmp));
    }

    for _i in 0..nb{
        for (_j, point) in points.iter().enumerate() {
            let row = convert_to_raw_data(point);
            unsafe{
                let prediction = predict(point, *weights);
                let error = (row[1] - prediction) as f64;
                (*weights)[0] = (*weights)[0] + step * error;
                (*weights)[1] = (*weights)[1] + step * error * row[0];
                (*weights)[2] = (*weights)[2] + step * error * row[2];
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
