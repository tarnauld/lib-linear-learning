use perceptron::couche::*;
use perceptron::neurone::*;
use perceptron::learning::*;
use linear::miscelanous::*;
use rand::{Rng};
use rand;
use rand::distributions::exponential;


fn create_couches(neurone_nb : i32, couche_nb: i32, neurones_output: i32){
    let mut cstart = Couche::new();
    let mut cinter = Couche::new();
    let mut cend = Couche::new();

    for _i in 0..neurone_nb{
        cstart.neurones.push(Neurone::new(0.0));
    }

    for _i in 0..couche_nb{
        cinter.neurones.push(Neurone::new(0.0));
    }

    for _i in 0..neurones_output{
        cend.neurones.push(Neurone::new(0.0));
    }

    let mut learnings : Vec<Learning> = Vec::new();

    for _i in 0..3{
        let mut l = Learning::new();
        learnings.push(l);

        for _i in 0..cstart.neurones.len(){
            l.input.push(0.0); //Replace by learning value
        }

        for _i in 0..cend.neurones.len(){
            l.output.push(0.0); //Replace by learning value
        }
    }

    for _i in 0..cstart.neurones.len(){
        cstart.neurones[_i].weights.push(rand::thread_rng().gen_range(-1., 1.));
    }

    for i in 0..cinter.neurones.len(){
        for _j in 0..cend.neurones.len(){
            cinter.neurones[i].weights.push(rand::thread_rng().gen_range(-1., 1.));
        }
    }

    let mut error = 1.0;
    let mut current = 0;

    loop{
        if error <= 0.095{
            break;
        }

        let mut l = learnings[current];

        for i in 0..cstart.neurones.len(){
            cstart.neurones[i].value = l.input[i];
            cstart.neurones[i].sigmoide = l.input[i];
        }

        for i in 0..cend.neurones.len(){
            cend.neurones[i].value = l.output[i];
        }

        for i in 0..cinter.neurones.len(){
            let mut pot = 0.0;

            for j in 0..cstart.neurones.len(){
                pot += cstart.neurones[j].sigmoide * cstart.neurones[j].weights[i];
            }
            cinter.neurones[i].potential = pot;
            cinter.neurones[i].sigmoide = 1./(1. + (-1. * pot).exp());
        }

        for i in 0..cend.neurones.len(){
            let mut pot = 0.0;

            for j in 0..cinter.neurones.len(){
                pot += cinter.neurones[j].sigmoide * cinter.neurones[j].weights[i];
            }

            cend.neurones[i].potential = pot;
            cend.neurones[i].sigmoide = 1./(1. + (-1. * pot).exp());
        }

        l.error = Couche::error_calculation(cend);

        for i in 0..cend.neurones.len(){
            Neurone::error_calculation(Box::into_raw(Box::new(cend.neurones[i])));
        }
    }
}
