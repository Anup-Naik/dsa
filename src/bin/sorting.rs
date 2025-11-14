
fn swap(v: &mut Vec<i32>, i: usize, j: usize) {
    let temp = v[i];
    v[i] = v[j];
    v[j] = temp;
}

fn selection_sort(v: &mut Vec<i32>, n: usize) {
    for i in 0..n {
        let mut min = i;
        for j in i..n {
            if v[j] < v[min] {
                min = j;
            }
        }
        swap(v, i, min);
    }
}
fn main() {
    let mut buf = String::new();
    std::io::stdin()
        .read_line(&mut buf)
        .expect("Error Reading n");
    let n: usize = buf.trim().parse().expect("Error Parsing Value");
    let mut v: Vec<i32>;
    v = std::io::stdin()
        .lines()
        .map(|v| v.unwrap().parse().unwrap())
        .take(n)
        .collect();
    println!("Original Array {:#?}", v);
    selection_sort(&mut v, n);
    println!("Sorted Array {:#?}", v);
}
