use std::pin::Pin;

trait MutMeSomehow {
    fn mut_me_somehow(self: Pin<&mut Self>);
}

trait SayHi: std::fmt::Debug {
    fn say_hi(self: Pin<&Self>) {
        println!("Hi from {:?}", self)
    }
}

fn mut_me_somehow_string() {
    let mut x = String::from("Hello world!");

    let x_pinned = std::pin::Pin::new(&mut x);
    println!("{:?}", x_pinned);
    x_pinned.mut_me_somehow();
    println!("{:?}", x);
}

fn mut_me_somehow_pinned_box() {
    let mut x = Box::new(String::from("Something which will mutated in the ''"));

    let x_pinned = std::pin::Pin::new(&mut x);
    println!("{:?}", x_pinned);
    x_pinned.mut_me_somehow();
    println!("{:?}", x);
}

impl MutMeSomehow for String {
    fn mut_me_somehow(self: Pin<&mut Self>) {
        let i = self.get_mut();
        i.push_str("I have muted by trait MutMeSomeHow");
    }
}

impl<T: Default> MutMeSomehow for Box<T> {
    fn mut_me_somehow(self: Pin<&mut Self>) {
        let pinned_box = self.get_mut().as_mut();
        *pinned_box = T::default();
    }
}

fn main() {
    mut_me_somehow_string();
    mut_me_somehow_pinned_box();
}
