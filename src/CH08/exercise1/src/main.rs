struct ReturnValue {
    mean: f32,
    median: f32,
    mode: Option<i32>,
}

struct ModeStruct {
    key: i32,
    value: i32,
}
fn main() {
    let list = vec![1, 2, 3, 3, 4, 4, 4];
    let result = test(&list);
    match result.mode {
        Some(i) => println!(
            "mean = {}, median = {}, mode = {}",
            result.mean, result.median, i
        ),
        None => println!(
            "mean = {}, median = {}, mode = None",
            result.mean, result.median
        ),
    }
}

fn test(list: &Vec<i32>) -> ReturnValue {
    // mean can be decimal number
    let mean: f32;

    // median can be the mean of two middle numbers, so type is `f32`
    let median: f32;

    // mode can be none, so type is `Option<i32>`
    let mut mode: Option<i32> = None;

    let mut sum = 0;

    for i in list {
        sum += i;
    }

    mean = (sum as f32) / (list.len() as f32);

    let mut sorted_list = list.clone();
    sorted_list.sort();

    if list.len() % 2 == 0 {
        // divide by 2, median will be average of 2 numbers
        median = (sorted_list[list.len() / 2] + sorted_list[list.len() / 2 - 1]) as f32 / 2.0;
    } else {
        // cannot divide by 2, only one number = median
        median = sorted_list[(list.len() - 1) / 2] as f32;
    }

    use std::collections::HashMap;

    let mut map: HashMap<i32, i32> = HashMap::new(); // frequency of each number

    for i in list {
        let count = map.entry(*i).or_insert(0);
        *count += 1;
    }

    if map.len() == 1 {
        // the list have only one number such as [1, 1, 1], so the number is the mode
        for (key, _value) in map {
            mode = Some(key);
        }
    } else {
        // the list have more than one number
        // sort by value, then compare the first 2 numbers

        let mut freq_vec = Vec::new();

        let mut map_to_vec: Vec<ModeStruct> = Vec::new();
        for (key, value) in map {
            map_to_vec.push(ModeStruct {
                key: key,
                value: value,
            });
            freq_vec.push(value);
        }
        freq_vec.sort();
        freq_vec.reverse();

        let have_mode_flag: bool = &freq_vec[0] != &freq_vec[1]; // use a flag to indicate whether have mode

        if have_mode_flag {
            for each_mode in map_to_vec {
                if each_mode.value == freq_vec[0] {
                    mode = Some(each_mode.key);
                }
            }
        } else {
            mode = None;
        }
    }

    ReturnValue {
        mean: mean,
        median: median,
        mode: mode,
    }
}
