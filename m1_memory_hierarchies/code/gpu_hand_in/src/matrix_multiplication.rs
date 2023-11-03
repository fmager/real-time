use crate::utility::{GPUHandles, mean_square_error, are_vectors_equivalent};

fn matrix_multiplication_cpu(
    left_matrix: &Vec<f32>,
    right_matrix: &Vec<f32>,
    outer_dimension_left_length: usize,
    inner_dimension_length: usize,
    outer_dimension_right_length: usize,
) -> Vec<f32> {
    let mut output: Vec<f32> = vec![0.0; outer_dimension_left_length * outer_dimension_right_length];
    for row_output in 0..outer_dimension_left_length {
        for column_output in 0..outer_dimension_right_length {
            for inner_dimension in 0..inner_dimension_length {
                output[row_output * outer_dimension_right_length + column_output] += left_matrix
                    [row_output * inner_dimension_length + inner_dimension]
                    * right_matrix[inner_dimension * outer_dimension_right_length + column_output];
            }
        }
    }
    output
}

pub fn matrix_multiplication(handles: &GPUHandles) -> bool {
    let outer_dimension_left: usize = 40; // M
    let inner_dimension: usize = 32; // N
    let outer_dimension_right: usize = 54;// K
    let left_matrix: Vec<f32> = (0..outer_dimension_left*inner_dimension).map(|x| x as f32 * 0.1).collect();
    let right_matrix: Vec<f32> = (0..inner_dimension*outer_dimension_right).map(|x| x as f32 * 0.1).collect();

    let ground_truth: Vec<f32> = matrix_multiplication_cpu(&left_matrix, &right_matrix, outer_dimension_left, inner_dimension, outer_dimension_right);
    let dummy_data: Vec<f32> = ground_truth.clone();
    
    //
    // YOUR CODE HERE
    // Make one version of matrix multiplication, ensure that it is correct
    // Make another version which uses tiling through shared memory and local accumulation in
    // a register. How big do the matrices have to be before you see a big performance
    // difference? Why do you think that is?

    println!("matrix_multiplication MSE: {}", mean_square_error(&ground_truth, &dummy_data));
    let success: bool = are_vectors_equivalent(&ground_truth, &dummy_data);
    println!("matrix_multiplication success: {}!", success);

    success
}