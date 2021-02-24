use std::time::SystemTime;

fn print_list(list: &[u8]) {
    let last_element = &list[list.len() - 1];
    for el in list.iter() {
        let comma = if el != last_element { true } else { false };
        let str_elem = if comma {
            format!("{},", el)
        } else {
            format!("{}", el)
        };
        print!("{}", str_elem);
    }
    println!();
}

fn quicksort(list: &mut [u8], left: usize, right: usize) {
    let pivot = list[(left + right) / 2];

    let mut i = left;
    let mut j = right;
    let mut aux: u8;
    while i < j {
        while list[i] < pivot {
            i += 1;
        }

        while list[j] > pivot {
            j -= 1;
        }

        if i <= j {
            aux = list[i];
            list[i] = list[j];
            list[j] = aux;
            i += 1;
            j -= 1;
        }
    }

    if left < j {
        quicksort(list, left, j);
    }
    if i < right {
        quicksort(list, i, right);
    }
}

fn main() {
    let mut array = [1, 12, 5, 26, 7, 14, 3, 7];
    let last = array.len() - 1;
    let start = SystemTime::now();
    println!("Unsorted:");
    print_list(&array);
    quicksort(&mut array, 0, last);
    println!("Sorted:");
    print_list(&array);
    let end = SystemTime::now();
    println!(
        "Time in secs that took to sort numbers: {}",
        end.duration_since(start).unwrap().as_nanos()
    );
}
