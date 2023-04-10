use std::{pin::Pin, rc::Rc};

#[derive(Debug)]
pub struct User<'a> {
    pub first_name: &'a str,
    pub last_name: &'a str,
    pub age: u32,
}

impl<'a> User<'a> {
    pub fn new(first_name: &'a str, last_name: &'a str, age: u32) -> Self {
        Self {
            first_name,
            last_name,
            age,
        }
    }

    pub fn info(&self) {
        println!(
            "My name is: {} {}. I am {} years old",
            self.first_name, self.last_name, self.age
        );

        println!("fn: {:p}", &self.first_name);
        println!("ln: {:p}", &self.last_name);
        println!("age: {:p}", &self.age);
    }
}

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

    let hi = String::from("Me");
    let pinned_hi = Pin::new(&hi);
    pinned_hi.say_hi();
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

impl SayHi for String {}

impl<T: Default> MutMeSomehow for Box<T> {
    fn mut_me_somehow(self: Pin<&mut Self>) {
        let pinned_box = self.get_mut().as_mut();
        *pinned_box = T::default();
    }
}

fn mem_replace_without_box() {
    let mut user = User::new("Daniel", "Tro", 21);
    let mut anonim = User::new("", "", 33);

    println!("{:?}", user);
    println!("User pointer {:p}", &user);
    user.info();
    println!("");

    println!("{:?}", anonim);
    println!("Anonim pointer {:p}", &anonim);
    anonim.info();
    println!("");

    println!("-------------------\n");
    std::mem::swap(&mut user, &mut anonim);

    println!("{:?}", user);
    println!("User pointer {:p}", &user);
    user.info();
    println!("");

    println!("{:?}", anonim);
    println!("Anonim pointer {:p}", &anonim);
    anonim.info();
}

fn mem_replace_with_pin() {
    let mut user = User::new("Daniel", "Tro", 21);
    let mut anonim = User::new("", "", 33);

    println!("{:?}", user);
    println!("User pointer {:p}", &user);
    user.info();
    println!("");

    println!("{:?}", anonim);
    println!("Anonim pointer {:p}", &anonim);
    anonim.info();
    println!("");

    let rc = Rc::new(&user);

    let pinned = Pin::new(rc.as_ref());
    println!("-------------------\n");

    println!("{:?}", user);
    println!("User pointer {:p}", &user);
    user.info();
    println!("");

    println!("{:?}", anonim);
    println!("Anonim pointer {:p}", &anonim);
    anonim.info();

    println!("{:?}", pinned)
}

fn main() {
    mut_me_somehow_string();
    mut_me_somehow_pinned_box();
    mem_replace_without_box();
    mem_replace_with_pin();
}
