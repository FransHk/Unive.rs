/// Add two arrays arrays together (elementwise)
pub fn add_arrays(array1: [f64; 2], array2: [f64; 2]) -> [f64; 2] {
    [array1[0] + array2[0], array1[1] + array2[1]]
}

/// Subtract two arrays (elementwise)
pub fn subtract_arrays(array1: [f64; 2], array2: [f64; 2]) -> [f64; 2] {
    [array1[0] - array2[0], array1[1] - array2[1]]
}

/// Allows for scalar multiplication with an array
/// that captures either position or velocity
pub fn scalar_mult(array: [f64; 2], scalar: f64) -> [f64; 2] {
    let mut result = [0.0, 0.0];
    for (i, &element) in array.iter().enumerate() {
        result[i] = element * scalar;
    }
    result
}

/// Returns the dot-product of two arrays as an f64
pub fn dot_product(array_1: [f64; 2], array_2: [f64; 2]) -> f64 {
    array_1[0] * array_2[0] + array_1[1] * array_2[1]
}

/// Gets the length of a vector
pub fn get_length(array: [f64; 2]) -> f64 {
    f64::sqrt(array[0] * array[0] + array[1] * array[1])
}

/// Returns a normalised vector
pub fn normalise_vector(array: [f64; 2]) -> [f64; 2] {
    let denom = array[0] * array[0] + array[1] * array[1];
    let denom = f64::sqrt(denom);
    let frac = 1.0 / denom; // Get normalisation scalar
    scalar_mult(array, frac) // Mult vector with norm scalar to get norm vector
}
