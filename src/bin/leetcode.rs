// 1018. Binary Prefix Divisible By 5
fn prefixes_div_by5(nums: Vec<i32>) -> Vec<bool> {
    let mut ans = Vec::with_capacity(nums.len());
    let mut remainder = 0;
    for num in nums {
        remainder *= 2;
        remainder += num;
        remainder %= 5;
        ans.push(remainder == 0);
    }
    ans
}

// 1796. Second Largest Digit in a String
fn second_largest_digit(s: String) -> i32 {
    let nums: Vec<i32> = s
        .chars()
        .filter(|c| c.is_digit(10))
        .map(|c| c.to_digit(10).unwrap() as i32)
        .collect();
    let mut largest = -1;
    let mut sec_largest = -1;
    for i in 0..nums.len() {
        if nums[i] > largest {
            sec_largest = largest;
            largest = nums[i];
        } else if nums[i] > sec_largest && nums[i] < largest {
            sec_largest = nums[i];
        }
    }
    sec_largest
}

fn main() {
    println!(
        "1018. Binary Prefix Divisible By 5 {:?}",
        prefixes_div_by5(vec![0, 1, 0, 1, 1, 0, 1])
    );
    println!(
        "1796. Second Largest Digit in a String. {} --> {}",
        "anup7598",
        second_largest_digit("anup7598".to_owned())
    );
}
