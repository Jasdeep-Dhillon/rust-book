// Importing hashmap (Not included by default like strings and vectors)
use std::collections::HashMap;

fn main() {
    let mut hashmap: HashMap<String, u32> = HashMap::new();
    hashmap.insert(String::from("Key"), 10);
    hashmap.insert(String::from("Key2"), 20);

    // Creating hashmap from data using tuple array
    let _: HashMap<String, u32> =
        HashMap::from([("Key1".to_string(), 20), ("Key2".to_string(), 10)]);

    // Accessing values in Hashmap
    
    // Using concise control flow to unwrap option values 
    if let Some(value) = hashmap.get("Key") {
        println!("Key has value of {value}");
    };
    if let Some(value) = hashmap.get("Key1") {
        println!("Key1 has value of {value}");
    };
    
    // Using unwrap_or to get default value
    let value = hashmap.get("Key1").copied().unwrap_or(u32::MAX);
    println!("Key1 has value of {value}");
    
    // Iterating on keys
    // Keys are in random order
    for (key, value) in &hashmap {
        println!("{key}: {value}");
    }
    
    let key = String::from("Key3");
    let value = 12;
    // Key ownership transferred to hashmap 
    // Value is copied instead
    hashmap.insert(key, value);
    println!("{hashmap:?}");
    
    // Replacing value of key by using insert
    hashmap.insert(String::from("Key3"), 24);
    println!("{hashmap:?}");
    
    // Getting value if exists, incrementing the value by 5 if exists
    let value3 = hashmap.entry(String::from("Key3")).or_insert(50);
    *value3 += 5;
    println!("Key3:{value3}");
    // Inserting value if doesn't exist
    hashmap.entry(String::from("Key4")).or_insert(50);
    println!("{hashmap:?}");
    
    // Hashmap by default uses SipHash
    // Can change the hashing function by using with_hasher function 
    // Use different hashmap optimized for the usecase
    // HashMap::with_hasher()
}
