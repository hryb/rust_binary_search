fn main() {
    let a = [1, 3, 5, 7, 9];

    let result = binary_search(& a, 9);

    println!("Result is {}", result);
}

fn binary_search(list: &[i32], item: i32) -> usize {
    let mut low = 0;
    let mut high = list.len();
    let mut mid = 0;

    while low < high {
        mid = (low + high) / 2;
        let guess = list[mid];

        if guess == item {
            break;
        }

        if guess > item {
            high = mid - 1;
        } else {
            low = mid + 1;
        }
    }

    return mid;
}
