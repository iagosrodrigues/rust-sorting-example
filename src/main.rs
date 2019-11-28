use std::io::Error;

fn main() -> Result<(), Error> {
    let numbers = vec![5, 8, 1, 2, 7, 3, 6, 9, 4, 10];
    println!("Array: {:?}, size: {}", &numbers, numbers.len());
    println!("CS: {:?}", counting_sort(&mut numbers.clone()));

    let mut numbers = vec![5, 8, 1, 2, 7, 3, 6, 9, 4, 10];
    selection_sort(&mut numbers);
    println!("SS: {:?}", numbers);

    let mut numbers = vec![5, 8, 1, 2, 7, 3, 6, 9, 4, 10];
    bubble_sort(&mut numbers);
    println!("BS: {:?}", numbers);

    let mut numbers = vec![5, 8, 1, 2, 7, 3, 6, 9, 4, 10];
    heap_sort(&mut numbers);
    println!("HS: {:?}", numbers);

    let mut numbers = vec![5, 8, 1, 2, 7, 3, 6, 9, 4, 10];
    quick_sort(&mut numbers);
    println!("QS: {:?}", numbers);

    Ok(())
}

fn selection_sort<T>(list: &mut [T])
where
    T: Ord,
{
    for key in 0..list.len() {
        if let Some((smaller, _)) = list.iter().enumerate().skip(key).min_by_key(|x| x.1) {
            list.swap(key, smaller);
        }
    }
}

fn bubble_sort<T>(list: &mut [T])
where
    T: PartialOrd,
{
    for end in (1..list.len()).rev() {
        for number in 0..end - 1 {
            if list[number] > list[number + 1] {
                list.swap(number, number + 1);
            }
        }
    }
}

fn heap_sort<T>(list: &mut [T])
where
    T: PartialOrd,
{
    for count in (1..list.len()).rev() {
        for root_index in (0..=(count as f32).log2().ceil() as usize).rev() {
            let indice = 2 * root_index;
            if indice < count && list[indice + 1] > list[indice] {
                list.swap(indice, indice + 1);
            }

            if indice + 2 <= count && list[indice + 2] > list[indice] {
                list.swap(indice, indice + 2);
            }
        }
        list.swap(0, count);
    }
}

fn quick_sort_recursive(numbers: &mut [i32], begin: usize, end: usize) {
    let pivot = numbers[(begin + end) / 2];
    let mut pos = (begin, end);

    while pos.0 <= pos.1 {
        while numbers[pos.0] < pivot && pos.0 < end {
            pos.0 += 1;
        }

        while numbers[pos.1] > pivot && pos.1 > begin {
            pos.1 -= 1;
        }

        if pos.0 <= pos.1 {
            numbers.swap(pos.0 as usize, pos.1 as usize);
            pos.0 += 1;
            pos.1 -= 1;
        }
    }

    if pos.1 > begin {
        quick_sort_recursive(numbers, begin, pos.1);
    }

    if pos.0 < end {
        quick_sort_recursive(numbers, pos.0, end);
    }
}

fn quick_sort(list: &mut [i32]) {
    quick_sort_recursive(list, 0, list.len() - 1);
}

fn counting_sort(list: &mut [i32]) -> Vec<i32> {
    let mut output = list.to_owned();
    if let Some(&max_value) = list.iter().max() {
        let mut counting = list
            .iter()
            .fold(&mut vec![0; max_value as usize], |arr, &el| {
                arr[el as usize - 1] += 1;
                arr
            })
            .iter()
            .scan(0, |state, number| {
                *state += *number;
                Some(*state)
            })
            .collect::<Vec<usize>>();

        list.iter().for_each(|&number| {
            output[counting[number as usize - 1] as usize - 1] = number;
            counting[number as usize - 1] -= 1;
        });
    }

    output.to_vec()
}
