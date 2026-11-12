# Vulnerability: CVE-2020-35923

| **Information**       | **Details**                                                                                         |
| --------------------- | --------------------------------------------------------------------------------------------------- |
| **CVE**               | [CVE-2020-35923](https://www.cve.org/CVERecord?id=CVE-2020-35923)                                   |
| **Vulnerable Commit** | [e8fe06a](https://github.com/reem/rust-ordered-float/tree/e8fe06a58476ea70ac5d42d649bdf19f188f2b1c) |
| **Fixed Commit**      | [96a9099](https://github.com/reem/rust-ordered-float/tree/96a909975b0f7f4c4e87ca318e086c26ad2f4085) |
| **Variants**          | - [fixed-crate](fixed-crate)                                                                        |
|                       | - [fixed-function](fixed-function)                                                                  |
|                       | - [vuln-crate](vuln-crate)                                                                          |
|                       | - [vuln-function](vuln-function)                                                                    |

### Vulnerable Lines

`src/lib.rs`

```rust
// The issue is that the assignment operators such as add_assign, mul_assign, sub_assign, div_assign, rem_assign, it is possible for the resulting NotNan value to contain a Nan. This could cause undefined behavior in safe code, because the safe NotNan::cmp method contains internal unsafe code that assumes the value is never Nan.

/// Adds a float directly.
///
/// Panics if the provided value is NaN.
impl<T: Float + AddAssign> AddAssign<T> for NotNan<T> {
    fn add_assign(&mut self, other: T) {
        // Vulnerable line
        self.0 += other;
        assert!(!self.0.is_nan(), "Addition resulted in NaN");
    }
}

/// Subtracts a float directly.
///
/// Panics if the provided value is NaN or the computation results in NaN
impl<T: Float + SubAssign> SubAssign<T> for NotNan<T> {
    fn sub_assign(&mut self, other: T) {
        // Vulnerable line
        self.0 -= other;
        assert!(!self.0.is_nan(), "Subtraction resulted in NaN");
    }
}

/// Multiplies a float directly.
///
/// Panics if the provided value is NaN.
impl<T: Float + MulAssign> MulAssign<T> for NotNan<T> {
    fn mul_assign(&mut self, other: T) {
        // Vulnerable line
        self.0 *= other;
        assert!(!self.0.is_nan(), "Multiplication resulted in NaN");
    }
}

/// Divides a float directly.
///
/// Panics if the provided value is NaN or the computation results in NaN
impl<T: Float + DivAssign> DivAssign<T> for NotNan<T> {
    fn div_assign(&mut self, other: T) {
        // Vulnerable line
        self.0 /= other;
        assert!(!self.0.is_nan(), "Division resulted in NaN");
    }
}

/// Calculates `%=` with a float directly.
///
/// Panics if the provided value is NaN or the computation results in NaN
impl<T: Float + RemAssign> RemAssign<T> for NotNan<T> {
    fn rem_assign(&mut self, other: T) {
        // Vulnerable line
        self.0 %= other;
        assert!(!self.0.is_nan(), "Rem resulted in NaN");
    }
}

```
