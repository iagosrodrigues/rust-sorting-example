use std::io::Error;

fn main() -> Result<(), Error> {
    let numbers = vec![5, 8, 1, 2, 7, 3, 6, 9, 4, 10];
    println!("Array: {:?}, size: {}", &numbers, numbers.len());
    println!("CS: {:?}", counting_sort(&numbers));
    println!("SS: {:?}", selection_sort(&numbers));
    println!("BS: {:?}", bubble_sort(&numbers));
    println!("HS: {:?}", heap_sort(&numbers));
    println!("QS: {:?}", quick_sort(&numbers)?);

    Ok(())
}

fn selection_sort<T>(numbers: &[T]) -> Vec<T> where T: Ord + Clone {
    let mut clone = Vec::from(numbers);

    for key in 0..clone.len() {
        if let Some((smaller, _)) = clone.iter().enumerate().skip(key).min_by_key(|x| x.1) {
            clone.swap(key, smaller);
        }
    }

    clone
}

fn bubble_sort(numbers: &[i32]) -> Vec<i32> {
    let mut clone = Vec::from(numbers);
    let mut end = clone.len();

    while end != 0 {
        for number in 0..end - 1 {
            if clone[number] > clone[number + 1] {
                clone[number] ^= clone[number + 1];
                clone[number + 1] ^= clone[number];
                clone[number] ^= clone[number + 1];
            }
        }

        end -= 1;
    }

    clone
}

fn heap_sort(numbers: &[i32]) -> Vec<i32> {
    let mut clone = numbers.to_owned();
    let max_heap = |clone: &mut [i32], size: usize| {
        let mut count = (size as f32).log2().ceil() as i32;
        let mut indice: usize;
        while count >= 0 {
            let count_usize = count as usize;
            indice = (2 * count) as usize;
            if indice < size && clone[indice + 1] > clone[count_usize] {
                clone.swap(count_usize, indice + 1);
            }

            if indice + 2 <= size && clone[indice + 2] > clone[count_usize] {
                clone.swap(count_usize, indice + 2);
            }

            count -= 1;
        }
    };

    for count in (0..numbers.len()).rev() {
        max_heap(&mut clone, count);
        clone.swap(0, count);
    }

    clone
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

fn quick_sort(numbers: &[i32]) -> Result<Vec<i32>, Error> {
    let mut clone = numbers.to_owned();

    quick_sort_recursive(&mut clone, 0, numbers.len() - 1);

    Ok(clone)
}

fn counting_sort(list: &[i32]) -> Vec<i32> {
    let maximum_value: usize = *list
        .iter()
        .max()
        .expect("Não foi possível determinar o valor máximo")
        as usize;
    let mut counting_vector = vec![0; maximum_value];
    let mut output = vec![0; list.len()];

    list.iter()
        .for_each(|&number| counting_vector[number as usize - 1] += 1);

    counting_vector = counting_vector
        .iter()
        .scan(0, |state, &number| {
            *state += number;
            Some(*state)
        })
        .collect();

    list.iter().for_each(|&number| {
        let pos = number as usize - 1;
        output[counting_vector[pos] as usize - 1] = number;
        counting_vector[pos] -= 1;
    });

    output
}
