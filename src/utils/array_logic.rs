pub trait Length {
    fn get_length(&self) -> f32;
}

pub trait Normalise {
    fn normalise(&self) -> [f32; 2];
}

/// Extension that normalises a vector by dividing
/// its components by its length.
impl Normalise for [f32; 2] {
    fn normalise(&self) -> [f32; 2] {
        let length = &self.get_length();
        let frac = 1.0 / length;
        scalar_mult(*self, frac)
    }
}

/// Extension that returns the length of a vector
impl Length for [f32; 2] {
    fn get_length(&self) -> f32 {
        f32::sqrt(dot_product(*self, *self))
    }
}

/// Subtract two arrays (element wise)
pub fn subtract_arrays(array1: [f32; 2], array2: [f32; 2]) -> [f32; 2] {
    [array1[0] - array2[0], array1[1] - array2[1]]
}

/// Allows for scalar multiplication with an array
/// that captures either position or velocity
pub fn scalar_mult(array: [f32; 2], scalar: f32) -> [f32; 2] {
    let mut result = [0.0, 0.0];
    for (i, &element) in array.iter().enumerate() {
        result[i] = element * scalar;
    }
    result
}

/// Returns the dot-product of two arrays as an f32
pub fn dot_product(array_1: [f32; 2], array_2: [f32; 2]) -> f32 {
    array_1[0] * array_2[0] + array_1[1] * array_2[1]
}

/// Add two arrays arrays together (elementwise)
pub fn add_arrays(array1: [f32; 2], array2: [f32; 2]) -> [f32; 2] {
    [array1[0] + array2[0], array1[1] + array2[1]]
}
