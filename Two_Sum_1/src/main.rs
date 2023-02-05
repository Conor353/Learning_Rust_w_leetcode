fn main() {
    let test_vec = vec![1,46,2,9,3,4];
    let result = two_sum(test_vec, 6);
    println!("{:?}",result)
}

//low memory but bad runtime

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut swap_vector = vec![0; nums.len()];

    for i in 0..nums.len() {
        swap_vector[i] = target - nums[i];
    }

    for j in 0..nums.len() {
        if swap_vector.contains(&nums[j]) {
            let pos_i = nums.iter().position(|&x| x == swap_vector[j]).unwrap();
            let pos_j = j as i32;

            if pos_i != j as usize {
                return vec![pos_i as i32, pos_j as i32];
            }
        }
    }

    return vec![];
}

