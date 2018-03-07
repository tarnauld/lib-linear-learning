pub struct Learning {
    input: Vec<f64>,
    output: Vec<f64>,
    error: f64
}

impl Learning {
    pub fn add_input(mut l: Learning, input: f64){
        l.input.push(input);
    }

    pub fn add_output(mut l: Learning, output: f64){
        l.output.push(output);
    }

    pub fn get_input(l: Learning, i: usize) -> f64{
        l.input[i]
    }

    pub fn get_output(l: Learning, i: usize) -> f64{
        l.output[i]
    }

    pub fn set_error(mut l: Learning, value: f64){
        l.error = value;
    }

    pub fn get_error(l: Learning) -> f64{
        l.error
    }

    pub fn new() -> Learning{
        Learning{
            input: Vec::new(),
            output: Vec::new(),
            error: 2 as f64
        }
    }
}
