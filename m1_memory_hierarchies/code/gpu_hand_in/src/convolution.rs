// use crate::utility::{GPUHandles, mean_square_error, are_vectors_equivalent, Uniform, run_compute_shader}, gpu_vector::GPUVector;
use crate::{utility::{GPUHandles, mean_square_error, are_vectors_equivalent, Uniform, run_compute_shader}, gpu_vector::GPUVector};

// The length of filter is assumed to be oddly number, i.e. 1, 3, 5, 7, 9, 11
fn convolution_cpu(signal: &Vec<f32>, filter: &Vec<f32>) -> Vec<f32> {
    let filter_offset = filter.len() / 2;
    let mut output: Vec<f32> = vec![0.0; signal.len()];
    for signal_index in 0..signal.len() {
        for filter_index in 0..filter.len() {
            let offset_signal_index: i64 = signal_index as i64 - filter_offset as i64 + filter_index as i64;
            if -1 < offset_signal_index && offset_signal_index < signal.len() as i64 {
                output[signal_index] += signal[offset_signal_index as usize] * filter[filter_index];
            }
        }
    }

    output
}

fn test_ground_truth() -> bool {
    let signal: Vec<f32> = vec![ 1.0, 1.0, 1.0, 1.0, 1.0];
    let filter: Vec<f32> = vec![ 0.25, 0.5, -0.25];
    let ground_truth_output: Vec<f32> = vec![ 0.25, 0.5, 0.5, 0.5, 0.75];

    let output: Vec<f32> = convolution_cpu(&signal, &filter);

    assert!(output.len() == ground_truth_output.len());
    for index in 0..ground_truth_output.len() {
        if 0.00001 < (output[index] - ground_truth_output[index]).abs() {
            println!("Provided output: {:?}", output);
            println!("Ground truth output: {:?}", ground_truth_output);
            return false;
        }
    }

    true
}

pub fn convolution(handles: &GPUHandles) -> bool {

    // A small test to ensure that the convolution_cpu function is actually correct.
    let ground_truth_is_correct: bool = test_ground_truth();
    println!("Convolution ground truth function is correct: {}", ground_truth_is_correct);
    assert!(ground_truth_is_correct);

    // Setup our CPU-side data
    let element_count: usize = 100000;
    // let element_count: usize = 7;
    let filter_size: usize = 19;
    let signal: Vec<f32> = (0..element_count).map(|x| x as f32 * 0.1).collect();
    let filter: Vec<f32> = (0..filter_size).map(|x| x as f32 * 0.1).collect();
    let filt_offset: usize = (filter.len() / 2) as usize;
    let filt_len: usize = filter.len() as usize;
    let output_naive: Vec<f32> = vec![0.0; element_count];
    let output_padded: Vec<f32> = vec![0.0; element_count];
    let output_shared: Vec<f32> = vec![0.0; element_count];

    let ground_truth: Vec<f32> = convolution_cpu(&signal, &filter);

    // 1) Do 1D convolution on the GPU, don't use shared memory.
    // Make sure to keep the filter and signal large enough to offset the cost of data transfer
    
    // HINT - You need a run_compute_shader() call per type of compute shader.
    // Figure out what the arguments are supposed to be (see vector_add.rs) and
    // call the correct shader function in the correct shader file.

    
    // Create our uniform for telling the shader how big the vectors are.
    let uniform: Uniform = Uniform::new(handles, element_count, filt_offset, filt_len, 0);

    // Create the GPU vectors.
    // Note the true at the end of the output vector creation.
    // This will result in a staging_buffer being created, which we
    // can read from on the CPU.

    let signal: GPUVector = GPUVector::new(&handles, signal, "signal", false);
    let filter: GPUVector = GPUVector::new(&handles, filter, "filter", false);

    let mut output_naive: GPUVector = GPUVector::new(&handles, output_naive, "output_naive", true);
    let mut output_padded: GPUVector = GPUVector::new(&handles, output_padded, "output_padded", true);
    let mut output_shared: GPUVector = GPUVector::new(&handles, output_shared, "output_shared", true);

    // We will use 32 threads in a work group/warp
    // We are doing this in 1 dimension, but could do it in
    // up to 3 dimensions.

    let block_size_x: usize = 32;
    let launch_blocks_x: u32 = ((element_count + block_size_x - 1) / block_size_x) as u32;
    let block_size_y: usize = 1;
    let launch_blocks_y: u32 = 1;
    let shader_file: &'static str = include_str!("convolution_naive.wgsl");
    let shader_function: &str = "convolution";

    run_compute_shader(
        handles,
        block_size_x, 
        launch_blocks_x,
        block_size_y,
        launch_blocks_y,
        shader_file,
        shader_function,
        &uniform,
        &signal,
        &filter,
        &mut output_naive,
    );

    // print the output
    // println!("output_naive: {:?}", output_naive.cpu_data);
    let data_naive: Vec<f32> = output_naive.cpu_data; // Remove this and replace with your own data

    // 2) Make a new version of 1D convolution which uses shared memory.
    // See which is the fastest, is it the signal in shared memory, is it the filter in
    // shared memory, is it both?
    // What happens when you set the block size to different multiples of 32? Why do you think that is?


    let shader_file: &'static str = include_str!("convolution_shared.wgsl");
    run_compute_shader(
        handles,
        block_size_x, 
        launch_blocks_x,
        block_size_y,
        launch_blocks_y,
        shader_file,
        shader_function,
        &uniform,
        &signal,
        &filter,
        &mut output_shared,
    );    
    let data_shared: Vec<f32> = output_shared.cpu_data; // Remove this and replace with your own data


    // 3) Make another version using a zero padded version of the original signal. Do not use any if's
    // inside the inner for-loop. This zero padding is (filter_size - 1) / 2 on each side of the
    // signal. What happens if you increase the padding with 0's to ensure that the signal is
    // always a multiple of your block size? HINT - You should be able to remove the outer if-guard.

    // Create padded signal
    let mut signal_padded = vec![0.0; element_count + filter_size - 1];
    for i in 0..element_count {
        signal_padded[i + (filter_size - 1) / 2] = signal.cpu_data[i];
    }

    // // Create the GPU vectors.
    // let signal: GPUVector = GPUVector::new(&handles, signal_padded, "signal", false);
    // let output_padded = vec![0.0; element_count];
    // let mut output_padded: GPUVector = GPUVector::new(&handles, output_padded, "output_padded", true);

    let shader_file: &'static str = include_str!("convolution_padded.wgsl");
    run_compute_shader(
        handles,
        block_size_x, 
        launch_blocks_x,
        block_size_y,
        launch_blocks_y,
        shader_file,
        shader_function,
        &uniform,
        &signal,
        &filter,
        &mut output_padded,
    );    
    let data_padded: Vec<f32> = output_padded.cpu_data; // Remove this and replace with your own data


    // Naive
    println!("convolution naive MSE: {}", mean_square_error(&ground_truth, &data_naive));
    let success: bool = are_vectors_equivalent(&ground_truth, &data_naive);
    println!("convolution naive success: {}!", success);

    // Tiled
    println!("convolution tiled MSE: {}", mean_square_error(&ground_truth, &data_shared));
    let success: bool = are_vectors_equivalent(&ground_truth, &data_shared);
    println!("convolution tiled success: {}!", success);
    // println!("data_shared: {:?}", data_shared);

    // Padded
    println!("convolution padded MSE: {}", mean_square_error(&ground_truth, &data_padded));
    let success: bool = are_vectors_equivalent(&ground_truth, &data_padded);
    println!("convolution padded success: {}!", success);

    success
}