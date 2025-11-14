// Select minimum value and insert from beginning
// Time complexity O(n^2) - Best, Avg, Worst
fn selection_sort(mut v: Vec<i32>, n: usize) -> Vec<i32> {
    for i in 0..n - 1 {
        let mut min = i;
        for j in i + 1..n {
            if v[j] < v[min] {
                min = j;
            }
        }
        v.swap(i, min);
    }
    v
}

// larger values bubble up to end
// Time complexity O(n^2) - Avg, Worst | O(n) - Best
fn bubble_sort(mut v: Vec<i32>, n: usize) -> Vec<i32> {
    let mut swapped;
    for i in (0..=n - 1).rev() {
        swapped = false;
        for j in 0..i {
            if v[j] > v[j + 1] {
                v.swap(j, j + 1);
                swapped = true;
            }
        }
        if !swapped {
            break;
        }
    }
    v
}

// insert the value in its correct place
// Time complexity O(n^2) - Avg, Worst | O(n) - Best
fn insertion_sort(mut v: Vec<i32>, n: usize) -> Vec<i32> {
    for i in 1..n {
        let mut j = i;
        while j > 0 && v[j] < v[j - 1] {
            v.swap(j, j - 1);
            j -= 1;
        }
    }
    v
}

use std::thread::{self, JoinHandle};
use std::time::Instant;
fn main() {
    let mut buf = String::new();
    std::io::stdin()
        .read_line(&mut buf)
        .expect("Error Reading n");

    let n: usize = buf.trim().parse().expect("Error Parsing Value");
    let v: Vec<i32> = std::io::stdin()
        .lines()
        .map(|v| v.unwrap().parse().unwrap())
        .take(n)
        .collect();

    println!("Original Array {:#?}", v);
    let sorting_algorithms = [selection_sort, bubble_sort, insertion_sort];
    let mut join_handles: Vec<JoinHandle<_>> = vec![];
    sorting_algorithms
        .iter()
        .enumerate()
        .for_each(|(i, &sort)| {
            let v = v.clone();
            let jh = thread::spawn(move || {
                let start = Instant::now();
                let arr = sort(v, n);
                let tt = start.elapsed();
                println!(
                    "Sorting Algorithm {} => {:#?} .Time Taken {:#?}",
                    i, arr, tt
                );
            });
            join_handles.push(jh);
        });
    for j in join_handles {
        j.join().unwrap();
    }
}
