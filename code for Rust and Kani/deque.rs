struct Deque<T> {
    data: Vec<T>,
}

impl<T> Deque<T> {
    fn new() -> Self {
        Deque { data: Vec::new() }
    }

    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    fn push_front(&mut self, item: T) {
        self.data.insert(0, item);
    }

    fn push_back(&mut self, item: T) {
        self.data.push(item);
    }

    fn pop_front(&mut self) -> Option<T> {
        self.data.pop()
    }

    fn pop_back(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            Some(self.data.remove(0))
        }
    }

    fn front(&self) -> Option<&T> {
        self.data.first()
    }

    fn back(&self) -> Option<&T> {
        self.data.last()
    }
}
#[cfg(kani)]
    #[kani::proof]
fn main() {
    let mut deque: Deque<i32> = Deque::new();

    deque.push_front(1);
    deque.push_back(2);
    deque.push_front(3);

    println!("Front: {:?}", deque.front());
    println!("Back: {:?}", deque.back());

    while let Some(item) = deque.pop_back() {
        println!("Popped: {}", item);
    }
} 