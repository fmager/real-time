// This doesn't actually need to be a
// struct. We could just have u32,
// but if we were using two- or more
// dimensions, having more dimensions
// would greatly cut down on the 
// amount of binding slots used.
// There is a limitation on those.

struct Uniform {
    element_count: u32,
    filt_offset: u32,
    filt_len: u32,
    not_used: u32,
};

// We can have different bind groups
// which each have their own set of bindings.
// var<uniform> means it is a read-only
// set of values which all threads can safely
// load in its entirety and it won't be updated.
@group(0) @binding(0)
var<uniform> dimensions: Uniform;

// Bind a read only array of 32-bit floats
@group(0) @binding(1)
var<storage, read> signal: array<f32>;

@group(0) @binding(2)
var<storage, read> filt: array<f32>;

// Bind a read/write array
@group(0) @binding(3)
var<storage, read_write> output_naive: array<f32>;

// With @compute we define that this is a compute shader.
// With @workgroup_size we define the size of the warp. 
// If we had a lot of register pressure we might
// define it as some lower 2^n number instead,
// or if we were doing something two-dimensional,
// it might for example make sense to use @workgroup_size(8, 8, 1)
// or @workgroup_size(16, 2, 1)
// @compute @workgroup_size(32, 1, 1)
@compute @workgroup_size(32, 1, 1)
fn convolution(
    // For this example we only need access to the global
    // thread ID.
    @builtin(global_invocation_id) global_id: vec3<u32>,
    // But you can also gain other more localized ID's
    //@builtin(workgroup_id) group_id: vec3<u32>, 
    //@builtin(local_invocation_id) local_id: vec3<u32>
    ) {
    // Get the global thread ID. Since we are
    // only working in one dimension, we only
    // need the ID in one dimension.
    let thread_id: u32 = global_id.x;

    // We need to know the offset of the filter
    // so we can make sure we don't go out of bounds
    // of the signal array.
    // let filt_offset: u32 = u32(f32(arrayLength(&filt)) / 2.0);
    let filt_offset: u32 = dimensions.filt_offset;
    let filt_len: u32 = dimensions.filt_len;

    // let sig: f32 = signal[0];
    // let fil: f32 = filt[0];

    // Make sure we are inside the valid range of the
    // arrays, if not, do nothing.

    // Naive convolution on cpu
    // let filter_offset = filter.len() / 2;
    // let mut output: Vec<f32> = vec![0.0; signal.len()];
    // for signal_index in 0..signal.len() {
    //     for filter_index in 0..filter.len() {
    //         let offset_signal_index: i64 = signal_index as i64 - filter_offset as i64 + filter_index as i64;
    //         if -1 < offset_signal_index && offset_signal_index < signal.len() as i64 {
    //             output[signal_index] += signal[offset_signal_index as usize] * filter[filter_index];
    //         }
    //     }
    // }
    
    if (thread_id < dimensions.element_count) {
        var filt_index: u32 = 0u;
        // output_naive[thread_id] = f32(thread_id);
        loop {
            if (filt_index >= filt_len){
                break;
                }
            let offset_signal_index: i32 = i32(thread_id) - i32(filt_offset) + i32(filt_index);
            
            // output_naive[thread_id] = f32(offset_signal_index);
            if -1 < offset_signal_index && offset_signal_index < i32(dimensions.element_count) {
                output_naive[thread_id] += signal[offset_signal_index] * filt[filt_index];
                // output_naive[thread_id] += f32(offset_signal_index);
            }
            continuing { filt_index = filt_index + 1u; }
        }  
    }
}