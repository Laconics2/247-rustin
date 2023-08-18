
fn main() {
    let vec = vec![3,2,4];
    let vec2 = vec![2,5,5,11];
    let vec3 = vec![1, 0, 0, 0, 11];
    println!("Answer: {:?}", two_sum(vec, 6));
    println!("Answer: {:?}", two_sum(vec2, 10));
    println!("Answer: {:?}", two_sum(vec3, 12));

}

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32>{
    let da_nums = nums;
    let mut _snug = da_nums.iter();
    let mut _snug2 = da_nums.iter();
    let mut ret_arr = Vec::with_capacity(2);
    println!("Given Target {}", target);
    println!("Array: {:?}", da_nums);

    //for first_val in iterator {
    //    for current_num in iterator {
    //        if (first_val + current_num) == target {
    //           //ret_arr.append(&slider);
    //           //ret_arr.append(val);
    //           ret_arr.push(target);
    //           ret_arr.push(*current_num);
    //           return ret_arr;
    //        }
    //    }
    //}
    //

//    while let Some(iterator) = iterator.next() {
//        let mut some_num = 0;
//        for number in iterator {
//            if (da_nums[some_num] + number) == target {
//                ret_arr.push(*number);
//                ret_arr.push(da_nums[some_num]);
//                return ret_arr;
//            }
//
//        }
//    }

    let mut i = 0;
    while let Some(_snug) = _snug.next() {
        println!("Outer loop i: {}", i);
        //let mut j = 1;
        for j in (i+1)..da_nums.len() {
            println!("Inner loop j: {}", j);
          // if j == da_nums.len(){
          //      //full dumbass mode cause ughhhh
          //      j = j - 1;
          //  }
            if (da_nums[i] + da_nums[j]) == target {
                ret_arr.push(i as i32);
                ret_arr.push(j as i32);
                return ret_arr;
            }

            //j = j + 1;
        }

        i = i + 1;
    }


    return vec![0, 0];

}
