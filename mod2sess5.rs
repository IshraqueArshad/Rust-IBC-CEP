use std::collections::HashMap;
use std::cmp::Ord;

// Define the SortByKey trait with a method for sorting elements by key
trait SortByKey<K, V> {
    fn sort_by_key(&self) -> Vec<(&K, &V)>;
}

// Implement the SortByKey trait for the HashMap struct
impl<K: Ord, V> SortByKey<K, V> for HashMap<K, V> {
    fn sort_by_key(&self) -> Vec<(&K, &V)> {
        let mut sorted_pairs: Vec<(&K, &V)> = self.iter().collect();
        sorted_pairs.sort_by(|a, b| a.0.cmp(b.0)); // Sort by key in ascending order
        sorted_pairs
    }
}

fn main() {
    // Create a new HashMap and add key-value pairs
    let mut my_map: HashMap<i32, &str> = HashMap::new();
    my_map.insert(5, "five");
    my_map.insert(2, "two");
    my_map.insert(8, "eight");
    my_map.insert(1, "one");

    println!("Original map:");
    for (key, value) in &my_map {
        println!("{}: {}", key, value);
    }

    // Sort the elements in the map by key and print the sorted map
    let sorted_pairs = my_map.sort_by_key();
    println!("Sorted map:");
    for (key, value) in sorted_pairs {
        println!("{}: {}", key, value);
    }
}
