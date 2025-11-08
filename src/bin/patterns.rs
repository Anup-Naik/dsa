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
        for j in 0..n - i {
            print!("{}", j + 1);
        }
        println!("");
    }
}
fn pat7(n: u8) {
    for i in 0..n {
        for _j in 0..n - i - 1 {
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

fn pat8(n: u8) {
    for i in 0..n {
        for _j in 0..i {
            print!(" ");
        }
        for _j in i..2 * n - i - 1 {
            print!("*");
        }
        for _j in n - i..2 * n - 1 {
            print!(" ");
        }
        println!();
    }
}

fn pat9(n: u8) {
    pat7(n);
    pat8(n);
}

fn pat10(n: u8) {
    let mut stars: u8;
    for i in 0..2 * n - 1 {
        stars = i + 1;
        if i >= n {
            stars = 2 * n - i - 1;
        }
        for _j in 0..stars {
            print!("*");
        }
        println!();
    }
}
fn pat11(n: u8) {
    for i in 0..n {
        for j in 0..=i {
            if i % 2 == 0 {
                if j % 2 == 0 {
                    print!("1");
                } else {
                    print!("0");
                }
            } else {
                if j % 2 == 0 {
                    print!("0");
                } else {
                    print!("1");
                }
            }
        }
        println!();
    }
}
fn pat12(n: u8) {
    for i in 0..n {
        for j in 0..i + 1 {
            print!("{}", j + 1);
        }
        for _j in i + 1..2 * n - 1 - i {
            print!(" ");
        }
        for j in 2 * n - 1 - i..2 * n {
            print!("{}", 2 * n - j);
        }
        println!();
    }
}
fn pat13(n: u8) {
    let mut count = 1;
    for i in 0..n {
        for _j in 0..=i {
            print!("{} ", count);
            count += 1;
        }
        println!();
    }
}
fn pat14(n: u8) {
    for i in 0..n {
        for j in 'A'..=char::from('A' as u8 + i) {
            print!("{}", j);
        }
        println!();
    }
}
fn pat15(n: u8) {
    for i in (0..n).rev() {
        for j in 'A'..=char::from('A' as u8 + i) {
            print!("{}", j);
        }
        println!();
    }
}
fn pat16(n: u8) {
    for i in 0..n {
        for _j in 0..=i {
            print!("{}", char::from(65 + i));
        }
        println!();
    }
}
fn pat17(n: u8) {
    for i in 0..n {
        for _j in 0..n - i {
            print!(" ");
        }
        for j in 0..i + 1 {
            print!("{}", char::from(65 + j));
        }
        for j in (0..i).rev() {
            print!("{}", char::from(65 + j));
        }
        for _j in 0..n - i {
            print!(" ");
        }
        println!();
    }
}
fn pat18(n: u8) {
    for i in 0..n {
        for j in char::from(65 + n - 1 - i)..=char::from(65 + n - 1) {
            print!("{}", j);
        }
        println!();
    }
}
fn pat19(n: u8) {
    for i in 0..n {
        for _j in 0..n - i {
            print!("*");
        }
        for _j in 0..2 * i {
            print!(" ");
        }
        for _j in 0..n - i {
            print!("*");
        }
        println!();
    }

    for i in 0..n {
        for _j in 0..=i {
            print!("*");
        }
        for _j in 0..2 * n - 2 * (i + 1) {
            print!(" ");
        }
        for _j in 0..=i {
            print!("*");
        }
        println!();
    }
}
fn pat20(n: u8) {
    for i in 0..n {
        for _j in 0..=i {
            print!("*");
        }
        for _j in 0..2 * n - 2 * (i + 1) {
            print!(" ");
        }
        for _j in 0..=i {
            print!("*");
        }
        println!();
    }
    for i in 1..n {
        for _j in 0..n - i {
            print!("*");
        }
        for _j in 0..2 * i {
            print!(" ");
        }
        for _j in 0..n - i {
            print!("*");
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
    pat7(n);
    pat8(n);
    pat9(n);
    pat10(n);
    pat11(n);
    pat12(n);
    pat13(n);
    pat14(n);
    pat15(n);
    pat16(n);
    pat17(n);
    pat18(n);
    pat19(n);
    pat20(n);
}
