// Select minimum value and insert from beginning
// Time complexity O(n^2) - Best, Avg, Worst
fn selection_sort(mut v: Vec<i32>) -> Vec<i32> {
    let n = v.len();
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
fn bubble_sort(mut v: Vec<i32>) -> Vec<i32> {
    let n = v.len();
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
fn insertion_sort(mut v: Vec<i32>) -> Vec<i32> {
    let n = v.len();
    for i in 1..n {
        let mut j = i;
        while j > 0 && v[j] < v[j - 1] {
            v.swap(j, j - 1);
            j -= 1;
        }
    }
    v
}

// Divide array in half until only one element is left and start merging
// Time Complexity O(nlogn) - Best, Avg, Worst
fn merge(left: Vec<i32>, right: Vec<i32>) -> Vec<i32> {
    let mut arr: Vec<i32> = vec![];
    let (mut l, mut r) = (0, 0);
    while l < left.len() || r < right.len() {
        if l == left.len() {
            arr.push(right[r]);
            r += 1;
        } else if r == right.len() {
            arr.push(left[l]);
            l += 1
        } else if left[l] < right[r] {
            arr.push(left[l]);
            l += 1
        } else {
            arr.push(right[r]);
            r += 1;
        }
    }
    arr
}

fn merge_sort(v: &[i32]) -> Vec<i32> {
    let n = v.len();
    if n <= 1 {
        return v.to_vec();
    }
    let mid = n / 2;
    let left = merge_sort(&v[0..mid]);
    let right = merge_sort(&v[mid..]);

    merge(left, right)
}

use std::thread::{self, JoinHandle};
use std::time::Instant;
fn main() {
    let mut buf = String::new();
    println!("Enter the size of input");
    std::io::stdin()
        .read_line(&mut buf)
        .expect("Error Reading n");

    let n: usize = buf.trim().parse().expect("Error Parsing Size");
  println!("Enter the Elements");
    let v: Vec<i32> = std::io::stdin()
        .lines()
        .map(|v| v.unwrap().parse().expect("Error Parsing Element"))
        .take(n)
        .collect();

    println!("Original Array: {:?}", v);
    let sorting_algorithms = [
        ("Selection Sort", selection_sort as fn(Vec<i32>) -> Vec<i32>),
        ("Bubble Sort", bubble_sort as fn(Vec<i32>) -> Vec<i32>),
        ("Insertion Sort", insertion_sort as fn(Vec<i32>) -> Vec<i32>),
        ("Merge Sort", |v: Vec<i32>| merge_sort(&v)),
    ];
    let mut join_handles: Vec<JoinHandle<_>> = vec![];
    for (name, sort) in sorting_algorithms {
        let v = v.clone();
        let jh = thread::spawn(move || {
            let start = Instant::now();
            let arr = sort(v);
            let tt = start.elapsed();
            println!(
                "{} Done. Time Taken: {:?}. Sorted Array: {:?}",
                name, tt, arr
            );
        });
        join_handles.push(jh);
    }
    for j in join_handles {
        j.join().unwrap();
    }
}
