use std;
use linear::miscelanous::*;
use nalgebra::core::DMatrix as DMatrix;

#[no_mangle]
pub fn rbf_apply(gamma: f64, raw_solutions: *mut std::os::raw::c_void,
                    raw_inputs : *mut std::os::raw::c_void,
                    nb_elements: u64, dim: u64) -> *mut std::os::raw::c_void {
    let inputs = import_external(raw_inputs, (nb_elements * dim) as usize);
    let solutions = import_external(raw_solutions, nb_elements as usize);

    let weights = safe_rbf_apply(gamma, solutions, inputs, nb_elements, dim);
    export_external(weights.as_slice())
}

fn lloyd_algorithm(inputs: &[f64], expected_clusters: u64) -> &[f64] {
    &[1.]
}

fn safe_rbf_apply(gamma: f64, solutions: &[f64],
                    inputs : &[f64],
                    nb_elements: u64, dim: u64) -> Vec<f64>{
    let mut matrix : Vec<f64> = Vec::new();

    for i in 0..nb_elements {
        for j in 0..nb_elements {
            let dist = inputs[(i + j * nb_elements) as usize];
            let value = -gamma * dist.powi(2);
            matrix.push(value.exp());
        }
    }
    let output = DMatrix::from_row_slice(nb_elements as usize, nb_elements as usize, matrix.as_slice());
    let solutions_matrix = DMatrix::from_row_slice(1 as usize, nb_elements as usize, solutions);
    let result = solutions_matrix * output.pseudo_inverse(1e-9);
    let mut weight_vector : Vec<f64> = Vec::new();
    let calculated_weights = result.data.data();

    for i in 0..dim {
        weight_vector.push(calculated_weights[i as usize]);
    }

    weight_vector
}

#[test]
fn should_work() {
    let solutions = [1., 1., 1.];
    let inputs = [
                        50., 1., 1.,
                        2., 2., 2.,
                        3., 3., 3.
                    ];

    let s = safe_rbf_apply(0.1, &solutions, &inputs, 3, 3);
    println!("{}, {}, {}", s[0], s[1], s[2]);
}
