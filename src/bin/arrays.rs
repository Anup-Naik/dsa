// max of an array
fn max_of_array(arr: &[i32]) -> i32 {
    let mut max = i32::MIN;
    for i in 0..arr.len() {
        if arr[i] > max {
            max = arr[i];
        }
    }
    max
}

// Second Largest
fn second_largest(arr: &[i32]) -> i32 {
    let mut largest = i32::MIN + 1;
    let mut sec_largest = i32::MIN;
    for i in 0..arr.len() {
        if arr[i] > largest {
            sec_largest = largest;
            largest = arr[i];
        } else if arr[i] > sec_largest && arr[i] < largest {
            sec_largest = arr[i];
        }
    }
    sec_largest
}

// Second Smallest
fn second_smallest(arr: &[i32]) -> i32 {
    let mut smallest = i32::MAX;
    let mut sec_smallest = i32::MAX - 1;
    for i in 0..arr.len() {
        if arr[i] < smallest {
            sec_smallest = smallest;
            smallest = arr[i];
        } else if arr[i] < sec_smallest && arr[i] > smallest {
            sec_smallest = arr[i];
        }
    }
    sec_smallest
}

// Is array sorted in ascending order
fn is_sorted_asc(arr: &[i32]) -> bool {
    let mut i = 1;
    while i < arr.len() {
        if arr[i - 1] > arr[i] {
            return false;
        }
        i += 1;
    }
    true
}

// Is array sorted in descending order
fn is_sorted_desc(arr: &[i32]) -> bool {
    let mut i = 1;
    while i < arr.len() {
        if arr[i - 1] < arr[i] {
            return false;
        }
        i += 1;
    }
    true
}

// Remove Duplicates In-Place
fn rmv_duplicates(arr: &mut [i32]) -> usize {
    let mut i = 0;
    let mut j = 1;
    while j < arr.len() {
        if arr[j] != arr[i] {
            i += 1;
            arr[i] = arr[j];
        }
        j += 1;
    }
    i + 1
}

// Avg of array values
fn avg_of_array(arr: &[i32]) -> i32 {
    let mut sum = 0;
    let n = arr.len();
    for i in 0..n {
        sum += arr[i];
    }
    sum / (n as i32)
}

// copy to another array
fn array_copy(arr: &[i32]) -> Vec<i32> {
    let mut new_arr = vec![];
    for i in 0..arr.len() {
        new_arr.push(arr[i]);
    }
    new_arr
}

// Reverse an array
fn reverse_array<'a>(arr: &'a mut [i32]) -> &'a [i32] {
    let n = arr.len();
    let mut temp;
    for i in 0..n / 2 {
        temp = arr[i];
        arr[i] = arr[n - 1 - i];
        arr[n - 1 - i] = temp;
    }
    arr
}

// matrix multiplication n x n
fn matrix_multiply(a: &Vec<Vec<i32>>, b: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let n = a.len();
    let mut c: Vec<Vec<i32>> = vec![vec![0; n]; n];
    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                c[i][j] += a[i][k] * b[k][j];
            }
        }
    }
    c
}
fn main() {
    let mut arr = [5, 4, 9, 7, 15, 2, 14];
    let sorted_arr = [-15, -10, 0, 2, 2, 5, 10];
    let mut duplicates = [1, 1, 2, 2, 2, 3, 3, 4, 5, 6, 6];
    println!(
        "Removed Duplicates, Unique Lenght = {} ---> {:?}. ",
        rmv_duplicates(&mut duplicates),
        duplicates
    );
    println!("Array {:?}", arr);
    println!("Another Sorted Array {:?}", sorted_arr);
    println!(
        "Is Sorted in Ascending Order? {}",
        is_sorted_asc(&sorted_arr)
    );
    println!(
        "Is Sorted in Descending Order? {}",
        is_sorted_desc(&sorted_arr)
    );
    println!("Max is {}", max_of_array(&arr));
    println!("Second Largest is {}", second_largest(&arr));
    println!("Second Smallest is {}", second_smallest(&arr));
    println!("Avg is {}", avg_of_array(&arr));
    println!("Copy {:?}", array_copy(&arr));
    println!("Reverse {:?}", reverse_array(&mut arr));
    let a = vec![vec![1, 2], vec![3, 4]];
    let b = vec![vec![1, 2], vec![3, 4]];
    println!("{:?} * {:?} = {:?}", a, b, matrix_multiply(&a, &b));
}
