use std::collections::{btree_map::Keys, HashMap};

fn main() {
    let array: [i32; 32] = [
        1, 2, 3, 4, 5, 6, 8, 9, -2, -2, 8, 7, 9, 4, 6, 7, 8, 3, 1, 1, -5, 6, -5, -8, -8, 7, 44, -45, -44, -45, 44,
        23,
    ];
    
    // Boş bir HashMap oluştur
    let mut my_map: HashMap<i32, usize> = HashMap::new();

    // Boş bir vektör oluştur
    let mut unic_array: Vec<i32> = Vec::new();
    
    for &num in &array {
        // HashMap'te bu numara var mı?
        let counter = my_map.entry(num).or_insert(0);
        // Eğer varsa, sayacı artır.
        *counter += 1;

        
    }

    for &key in my_map.keys(){
        unic_array.push(key);
    }

    println!("Sonuç: {:?}", my_map);
    println!("unic array: {:?}", unic_array);
    
}
