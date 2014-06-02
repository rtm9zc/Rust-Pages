//inline 1
struct Node {
    val: int,
    tail: Option<~Node>
}
//end 1

type List = Option<~Node>;

fn test_list(n: int, x: int) -> List {
    if n == 0 {
        None
    } else {
        Some(~Node{val: x, tail: test_list(n - 1, x + 1)})
    }
}
//inline 3
trait Map {
    fn mapr(&self, extern fn(int) -> int) -> List;
}
//end 3
impl Map for List {
    fn mapr(&self, f: fn(int) -> int) -> List {
         match(*self) {
            None => None,
            Some(ref current) => { 
                let (port, chan) : (Port<int>, Chan<int>) = Chan::new();
                let currentval = current.val;
                do spawn {
                    let result = f(currentval);
                    chan.send(result);
                }
                let newtail = current.tail.mapr(f);
                let newval = port.recv();
                Some(~Node{ val: newval, tail: newtail }) },
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
