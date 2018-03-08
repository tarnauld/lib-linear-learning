use std;
use linear::miscelanous::*;
use nalgebra::core::DMatrix as DMatrix;
use rand;
use rand::{Rng};

#[no_mangle]
pub fn rbf_apply(expected_clusters: u64, gamma: f64, raw_solutions: *mut std::os::raw::c_void,
                    raw_inputs : *mut std::os::raw::c_void,
                    nb_elements: u64, dim: u64) -> *mut std::os::raw::c_void {
    let inputs = import_external(raw_inputs, (nb_elements * dim) as usize);
    let solutions = import_external(raw_solutions, nb_elements as usize);

    let weights = safe_rbf_apply(expected_clusters, gamma, solutions, inputs, nb_elements, dim);
    export_external(weights.as_slice())
}

fn mean_point(inputs: &[f64], dim: u64) -> Vec<f64>{
    let mut mean : Vec<f64> = Vec::new();
    let length = inputs.len();
    let nb_elements = length as u64 / dim;
    let mut sums : Vec<f64> = Vec::new();

    for i in 0..dim {
        sums.push(0.);
    }

    for i in 0..nb_elements {
        for j in 0..dim {
            sums[j as usize] += inputs[(i *dim + j) as usize];
        }
    }

    for i in 0..dim {
        mean.push(sums[i as usize] / nb_elements as f64);
    }
    mean
}

fn nearest_point(point: &[f64], solution : f64, point_list: &[f64], dim: u64) -> usize{
    let length = point_list.len();
    let nb_elements = length as u64 / dim;
    let start_index = if solution > 0. { 0 } else { 1 };
    let mut minimum = calculate_square_dist_using_points(point, get_range_from_slice(point_list, start_index as usize, ((start_index + 1) * dim) as usize).as_slice(), dim);
    let mut min_index = 0 as usize;

    for i in 0..(nb_elements / 2) {
        let value = calculate_square_dist_using_points(point,
             get_range_from_slice(point_list, ((start_index + i * 2) * dim) as usize,
             ((start_index + 1 + i * 2) * dim) as usize).as_slice(), dim);
        if value < minimum {
            minimum = value;
            min_index = i as usize;
        }
    }
    min_index
}

fn random_point(dim: u64) -> Vec<f64>{
    let mut point : Vec<f64> = Vec::new();
    for i in 0..dim {
        point.push(rand::thread_rng().gen_range(-1000., 1000.))
    }
    point
}

fn random_point_list(number: u64, dim: u64) -> Vec<f64>{
    let mut points : Vec<f64> = Vec::new();
    for i in 0..number {
        points.extend_from_slice(random_point(dim).as_slice());
    }
    points
}

fn vec_compare(left: &Vec<Vec<f64>>, right: &Vec<Vec<f64>>) -> bool {
    if left.len() != right.len() {
        return false
    }

    for i in 0..left.len() {
        if left[i].len() != right[i].len() {
            return false
        }
        for j in 0..left[i].len() {
            if left[i][j] != right[i][j] {
                return false
            }
        }
    }
    true
}

fn vec_copy<'a>(vector: &'a Vec<Vec<f64>>) -> Vec<Vec<f64>> {

    let mut output : Vec<Vec<f64>> = Vec::new();
    for i in 0..vector.len() {
        output.push(Vec::new());
        for j in 0..vector[i].len() {
            output[i].push(vector[i][j]);
        }
    }
    output
}

fn lloyd_algorithm(inputs: &[f64], solutions: &[f64], expected_clusters: u64, dim: u64, nb_elements: u64) -> Vec<f64> {
    let mut references = random_point_list(expected_clusters, dim);
    let mut previous_cluster : Vec<Vec<f64>> = Vec::new();

    loop {

        let mut cluster : Vec<Vec<f64>> = Vec::new();

        for i in 0..expected_clusters {
            cluster.push(Vec::new());
        }

        for i in 0..nb_elements {
            let current_point = get_range_from_slice(inputs, (i * dim) as usize, ((i + 1) * dim) as usize);
            let current_solution = solutions[i as usize];
            let index = nearest_point(&current_point, current_solution, &references, dim);

            for j in 0..dim {
                cluster[index].push(current_point[j as usize]);
            }
        }

        if vec_compare(&cluster, &previous_cluster) {
            return references
        }
        previous_cluster = vec_copy(&cluster);

        for i in 0..expected_clusters {
            let point = mean_point(&(cluster[i as usize]).as_slice(), dim);

            for j in 0..dim {
                references[(i * dim + j) as usize] = point[j as usize];
            }

        }
    }


}

fn generate_random_solution_matrix(size: u64) -> Vec<f64>{
    let mut solutions : Vec<f64> = Vec::new();

    for i in 0..size {
        if size % 2 == 0 {
            solutions.push(1.);
        }
        else {
            solutions.push(-1.);
        }
    }
    solutions
}

fn safe_rbf_apply(expected_clusters: u64, gamma: f64, solutions: &[f64],
                    inputs : &[f64],
                    nb_elements: u64, dim: u64) -> Vec<f64>{
    let mut matrix : Vec<f64> = Vec::new();
    let references_vec = lloyd_algorithm(inputs, solutions, expected_clusters, dim, nb_elements);
    let references = references_vec.as_slice();

    for i in 0..expected_clusters {
        for j in 0..expected_clusters {
            let square_dist = calculate_square_dist(references, i, j, dim);
            let value = -gamma * square_dist;
            matrix.push(value.exp());
        }
    }
    let output = DMatrix::from_row_slice(expected_clusters as usize, expected_clusters as usize, matrix.as_slice());
    let generated_solutions = generate_random_solution_matrix(expected_clusters);
    let solutions_matrix = DMatrix::from_row_slice(1 as usize, expected_clusters as usize, generated_solutions.as_slice());
    let result = solutions_matrix * output.pseudo_inverse(1e-9);
    let mut weight_vector : Vec<f64> = Vec::new();
    let calculated_weights = result.data.data();

    for i in 0..expected_clusters {
        weight_vector.push(calculated_weights[i as usize]);
    }

    weight_vector
}

fn safe_rbf_apply_full(gamma: f64, solutions: &[f64],
                    inputs : &[f64],
                    nb_elements: u64, dim: u64) -> Vec<f64>{
    let mut matrix : Vec<f64> = Vec::new();

    for i in 0..nb_elements {
        for j in 0..nb_elements {
            let square_dist = calculate_square_dist(inputs, i, j, dim);
            let value = -gamma * square_dist;
            matrix.push(value.exp());
        }
    }
    let output = DMatrix::from_row_slice(nb_elements as usize, nb_elements as usize, matrix.as_slice());
    let solutions_matrix = DMatrix::from_row_slice(1 as usize, nb_elements as usize, solutions);
    let result = solutions_matrix * output.pseudo_inverse(1e-9);
    let mut weight_vector : Vec<f64> = Vec::new();
    let calculated_weights = result.data.data();

    for i in 0..nb_elements {
        weight_vector.push(calculated_weights[i as usize]);
    }

    weight_vector
}

fn calculate_square_dist(inputs : &[f64], x: u64, y: u64, dim: u64) -> f64 {
    let x_index = x * dim;
    let y_index = y * dim;
    let mut sum = 0.;

    for i in 0..dim {
        sum += (inputs[(x_index + i) as usize] - inputs[(y_index + i) as usize]).powi(2);
    }
    sum
}

fn calculate_square_dist_using_points(left : &[f64], right : &[f64], dim: u64) -> f64 {
    let mut sum = 0.;
    for i in 0..dim {
        sum += (left[i as usize] - right[i as usize]).powi(2);
    }
    sum
}

fn get_range_from_slice(slice: &[f64], left: usize, right: usize) -> Vec<f64> {
    let mut range: Vec<f64> = Vec::new();

    for i in left..right {
        range.push(slice[i]);
    }
    range
}

#[test]
fn should_calculate_weights_with_lloyd_algorithm() {
    let solutions = [-1., 1., 1., 1.];
    let inputs = [
                        50., 1., 1.,
                        2., 2., 2.,
                        3., 3., 3.,
                        4., 4., 4.
                    ];

    let w = safe_rbf_apply(2, 0.1, &solutions, &inputs, 4, 3);
    for i in 0..2 {
        println!("W{} = {}", i, w[i]);
    }

}

#[test]
fn should_calculate_weights() {
    let solutions = [-1., 1., 1., 1.];
    let inputs = [
                        50., 1., 1.,
                        2., 2., 2.,
                        3., 3., 3.,
                        4., 4., 4.
                    ];

    let w = safe_rbf_apply_full(0.1, &solutions, &inputs, 4, 3);
    for i in 0..4 {
        println!("W{} = {}", i, w[i]);
    }

}
