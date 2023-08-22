
fn main() {
    // the main funciton
    let _vec = vec![5,6,4,6,5];
    let _vec2 = vec![4,1,2,1,2];
    let _vec3 = vec![11, 0, 0, 3, 11];
    let _vec4 = vec![3,2,1];
    let _vec5 = vec![1,1,2,3,4,4,3];
//    println!("Non Double: {}", other_num(_vec));
//    println!("Non Double: {}", other_num(_vec2));
//    println!("Non Double: {}", other_num(_vec3));
    //third_try(_vec4);
    //println!("Non double: {}", fourth_try(_vec5));
    //println!("Non Double: {}", fourth_try(_vec));
    //println!("Non double: {}", fourth_try(_vec2));
    println!("Non Double: {}, shoudl be 4", fifth_try(_vec));
    println!("Non double: {}, should be 2", fifth_try(_vec5));
    println!("Non double: {}, should be 3", fifth_try(_vec3));
}

fn single_number(nums: Vec<i32>) -> i32 {
    // takes in an array of numbers
    // some numbers appear twice, find that one that appears once
    let da_nums = nums;
    let mut hold_arr = Vec::with_capacity(da_nums.len());
    let mut ret_num = da_nums[0];

    for i in 0..da_nums.len() {
        println!("On loop: {}", i);
        println!("Current num is: {}", da_nums[i]);
        if !hold_arr.contains(&da_nums[i]) {
            hold_arr.push(da_nums[i]);
            ret_num = da_nums[i];
        }
    }
    return ret_num;
    
}

fn other_num(nums: Vec<i32>) -> i32 {
    let da_nums = nums;
    let mut current_winner_index = 0;
    let mut seen_arr = Vec::with_capacity(da_nums.len());
    seen_arr.push(da_nums[0]);

    for i in 1..da_nums.len() {
        //If the value at the current winner is equal to the value at i 
        // then it can't be the value so we'll inc current winner position
        // and also add
    //    if da_nums[i] == da_nums[current_winner_index] || seen_arr.contains(&da_nums[i]) {
    //        current_winner_index = current_winner_index + 1;
    //    } else {
    //        seen_arr.push(da_nums[i])
    //    }

        if !seen_arr.contains(&da_nums[i]) {
            seen_arr.push(da_nums[i]);
        } else if da_nums[current_winner_index] == da_nums[i] {
            current_winner_index = current_winner_index +1;
        }
    }
    return da_nums[current_winner_index];
}

//fn third_try(nums: Vec<i32>) -> i32 {
//    // ughhhh this is so dumbbbb
//    let da_nums = nums;
//    //let mut da_string = "";
//    let mut curr_num = "";
//    for i in 0..da_nums.len() {
//       //curr_num = curr_num.push_str(&da_nums[i].to_string());
//       println!("Current num is {}", curr_num);
//    }
//
//    return 0;
//
//}

fn fourth_try(nums: Vec<i32>) -> i32 {
    //worked :|
    let da_nums = nums;
    let mut result = 0;
    for i in 0..da_nums.len() {
        result = result^da_nums[i];
    }
    return result;
}

fn fifth_try(nums: Vec<i32>) -> i32 {
    let da_nums = nums;
    let mut res = da_nums[0];
    let mut seen_arr = Vec::with_capacity(da_nums.len());
    seen_arr.push(da_nums[0]);
    for i in 1..da_nums.len() {
        if seen_arr.contains(&da_nums[i]) {
            res -= da_nums[i]
        } else {
            res += da_nums[i];
            seen_arr.push(da_nums[i]);
        }
    }
    return res;
}
