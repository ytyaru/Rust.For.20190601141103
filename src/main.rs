/*
 * Rustでfor。
 * CreatedAt: 2019-06-01
 */
fn main() {
//    let mut x = 0;
//    for (x = 0; x < 10; x++) {
//        println!("for {}", x);
//    }
    for x in 0..10 {
        println!("for {}", x);
    }
    for x in (0..10).rev() { println!("for {}", x); }

    let array = [1, 3, 5];
    for v in array.iter() { println!("array {}", v); }

    let names = vec!["Bob", "Frank", "Ferris"];
    for name in names.iter() { println!("{}", name); }

//    for x in get_ten() { println!("for {}", x); } // 
}
//fn get_ten() { 0..10 } // error[E0308]: mismatched types    std::ops::Range
