pub struct Neurone{
    pub value: f64,
    pub potential: i32,
    pub sigmoide: f64,
    pub error : i64,
    pub weights: Vec<f64>
}

impl Neurone{
    pub fn add_weight(mut n: Neurone, w: f64){
        n.weights.push(w);
    }

    pub fn get_weight(n: Neurone, i: usize) -> f64{
        n.weights[i]
    }

    pub fn set_weight(mut n: Neurone, i: usize, w: f64){
        n.weights[i] = w;
    }

    pub fn error_calculation(n: *mut Neurone) -> i64{
        unsafe{
            let p = (*n).sigmoide * (1 as f64 - (*n).sigmoide) as f64;
            (*n).error = (p * Neurone::error_gap(n)) as i64;
            return (*n).error;
        }
    }

    pub fn error_gap(n: *mut Neurone) -> f64{
        unsafe{
            (*n).value - (*n).sigmoide
        }
    }

    pub fn new(value: f64, sismoide : f64) -> Neurone{
        Neurone{
            value: value,
            potential: 0,
            sigmoide: value,
            error: 0,
            weights: Vec::new()
        }
    }
}
