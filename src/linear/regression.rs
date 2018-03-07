use nalgebra::core::dimension::Dynamic as Dynamic;
use nalgebra::core::MatrixMN as MatrixMN;

#[no_mangle]
pub extern fn linear_regression(step: f64, p : &[f64], dim: u64, nb: u64) -> bool{

    let m = Dynamic::new(nb as usize);
    let n = Dynamic::new(dim as usize);
    let matrix : MatrixMN<f64, Dynamic, Dynamic> = MatrixMN::from_row_slice_generic(m, n, p);
    let inversedMatrix = matrix.pseudo_inverse(1e-9);

    return true;
}
