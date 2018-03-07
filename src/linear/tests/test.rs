use linear::classification::*;

#[test]
fn should_generate_random_values(){
    unsafe{
        let w = generate_weight();
    }
}

#[test]
fn should_classify(){
    unsafe{
            assert!(classify(&mut[1.0 as f64, 1.0 as f64], &mut[1.0 as f64, 1.0 as f64, 1.0 as f64]) == 1 as f64);
            assert!(classify(&mut[-1.0 as f64, -1.0 as f64], &mut[1.0 as f64, 1.0 as f64, 1.0 as f64]) == -1 as f64);
            assert!(classify(&mut[1.0 as f64, -1.0 as f64], &mut[1.0 as f64, 1.0 as f64, 1.0 as f64]) == 1 as f64);

            assert!(classify(&mut[1.0 as f64, 1.001 as f64], &mut[0.0 as f64, -1.0 as f64, 1.0 as f64]) == 1 as f64);
            assert!(classify(&mut[1.0 as f64, 0.999 as f64], &mut[0.0 as f64, -1.0 as f64, 1.0 as f64]) == -1 as f64);
            assert!(classify(&mut[0.0 as f64, -1.0 as f64], &mut[0.0 as f64, -1.0 as f64, 1.0 as f64]) == -1 as f64);

            assert!(classify(&mut[1.0 as f64, 1.0 as f64], &mut[-0.5 as f64, 1.0 as f64, 0.0 as f64]) == 1 as f64);
            assert!(classify(&mut[-1.0 as f64, -1.0 as f64], &mut[-0.5 as f64, 1.0 as f64, 0.0 as f64]) == -1 as f64);
            assert!(classify(&mut[-1.0 as f64, 1.0 as f64], &mut[-0.5 as f64, 1.0 as f64, 0.0 as f64]) == -1 as f64);
    }
}

#[test]
fn should_train_nodes(){
    unsafe{
        let w = generate_weight();
        // let data_set = [[]];
        //weights_training(w, )
    }
}
