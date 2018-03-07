use linear::classification::*;

#[test]
fn should_generate_random_values(){
    unsafe{
        let w = generate_weight();
        for i in 0..3{
            assert!((*w)[i] >= -1 as f64 && (*w)[i] <= 1 as f64)
        }
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
    let mut data_set : [f64; 9] = [1.0 as f64, 1.0 as f64, 1.0 as f64,
    -1.0 as f64, -1.0 as f64, -1.0 as f64, 1.0 as f64, 1.0 as f64, -1.0 as f64];

    unsafe{
        let w = generate_weight();
        weights_training(w, Box::into_raw(Box::new(data_set)));

        assert!(classify(&mut[1.0 as f64, 1.0 as f64], w) > 0 as f64);
        assert!(classify(&mut[-1.0 as f64, -1.0 as f64], w) < 0 as f64);
        assert!(classify(&mut[1.0 as f64, -1.0 as f64], w) > 0 as f64);

        assert!(classify(&mut[1.0 as f64, 1.001 as f64], w) > 0 as f64);
        assert!(classify(&mut[1.0 as f64, 0.999 as f64], w) > 0 as f64);
        println!("{:?}", classify(&mut[0.0 as f64, -1.0 as f64], w) );
        assert!(classify(&mut[0.0 as f64, -1.0 as f64], w) < 0 as f64);

        assert!(classify(&mut[1.0 as f64, 1.0 as f64], w) > 0 as f64);
        assert!(classify(&mut[-1.0 as f64, -1.0 as f64], w) < 0 as f64);
        assert!(classify(&mut[-1.0 as f64, 1.0 as f64], w) < 0 as f64);
    }
}
