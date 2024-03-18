/*
This example code counts the frequency of each number in the vector.
 */
use std::collections::HashMap;
use std::hash::Hash;
use cli_utils::read_stdin;
    

fn logic<T: Hash + Eq>(numbers: Vec<T>) -> Vec<(T, u32)> {
    let mut frequencies = HashMap::new();

    for num in numbers {
        let frequency = frequencies.entry(num).or_insert(0);
        *frequency += 1;
    }

    let mut result = Vec::new();

    for (num, frequency) in frequencies {
        result.push((num, frequency));
    }

    result.sort_by(|a, b|a.1.cmp(&b.1));

    result
}

fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 3];
    let result = logic(numbers);
    //print the results in a human readable format that explains what the result is.
    println!(
        "The frequency of each number in the vector is: {:?}",
        result
    );

    let words = vec!["some".to_string(), "words".to_string(), "are".to_string(), "just".to_string(), "words".to_string()];
    let result = logic(words);
    //print the results in a human readable format that explains what the result is.
    println!(
        "The frequency of each word in the vector is: {:?}",
        result
    );

    println!("Please insert the words");
    let input = read_stdin();
    let words = input.split(" ").map(|s| s.to_string()).collect::<Vec<String>>();
    let result = logic(words);
    println!(
        "The frequency of each word in the vector is: {:?}",
        result
    );


}
