// generics.rs

//inline 1
// a function
// there's an error here!
fn fill_a_vector<T>(param: &T) -> Vec<T> {
	let mut a_vec: Vec<T> = Vec::new();
	let the_param = param.clone();
	for _ in range(0,5) {
		a_vec.push(the_param); 
	}
	a_vec
}

// a type
type SingleTypeMap<T> = std::collections:HashMap<T, T>;

// a struct
struct Node<T> {
	val: T,
	next: Option<Node<T>>
}

// an enum
enum vec_or_map<T> {
	Vec<T>,
	std::collections::HashMap<T, T>
}
//end 1

// 2nd version
//inline 2
fn fill_a_vector<T: Clone>(param: &T) -> Vec<T> {
	let mut a_vec: Vec<T> = Vec::new();
	let the_param = param.clone();
	for _ in range(0,5) {
		a_vec.push(the_param); 
	}
	a_vec
}
//end 2

// final version
//inline 3
fn fill_a_vector<T: Clone + Copy>(param: &T) -> Vec<T> {
	let mut a_vec: Vec<T> = Vec::new();
	let the_param = param.clone();
	for _ in range(0,5) {
		a_vec.push(the_param); 
	}
	a_vec
}
//end 3

/* errors
//inline 4
generics.rs:7:14: 7:23 error: mismatched types: expected `T` but found `&T` (expected type parameter but found &-ptr)
generics.rs:7 		a_vec.push(the_param);
//end 4

//inline 5
generics.rs:7:14: 7:23 error: use of moved value: `the_param`
generics.rs:7 		a_vec.push(the_param); 
              		           ^~~~~~~~~
generics.rs:7:14: 7:23 note: `the_param` moved here because it has type `T`, which is non-copyable (perhaps you meant to use clone()?)
generics.rs:7 		a_vec.push(the_param); 
//end 5

