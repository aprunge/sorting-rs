use rand::seq::SliceRandom;
use rand::thread_rng;

fn shuffle_array(arr: &mut [i32]) {
    let mut rng = thread_rng();
    arr.shuffle(&mut rng);
}

fn bubble_sort(arr: &mut [i32]) {
    let mut sorted = false;
    while !sorted {
        sorted = true;
        for i in 0..arr.len() - 1 {
            if arr[i] > arr[i + 1] {
                arr.swap(i, i + 1);
                sorted = false;
            }
        }
    }
    println!("Bubble Sort: \n{:?}\n", arr);
}

fn selection_sort(arr: &mut [i32]) {
    for i in 0..arr.len() {
        let mut small = i;
        for j in (i + 1)..arr.len() {
            if arr[j] < arr[small] {
                small = j;
            }
        }
        arr.swap(small, i);
    }
    println!("Selection Sort: \n{:?}\n", arr);
}

fn insertion_sort(arr: &mut [i32]) {
    let n = arr.len();
    for i in 1..n {
        let key = arr[i];
        let mut j = i;

        while j > 0 && arr[j - 1] > key {
            arr[j] = arr[j - 1];
            j -= 1;
        }
        arr[j] = key;
    }
    println!("Insertion Sort: \n{:?}\n", arr);
}

fn merge_sort(arr: &mut [i32], depth: usize) {
    if arr.len() <= 1 {
        return;
    }

    let middle_index = arr.len() / 2;
    let mut merged = vec![0; arr.len()];

    merge_sort(&mut arr[..middle_index], depth + 1);
    merge_sort(&mut arr[middle_index..], depth + 1);

    let (mut left_index, mut right_index, mut merged_index) = (0, middle_index, 0);

    while left_index < middle_index && right_index < arr.len() {
        if arr[left_index] <= arr[right_index] {
            merged[merged_index] = arr[left_index];
            left_index += 1;
        } else {
            merged[merged_index] = arr[right_index];
            right_index += 1;
        }
        merged_index += 1;
    }

    while left_index < middle_index {
        merged[merged_index] = arr[left_index];
        left_index += 1;
        merged_index += 1;
    }

    while right_index < arr.len() {
        merged[merged_index] = arr[right_index];
        right_index += 1;
        merged_index += 1;
    }

    arr.copy_from_slice(&merged);

    if depth == 0 {
        println!("Merge Sort: \n{:?}\n", arr);
    }
}

fn quick_sort(arr: &mut [i32], print_sorted: bool) {
    if arr.len() <= 1 {
        return;
    }

    let pivot_index = arr.len() - 1;
    let pivot = arr[pivot_index];
    let mut i = 0;

    for j in 0..pivot_index {
        if arr[j] <= pivot {
            arr.swap(i, j);
            i += 1;
        }
    }

    arr.swap(i, pivot_index);

    quick_sort(&mut arr[0..i], false);
    quick_sort(&mut arr[i + 1..], false);

    if print_sorted {
        println!("Quick Sort: \n{:?}\n", arr);
    }
}

fn heap_sort(arr: &mut [i32]) {
    fn heapify(arr: &mut [i32], heap_size: usize, root_index: usize) {
        let mut largest = root_index;
        let left = 2 * root_index + 1;
        let right = 2 * root_index + 2;

        if left < heap_size && arr[left] > arr[largest] {
            largest = left;
        }

        if right < heap_size && arr[right] > arr[largest] {
            largest = right;
        }

        if largest != root_index {
            arr.swap(root_index, largest);
            heapify(arr, heap_size, largest);
        }
    }

    let n = arr.len();

    // Build the max heap
    for i in (0..n / 2).rev() {
        heapify(arr, n, i);
    }

    // Extract elements from the heap in descending order
    for i in (1..n).rev() {
        arr.swap(0, i);
        heapify(arr, i, 0);
    }

    println!("Heap Sort: \n{:?}\n", arr);
}

fn counting_sort(arr: &mut [i32]) {
    if arr.is_empty() {
        return;
    }

    let min_value = *arr.iter().min().unwrap();
    let max_value = *arr.iter().max().unwrap();

    let mut counts = vec![0; (max_value - min_value + 1) as usize];

    for &num in arr.iter() {
        counts[(num - min_value) as usize] += 1;
    }

    let mut index = 0;

    for (i, &count) in counts.iter().enumerate() {
        for _ in 0..count {
            arr[index] = i as i32 + min_value;
            index += 1;
        }
    }

    println!("Counting Sort: \n{:?}\n", arr);
}

fn radix_sort(arr: &mut [i32]) {
    if arr.is_empty() {
        return;
    }

    let mut radix = 1;
    let max_value = *arr.iter().max().unwrap();

    while max_value / radix > 0 {
        let mut counts = vec![0; 10];

        for &num in arr.iter() {
            let digit = (num / radix) % 10;
            counts[digit as usize] += 1;
        }

        for i in 1..counts.len() {
            counts[i] += counts[i - 1];
        }

        let mut sorted = vec![0; arr.len()];

        for &num in arr.iter().rev() {
            let digit = (num / radix) % 10;
            let index = counts[digit as usize] - 1;
            sorted[index] = num;
            counts[digit as usize] -= 1;
        }

        arr.copy_from_slice(&sorted);

        radix *= 10;
    }

    println!("Radix Sort: \n{:?}\n", arr);
}

fn main() {
    let mut dataset: [i32; 100] = [
        71, 22, 38, 49, 81, 89, 76, 14, 50, 32, 19, 7, 63, 55, 91, 94, 20, 9, 30, 28,
        1, 3, 11, 17, 64, 86, 74, 4, 27, 65, 36, 24, 99, 47, 13, 67, 82, 45, 23, 79,
        60, 85, 26, 92, 15, 44, 95, 2, 68, 41, 77, 8, 90, 40, 18, 59, 53, 97, 33, 70,
        16, 42, 93, 31, 62, 5, 88, 72, 75, 98, 51, 39, 21, 46, 83, 6, 87, 12, 25, 35,
        56, 66, 43, 80, 96, 58, 48, 54, 61, 69, 37, 29, 84, 57, 10, 78, 34, 73, 52, 100
    ];

    // Run Bubble Sort
    println!("Unsorted: \n{:?}\n", dataset);
    bubble_sort(&mut dataset);
    shuffle_array(&mut dataset);

    // Run Selection Sort
    println!("Unsorted: \n{:?}\n", dataset);
    selection_sort(&mut dataset);
    shuffle_array(&mut dataset);

    // Run Insertion Sort
    println!("Unsorted: \n{:?}\n", dataset);
    insertion_sort(&mut dataset);
    shuffle_array(&mut dataset);

    // Run Merge Sort
    println!("Unsorted: \n{:?}\n", dataset);
    merge_sort(&mut dataset, 0);
    shuffle_array(&mut dataset);

    // Run Quick Sort
    println!("Unsorted: \n{:?}\n", dataset);
    quick_sort(&mut dataset, true);
    shuffle_array(&mut dataset);

    // Run Heap Sort
    println!("Unsorted: \n{:?}\n", dataset);
    heap_sort(&mut dataset);
    shuffle_array(&mut dataset);

    // Run Counting Sort
    println!("Unsorted: \n{:?}\n", dataset);
    counting_sort(&mut dataset);
    shuffle_array(&mut dataset);

    // Run Radix Sort
    println!("Unsorted: \n{:?}\n", dataset);
    radix_sort(&mut dataset);
    shuffle_array(&mut dataset);
}
