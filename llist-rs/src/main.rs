pub mod list;

#[derive(Debug)]
struct Noisy(u8);

impl Drop for Noisy {
    fn drop(&mut self) {
        println!("[Noisy::drop] {}", self.0)
    }
}

fn main() {
    let mut list = list::LinkedList::from([Noisy(1)]);

    for i in 0..19 {
        list.append(Noisy(i))
    }
 
    println!("{:?}", list)
}
