use colored::Colorize;
use plotters::prelude::*;
use rand::Rng;
use std::collections::HashMap;
use std::i32;
use std::sync::mpsc;
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

// Types
struct Msg(String, Duration);

// Input Helper
fn take_array_input() -> Vec<usize> {
    let mut buf = String::new();
    println!("{}", "Enter the no of inputs".yellow());
    std::io::stdin()
        .read_line(&mut buf)
        .expect("Error Reading n");

    let n: usize = buf.trim().parse().expect("Error Parsing Size");
    println!("{}", "Enter the size of inputs (n)".yellow());
    let v: Vec<usize> = std::io::stdin()
        .lines()
        .map(|v| v.unwrap().parse().expect("Error Parsing Element"))
        .take(n)
        .collect();
    v
}

// Printer
fn printer(input_size: usize, output: &Vec<Msg>) {
    println!();
    println!(
        "For Input Size n = {} the Results are ",
        input_size.to_string().green().bold()
    );
    println!("{:->45}", "");
    println!(
        "|{:<20} | {:<20}|",
        "Sorting Algorithm".green().bold(),
        "Time Taken".green().bold()
    );
    println!("{:->45}", "");
    for r in output {
        println!("|{:<20} | {:<20?}|", r.0, r.1);
    }
    println!("{:->45}", "");
    println!();
}

// Plot Graph
fn plotter(data: Vec<(usize, Vec<Msg>)>) {
    // Data Transformation
    let mut data_mp: HashMap<String, Vec<(usize, f64)>> = HashMap::new();
    for (n, output) in data {
        for msg in output {
            if let Some(v) = data_mp.get_mut(&msg.0) {
                v.push((n, msg.1.as_secs_f64()));
            } else {
                data_mp.insert(msg.0, vec![(n, msg.1.as_secs_f64())]);
            }
        }
    }
    // println!("{:#?}", data_mp);

    // Path to Store Plots
    let img_name = format!(
        "img/chart-{}.png",
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
    );

    // Initialize Plot on BitMap Backend using PNG Format
    let root = BitMapBackend::new(&img_name, (1920, 1080)).into_drawing_area();
    root.fill(&WHITE).unwrap();

    // Create Context
    let mut ctx = ChartBuilder::on(&root)
        .caption("Compare Sorting Algorithms", ("Ariel", 25))
        .margin(20)
        .set_left_and_bottom_label_area_size(60)
        .build_cartesian_2d(0usize..100000usize, 0f64..100f64)
        .unwrap();

    // Draw Mesh
    ctx.configure_mesh()
        .x_desc("Input Size (n)") //axes Descriptons and Styling
        .y_desc("Time Taken in Seconds (s)")
        .axis_desc_style(("Ariel", 25))
        .draw()
        .unwrap();
    for (label, points) in data_mp {
        // Color Map
        let color = match label.as_str() {
            "Merge Sort" => RED,
            "Selection Sort" => MAGENTA,
            "Bubble Sort" => CYAN,
            "Insertion Sort" => BLUE,
            _ => BLACK,
        };
        // Draw Lines
        ctx.draw_series(LineSeries::new(points, &color))
            .unwrap()
            .label(label)
            .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &color));
    }
    // Line Series Config
    ctx.configure_series_labels()
    .label_font(("Ariel",20))
        .border_style(&BLACK)
        .background_style(&WHITE.mix(0.8))
        .draw()
        .unwrap();
}

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

// Merge two sorted arrays
fn merge(left: Vec<i32>, right: Vec<i32>) -> Vec<i32> {
    let mut arr: Vec<i32> = Vec::with_capacity(left.len() + right.len());
    let (mut l, mut r) = (0, 0);
    while l < left.len() && r < right.len() {
        if left[l] < right[r] {
            arr.push(left[l]);
            l += 1
        } else {
            arr.push(right[r]);
            r += 1;
        }
    }
    while l < left.len() {
        arr.push(left[l]);
        l += 1
    }
    while r < right.len() {
        arr.push(right[r]);
        r += 1;
    }
    arr
}

fn merge_sort(v: &[i32]) -> Vec<i32> {
    let n = v.len();
    if n <= 1 {
        return v.to_vec();
    }
    let mid = n / 2;
    // Divide
    let left = merge_sort(&v[0..mid]);
    let right = merge_sort(&v[mid..]);
    // Conquer/Merge
    merge(left, right)
}

// Run Sorting Algorithms
fn run_sorting_algorithms(n: usize) -> (usize, Vec<Msg>) {
    // Creating random input array of size n
    let mut rng = rand::rng();
    let v: Vec<i32> = (0..n).map(|_| rng.random_range(0..i32::MAX)).collect();
    // println!("Original Array: {:?}", v);
    // println!();
    let sorting_algorithms = [
        ("Selection Sort", selection_sort as fn(Vec<i32>) -> Vec<i32>),
        ("Bubble Sort", bubble_sort as fn(Vec<i32>) -> Vec<i32>),
        ("Insertion Sort", insertion_sort as fn(Vec<i32>) -> Vec<i32>),
        ("Merge Sort", |v: Vec<i32>| merge_sort(&v)),
    ];
    let mut join_handles: Vec<JoinHandle<_>> = vec![];
    let (tx, rx) = mpsc::channel::<Msg>();
    for (name, sort) in sorting_algorithms {
        let v = v.clone();
        let transmitter = tx.clone();
        let jh = thread::spawn(move || {
            let start = Instant::now();
            let _arr = sort(v);
            let tt = start.elapsed();
            // println!(
            //     "{} Done. Time Taken: {:?}.\n Sorted Array: {:?}",
            //     name, tt, arr
            // );
            // println!();
            transmitter
                .send(Msg(name.to_owned(), tt))
                .expect("Error Sending Message");
        });
        join_handles.push(jh);
    }
    for j in join_handles {
        j.join().unwrap();
    }
    let outputs: Vec<Msg> = rx.iter().take(4).collect();
    (n, outputs)
}

fn console_app() {
    println!();
    println!(
        "{:-^50}",
        "Compare Sorting Algorithms"
            .bright_magenta()
            .bold()
            .italic()
    );
    println!("{}","The longer the input size the longer it will take.\nSo try to enter input(n) sizes in range 1000 to 10000.".red());
    println!();
    let v = take_array_input();
    let mut data = vec![];
    for i in v {
        let (n, o) = run_sorting_algorithms(i);
        printer(n, &o);
        data.push((n, o));
    }
    println!("{:-^50}", "END".magenta());
    plotter(data);
}

fn main() {
    console_app();
    // plotter();
}
