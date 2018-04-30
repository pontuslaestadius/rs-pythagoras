use std::f64;


/// Calculates Pythagoras.
/// #Examples
/// ```
/// let a = 3;
/// let b = 4;
/// let c = pythagoras::equation(a, b);
/// assert_eq!(5f64, c);
/// ```
pub fn equation<T: Into<f64>>(a: T, b: T) -> f64 {
    let c: f64 = a.into().powi(2) + b.into().powi(2);
    return c.sqrt();
}