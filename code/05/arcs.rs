extern crate sync;
use sync::Arc;

fn main() {
//inline 1
    let nums = [1,78,3,5,-2,5,7,-11];

    let numArc = Arc::new(nums);

    for i in range(0, nums.len()) {
        let taskArc = numArc.clone();
        spawn(proc() {
            let taskNums = *taskArc;
            println!("{:d}", taskNums[i]);
        });
    }
//end 1
}
