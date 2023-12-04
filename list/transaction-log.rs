use std::cell::RefCell;
use std::rc::Rc;

type SingleLink = Option<Rc<RefCell<Node>>>;

#[derive(Clone)]
struct Node {
    value: String,
    next: SingleLink
}

struct TransactionLog {
    head: SingleLink,
    tail: SingleLink,
    pub length: u64
}

impl Node {
    //creating a new node
    fn new(value: String) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node{
            value, //shorthand for value : value
            next: None, //None variant
        }))
    }
}

impl TransactionLog {
    pub fn new_empty () -> TransactionLog {
        TransactionLog { head: None, tail: None, length: 0 }
    }

    pub fn append(&mut self, value: String) {
        let new = Node::new(value);
        //option.take() takes the value out of option
        //leaving None at it's place 
        match self.tail.take() {
            Some(old) => old.borrow_mut().next = Some(new.clone()),
            None => self.head = Some(new.clone())
        };
        self.length += 1;
        self.tail =  Some(new);
    }

    pub fn pop(&mut self) -> Option<String> {
        self.head.take().map(|head| {
            if let Some(next) = head.borrow_mut().next.take(){
                self.head = Some(next);
            }
            else {
                //makes the tail None as no more nodes left
                self.tail.take();
            }
            self.length -= 1;
            Rc::try_unwrap(head)
            .ok()
            .expect("error")
            .into_inner()//for ref_cell
            .value
        })
    }
}

fn main(){
    println!("Hello World!");
}