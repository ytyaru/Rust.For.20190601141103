/*
 * Rustでfor。
 * CreatedAt: 2019-06-01
 */
use std::ops::Range;

fn main() {
//    let mut x = 0;
//    for (x = 0; x < 10; x++) {
//        println!("for {}", x);
//    }
    for x in 0..10 {
        println!("for {}", x);
    }
    for x in (0..10).rev() { println!("for {}", x); }
//    for x in 9..-1 { println!("for {}", x); }

    let array = [1, 3, 5];
    for v in array.iter() { println!("array {}", v); }

    let names = vec!["Bob", "Frank", "Ferris"];
    for name in names.iter() { println!("{}", name); }

    for x in get_ten() { println!("for {}", x); }
}
//fn get_ten() { 0..10 } // error[E0308]: mismatched types    std::ops::Range
//fn get_ten() -> i32 { 0..10 } // error[E0308]: mismatched types    std::ops::Range
//fn get_ten() -> std::ops::Range { 0..10 } // error[E0107]: wrong number of type arguments: expected 1, found 0
//fn get_ten() -> Range { 0..10 } // error[E0107]: wrong number of type arguments: expected 1, found 0
fn get_ten() -> Range<i32> { 0..10 } 

