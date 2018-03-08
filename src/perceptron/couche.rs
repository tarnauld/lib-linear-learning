use perceptron::neurone::*;

pub struct Couche {
    pub neurones: Vec<Neurone>
}

impl Couche {
    pub fn error_calculation(c: Couche) -> f64{
        let mut err = 0 as f64;

        for (i, val) in c.neurones.iter().enumerate(){
            let p = val.value - val.sigmoide;
            err += p.powf(2 as f64);
        }
        0.5 * err
    }

    pub fn new() -> Couche{
        Couche{
            neurones: Vec::new()
        }
    }
}
