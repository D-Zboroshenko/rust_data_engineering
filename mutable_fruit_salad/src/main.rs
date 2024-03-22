use std::collections::HashMap;

pub fn remove_from_salad(fruit_salad: &mut Vec<&str>, fruit_to_remove: &str) {
    if let Some(i) = fruit_salad.iter().position(|x| *x == fruit_to_remove) {
        fruit_salad.remove(i);
    }
    else {
        println!("There is no {} in the salad", fruit_to_remove);
    }
}

pub fn count_frequencies(fruit_salad: &Vec<&str>) -> HashMap<String, usize> {
    let mut freqs = HashMap::new();
    for &fruit in fruit_salad.iter() {
        freqs.entry(fruit.to_string()).and_modify(|e| *e += 1).or_insert(1);
    }
    freqs
}

fn main() {
    let mut fruit_salad = vec!["apple", "banana", "cherry", "dates", "elderberries"];
    
    // push figs to salad
    fruit_salad.push("figs");

    // remote the last fruit from the salad
    fruit_salad.pop();
    
    // apply the function for removing the fruit from the salad
    remove_from_salad(&mut fruit_salad, "apple");

    // sort the salad alphabetically
    fruit_salad.sort_unstable();

    println!("Modified fruit salad contains:");
    for fruit in fruit_salad.iter() {
        println!("{}", fruit);
    }

    println!();
    println!("Frequencies of fruits in the salad are:");
    let frequencies = count_frequencies(& fruit_salad);
    for (key, val) in frequencies.iter() {
        println!("{} {}", key, val);
    }
}
