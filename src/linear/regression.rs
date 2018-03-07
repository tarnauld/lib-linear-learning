use nalgebra::core::DMatrix as DMatrix;
use linear::miscelanous::*;
use std;

#[no_mangle]
pub unsafe extern fn regress_point(weights: *mut [f64; 3], point: [f64; 2]) -> f64 {
    point[0] * (*weights)[1] + point[1] * (*weights)[2] + (*weights)[0]
}

#[no_mangle]
pub extern fn linear_regression(raw_points : *mut std::os::raw::c_void,
                                raw_results : *mut std::os::raw::c_void,
                                dim: u64,
                                nb_elements: u64) -> *mut [f64; 3] {
    let results = import_external(raw_results, nb_elements as usize);
    let points : &[f64] = import_external(raw_points, (dim * nb_elements) as usize);
    let mut weights : Vec<f64> = Vec::new();
    let matrix_data = prepare_matrix_data(points, nb_elements as usize);
    let matrix = DMatrix::from_row_slice(dim as usize, nb_elements as usize, matrix_data.as_slice());
    let result_matrix = DMatrix::from_row_slice(1 as usize, nb_elements as usize, results);
    let transposed_matrix = matrix.transpose();
    let square_matrix = matrix.clone() * transposed_matrix.clone();
    let inverted_matrix = square_matrix.pseudo_inverse(1e-19);
    let verification_matrix = transposed_matrix * inverted_matrix;
    let result = result_matrix * verification_matrix;

    let calculated_weights = result.data.data();

    for i in 0..calculated_weights.len() {
        weights.push(calculated_weights[i]);
    }
    let w: [f64; 3] = [weights[0], weights[1], weights[2]];
    Box::into_raw(Box::new(w))
}

fn prepare_matrix_data(points: &[f64], nb_elements: usize) -> Vec<f64> {
    let mut points_vector = vec![];

    for _ in 0..nb_elements {
        points_vector.push(1.0);
    }
    for i in 0..nb_elements {
        points_vector.push(points[i*3]);
    }
    for i in 0..nb_elements {
        points_vector.push(points[i*3+2]);
    }
    points_vector
}
