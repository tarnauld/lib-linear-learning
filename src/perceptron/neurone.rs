pub struct Neurone{
    pub value: f64,
    pub potential: f64,
    pub sigmoide: f64,
    pub error : f64,
    pub weights: Vec<f64>
}

impl Neurone{
    pub fn error_calculation(n: *mut Neurone) -> f64{
        unsafe{
            let p = (*n).sigmoide * (1 as f64 - (*n).sigmoide) as f64;
            (*n).error = p * Neurone::error_gap(n);
            return (*n).error;
        }
    }

    pub fn error_gap(n: *mut Neurone) -> f64{
        unsafe{
            (*n).value - (*n).sigmoide
        }
    }

    pub fn new(value: f64) -> Neurone{
        Neurone{
            value: value,
            potential: 0.,
            sigmoide: value,
            error: 0.,
            weights: Vec::new()
        }
    }
}
