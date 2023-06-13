use std::io;

fn quicksort(vec: &Vec<u32>) -> Vec<u32> {
    let len = vec.len();
    if len <= 1 {
        let single_vec: Vec<u32> = vec.clone();
        return single_vec;
    }

    let mut left: Vec<u32> = vec![];
    let mut right: Vec<u32> = vec![];

    let pivot = vec[0];

    for item in &vec[1..] {
        if item < &pivot {
            left.push(*item);
        } else {
            right.push(*item)
        }
    }

    let mut res = quicksort(&left);
    res.extend([pivot]);
    res.extend(quicksort(&right));

    res
}

fn main() {

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Read error");
    let mut input: Vec<u32> = input.split_whitespace().map(|num| num.parse().unwrap()).collect();

    let new_vec = quicksort(&mut input);

    println!("{:?}", new_vec);
}
