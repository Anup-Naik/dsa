use std::io;

fn pat1(n: u8) {
    for _i in 0..n {
        for _j in 0..n {
            print!("*");
        }
        println!("");
    }
}

fn pat2(n: u8) {
    for i in 0..n {
        for _j in 0..=i {
            print!("*");
        }
        println!("");
    }
}
fn pat3(n: u8) {
    for i in 0..n {
        for j in 0..=i {
            print!("{}", j + 1);
        }
        println!("");
    }
}
fn pat4(n: u8) {
    for i in 0..n {
        for _j in 0..=i {
            print!("{}", i + 1);
        }
        println!("");
    }
}
fn pat5(n: u8) {
    for i in 0..n {
        for _j in 0..n - i {
            print!("*");
        }
        println!("");
    }
}
fn pat6(n: u8) {
    for i in 0..n {
        for _j in 0..n - i {
            print!(" ");
        }
        for _j in n - i - 1..n + i {
            print!("*");
        }
        for _j in n + i..2 * n - 1 {
            print!(" ");
        }
        println!();
    }
}
fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let n: u8 = input.trim().parse().unwrap();
    pat1(n);
    pat2(n);
    pat3(n);
    pat4(n);
    pat5(n);
    pat6(n);
}
