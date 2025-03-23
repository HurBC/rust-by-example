enum List {
    Node(u32, Box<List>),
    Tail
}

impl List {
    fn new() -> List {
        List::Tail
    }

    fn prepend(self, item: u32) -> List {
        List::Node(item, Box::new(self))
    }

    fn len(&self) -> u32 {
        match *self {
            List::Node(_, ref tail, ) => 1 + tail.len(),
            List::Tail => 0,
        }
    }

    fn stringify(&self) -> String {
        match *self {
            List::Node(head, ref tail) => {
                format!("{}, {}", head, tail.stringify())
            },
            List::Tail => {
                format!("Tail")
            },
        }
    }
}

pub(super) fn test_case() {
    let mut list = List::new();

    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    println!("--linked list has length: {}", list.len());
    println!("--linked list has elements: {}", list.stringify());
}
