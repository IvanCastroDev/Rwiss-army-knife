pub fn calc_fahrenheit_to_celcius(fahrenheit: f32) -> f32 {
    (fahrenheit - 32.0) * 5.0/9.0
}

pub fn calc_celcius_to_fahrenheit(celcius: f32) -> f32 {
    (celcius * 9.0/5.0) + 32.0
}

pub fn calc_fib_sequence(n: f32) -> Vec<u64> {
    let mut sequence: Vec<u64> =  if n > 1.0 {vec![0, 1]} else {vec![0]};

    // first iteration: 
    // sequence[ 2 - 1 = 1] value in index 1 = 1
    // sequence[ 2 - 2 = 0] value in index 0 = 0
    // next value in sequence = 1 + 0 = 1
    // result = [0, 1, 1]
    // second iteration:
    // sequence[ 3 - 1 = 2] value in index 2 = 1
    // sequence[ 3 - 2 = 1] value in index 1 = 1
    // next value in sequence = 1 + 1 = 2
    // result = [0, 1, 1, 2] ...continue until n
    for i in 2..n as usize {
        sequence.push(sequence[i - 1] + sequence[i - 2]);
    }

    sequence
}