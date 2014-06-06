use std::comm::channel;

//inline 1
struct Node {
    val: int,
    tail: Option<Box<Node>>
}
//end 1

type List = Option<Box<Node>>;

fn test_list(n: int, x: int) -> List {
    if n == 0 {
        None
    } else {
        Some(box Node{val: x, tail: test_list(n - 1, x + 1)} )
    }
}
//inline 3
trait Map {
    fn mapr(&self, fn(int) -> int) -> List;
}
//end 3
impl Map for List {
    fn mapr(&self, f: fn(int) -> int) -> List {
         match(*self) {
            None => None,
            Some(ref current) => { 
                let (port, chan) = channel();
                let currentval = current.val;
                spawn(proc() {
                    let result = f(currentval);
                    port.send(result);
                });
                let newtail = current.tail.mapr(f);
                let newval = chan.recv();
                Some(box Node{ val: newval, tail: newtail }) },
        } 
    } 
}

fn main()
{
 
}

fn plustwo(n: int) -> int {
    n + 2
}
fn timestwo(n:int) -> int {
    n*2
}

fn nodes() {
//inline 2
    let node1 = Node {val:10,  tail: None};
    let mut node2 = Node {val: 10, tail: None};
    node2.val = 15; 
//end 2
}
