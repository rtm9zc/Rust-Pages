fn main() {
//inline 1
    let mut vec = ~[0, 1, 2];
    vec.push(3); // Appends to end: [0, 1, 2, 3]

    vec.insert(2, 10); 
    // Inserts 10 at position 2: [0, 1, 10, 2, 3]

    let last = vec.pop(); 
    // Returns last element, removing it from the vector: [0, 1, 10, 2]

    let element1 = vec.remove(1); 
    // Returns and removes element at specified index: [0, 10, 2]

    // An iterator for going through all elements in order: 
    for &x in vec.iter() { // note the use of & for borrowing
        println!("{}", x);
    }
    
    // len() returns the number of elements in the vector
    for i in range(0, vec.len()) { 
        println!("{}", vec[i]);
    }
//end 1
}
