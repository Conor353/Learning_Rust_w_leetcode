fn main() {
    println!("{:?}", generate(4));

}


//fn generate(num_rows: i32) -> Vec<Vec<i32>> {

fn generate(num_rows: i32) -> Vec<Vec<i32>> {



// [1,2,1] -> [1,3,3,1]
fn next_row(input_vec: Vec<i32>) -> Vec<i32> {
    
    let num_rows_input = input_vec.len();
    
    // make empty output vector
    let mut out_vector = vec![0; input_vec.len() + 1];

    // add 1 to edges
    out_vector[0] = 1;
    out_vector[input_vec.len()] = 1;

    for i in 1..input_vec.len() {
        out_vector[i] = input_vec[i - 1] + input_vec[i];
    }

    return out_vector;
}




    
    let mut row_count = 0;

    let mut starting_vector = vec![vec![1]];

    while row_count < (num_rows -1) {
        
        let current_row = &starting_vector[row_count as usize];
        let next_row = next_row(current_row.to_vec());
        starting_vector.push(next_row);

        row_count += 1;
    }
    return starting_vector;
}

