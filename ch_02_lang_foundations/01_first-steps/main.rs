use std::ops::{Add};

fn main() {
    let a = 10;
    let b: i32 = 20;

    let c = add(a, b);
    println!("a + b = {}", c);

    numbers();

    numeric_literals();

    find_an_int();

    use_match();

    println!("---");
    let res = add_with_lifetimes(&10, &20);
    println!("{}", res);
    println!("-------");

    let (m, n) = (1.2, 3.4);
    let (x, y) = (10, 20);
    let o = generic_func_add(m, n);
    let z = generic_func_add(x, y);
    println!("{} + {} = {}", m, n, o);
    println!("{} + {} = {}", x, y, z);
    println!("-------");

}

fn add(i: i32, j: i32) -> i32 {
    i + j
}

fn numbers() {
    let twenty = 20;
    let twenty_one: i32 = twenty + 1;
    let floats_ok = 21.0;
    let one_million = 1_000_000;

    println!("{}; {}; {}; {}", twenty, twenty_one, floats_ok, one_million);
    println!("---");
}

fn numeric_literals() {
    let three = 0b11;
    let thirty = 0o36;
    let three_hundred = 0x12c;

    // 0b -> binary (base 2)
    // 0o -> octal (base 8)
    // 0x -> hexidecimal (base 16)

    println!("{}, {}, {}", three, thirty, three_hundred);
    println!("{:b}, {:b}, {:b}", three, thirty, three_hundred);
    println!("{:x}, {:x}, {:x}", three, thirty, three_hundred);
    println!("---");
}

fn find_an_int() {
    let needle = 42;
    let haystack = [1, 1, 2, 5, 14, 42, 132, 429, 1430, 4862];

    for reference in haystack.iter() {
        let item = *reference;
        if item == needle {
            println!("{}", item);
        }

        if reference == &needle {
            println!("{}", reference);
        }
    }
    println!("---");
}

fn use_match() {
    let haystack = [1, 1, 2, 5, 14, 42, 132, 429, 1430, 4862];

    for reference in haystack.iter() {
        let item = *reference;

        let result = match item {
            42 | 132 => "hit!", // 42 and 132
            _ => "miss", // _ == wildcard that matches anything
        };

        if result == "hit!" {
            println!("{}: {}", item, result);
        }
    }
    println!("---");
}

fn add_with_lifetimes<'a, 'b>(i: &'a i32, j: &'b i32)  -> i32 {
    // Adds the deereferenced objects together
    // i.e. the referent values held by i and j.
    *i + *j
}

// Generics
// <T: Add<Output = T>> requires that T must implement Add and that
// implementation outputs a value also of type T.
fn generic_func_add<T: Add<Output = T>>(i: T, j: T) -> T {
    i + j
}
