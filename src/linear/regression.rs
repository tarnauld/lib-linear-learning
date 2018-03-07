use nalgebra::core::dimension::Dynamic as Dynamic;
use nalgebra::core::MatrixMN as MatrixMN;
use miscelanous::*;

#[no_mangle]
pub extern fn linear_regression(raw_points : *mut std::os::raw::c_void,
                                raw_results : *mut std::os::raw::c_void,
                                dim: u64,
                                nb_elements: u64) -> *mut std::os::raw::c_void {

    let mut weights : Vec<f64> = Vec::new();
    let mut results = import_external(raw_results, nb_elements as usize);
    let mut points : &[f64] = import_external(raw_points, (dim * nb_elements) as usize);
    let m = Dynamic::new(nb_elements as usize);
    let n = Dynamic::new(dim as usize);
    let u1 = Dynamic::new(1 as usize);
    let matrix : MatrixMN<f64, Dynamic, Dynamic> = MatrixMN::from_row_slice_generic(m, n, points);
    let result_matrix : MatrixMN<f64, Dynamic, Dynamic> = MatrixMN::from_row_slice_generic(m, u1, points);
    let transposed_matrix = matrix.transpose();
    let square_matrix = matrix.clone() * transposed_matrix.clone();
    let inversed_matrix = square_matrix.try_inverse().unwrap();
    let verification_matrix = inversed_matrix * transposed_matrix;
    let result = verification_matrix * result_matrix;

    let calculated_weights = result.data.data();

    for i in 0..calculated_weights.len() {
        weights.push(calculated_weights[i]);
    }

    export_external(weights.as_mut_slice())
}
