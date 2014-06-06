//inline 7
use std::fmt::Show; 
use std::hash::Hash;
use std::collections::hashmap::HashMap;

fn main() {
	
	// some strings with some words
	let mut strings: Vec<String> = vec!("these are a bunch of words".to_string(), 
		"those are a bunch of words too".to_string(), 
		"lots of words".to_string(),
		"there certainly are a lot of words floating around here".to_string(),
		"never before have I seen so many words just sitting about".to_string(),
		"with not a thing to do".to_string());
//inline 6	
	// function for map
	fn create_pairs(s: &String) -> Vec<(String, int)> {
		let mut retvals: Vec<(String,int)> = vec!(); 
		for word in s.as_slice().split(' ') {
			retvals.push((word.to_string(), 1));
		}
		retvals
	}
	
	// function for reduce
	fn reduce_pairs(key: String, vals: Vec<int>) -> Vec<(String, int)> {
		let mut total: int = 0;
		for val in vals.iter() {
			total += *val;
		}
		vec!((key, total))
	}
//end 6	
	// let's do it
	strings.mapreduce::<String,int>(create_pairs, reduce_pairs);	
}
//inline 2
trait MapReduce {
	fn mapreduce<K: Clone + Show + Hash + Equiv<K> + Eq + Send, V: Clone + Show + Send>(&mut self, fn(&String) -> Vec<(K, V)>, 
		fn(K, Vec<V>) -> Vec<(K, V)>);
}

impl MapReduce for Vec<String> {
	fn mapreduce<K: Clone + Show + Hash + Equiv<K> + Eq + Send, V: Clone + Show + Send>(&mut self, mapf: fn(&String) -> Vec<(K, V)>, 
		redf: fn(K, Vec<V>) -> Vec<(K, V)>) {
		
		let (sender, receiver): (Sender<Vec<(K, V)>>, Receiver<Vec<(K, V)>>) = channel();
		let mut tasks: int = 0;
//inline 3
		// map 
		for item in self.iter() {
			tasks += 1;			
			let item_owned = item.clone();
			let sender_child = sender.clone();
			spawn(proc() {			
				sender_child.send(mapf(&item_owned));
			});
		}
//end 2
//inline 4
		// intermediate 
		let mut kv_map: HashMap<K, Vec<V>> = HashMap::new();
		for _ in range(0, tasks) {
			let ivals: Vec<(K, V)> = receiver.recv();
			for pair in ivals.iter() {
				let mut key: K;
				let mut val: V;
				match pair.clone() {
					(a, b) => {
						key = a.clone();
						val = b.clone();
					}
				}
				
				if kv_map.contains_key_equiv(&key) {
					kv_map.get_mut(&key).push(val);
				}
				else {
					kv_map.find_or_insert(key, vec!(val));
				}
			}		
		}
//end 3		
		// reduce
		tasks = 0;
		for key in kv_map.keys() {
			tasks += 1;
			let vals = kv_map.get(key).clone();
			let key_owned = key.clone();
			let sender_child = sender.clone();
			spawn(proc() {
				sender_child.send(redf(key_owned, vals));
			});		
		}
		
		// print final values
		for _ in range(0, tasks) {
			let rvals: Vec<(K, V)> = receiver.recv();
			println!("{}", rvals);
		}		
	}
}	
//end 4
//end 7

//EVERYTHING BELOW THIS LINE ARE TEMPLATES USED FOR TUTORIAL CODE EXAMPLES

/*
//inline 1
trait MapReduce {
        fn mapreduce<K, V>(&mut self, fn(&String) -> Vec<(K,V)>, fn(K, Vec<V>) => Vec<(K,V)>);
}
//end 1

//inline 5
fn main() {
	
	// some strings with some words
	let mut strings: Vec<String> = vec!("these are a bunch of words".to_string(), 
		"those are a bunch of words too".to_string(), 
		"lots of words".to_string(),
		"there certainly are a lot of words floating around here".to_string(),
		"never before have I seen so many words just sitting about".to_string(),
		"with not a thing to do".to_string());
	
	// function for map
	fn create_pairs(s: &String) -> Vec<(String, int)> {
		//code
	}
	
	// function for reduce
	fn reduce_pairs(key: String, vals: Vec<int>) -> Vec<(String, int)> {
		//code
	}
	
	// let's do it
	strings.mapreduce::<String,int>(create_pairs, reduce_pairs);	
}
//end 5
*/
