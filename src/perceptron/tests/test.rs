use perceptron::perceptron;

#[test]
fn should_learn_correctly(){
    perceptron::perceptron(4, 2, 1);
}
