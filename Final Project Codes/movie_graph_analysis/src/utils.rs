pub mod utils {
    /// Normalizes a given value between a specified range.
    /// 
    /// # Arguments
    /// 
    /// * `value` - The value to normalize.
    /// * `min` - The minimum value of the range.
    /// * `max` - The maximum value of the range.
    /// 
    /// # Returns
    /// 
    /// A normalized value between 0 and 1.
    pub fn normalize(value: f32, min: f32, max: f32) -> f32 {
        if max - min == 0.0 {
            0.0
        } else {
            (value - min) / (max - min)
        }
    }

    /// Computes the average of values in a slice.
    /// 
    /// # Arguments
    /// 
    /// * `values` - A slice of f32 values.
    /// 
    /// # Returns
    /// 
    /// The average of the values.
    pub fn average(values: &[f32]) -> f32 {
        if values.is_empty() {
            0.0
        } else {
            let sum: f32 = values.iter().sum();
            sum / values.len() as f32
        }
    }
}
