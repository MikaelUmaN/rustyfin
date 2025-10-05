/// What convergence criterion triggered success.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConvergenceType {
    /// |x_n - x_{n-1}| <= xtol
    XTolerance,
    /// |f(x_n)| <= ftol
    FTolerance,
}

/// Errors that can occur during the secant root-finding method.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SecantError {
    /// The method failed to converge within the maximum number of iterations.
    MaxIterationsExceeded,
    /// The denominator (f(x1) - f(x0)) became too small indicating a possible repeated root or stagnation.
    DivisionByZero,
}

/// Result of a successful secant method execution.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SecantOk {
    /// Estimated root.
    pub root: f64,
    /// Number of iterations performed (0-based).
    pub iterations: usize,
    /// Which convergence criterion triggered termination.
    pub convergence_type: ConvergenceType,
}

/// Secant root-finding method.
///
/// # Arguments
/// - `f`: function whose root is sought.
/// - `x0`, `x1`: initial guesses (should bracket or be reasonably close to a root).
/// - `xtol`: absolute tolerance for successive x differences.
/// - `ftol`: absolute tolerance for function value.
/// - `max_iter`: maximum number of iterations.
///
/// # Returns
/// [`Result`]<[`SecantOk`], [`SecantError`]>`
pub fn secant(
    f: impl Fn(f64) -> f64,
    x0: f64,
    x1: f64,
    xtol: f64,
    ftol: f64,
    max_iter: usize,
) -> Result<SecantOk, SecantError> {
    let mut x0 = x0;
    let mut x1 = x1;
    for i in 0..max_iter {
        let f0 = f(x0);
        let f1 = f(x1);
        if f1.abs() < ftol {
            return Ok(SecantOk { root: x1, iterations: i, convergence_type: ConvergenceType::FTolerance });
        }
        else if (f1 - f0).abs() < ftol {
            return Err(SecantError::DivisionByZero);
        }
        let x2 = x1 - f1 * (x1 - x0) / (f1 - f0);
        if (x2 - x1).abs() < xtol{
            return Ok(SecantOk { root: x2, iterations: i, convergence_type: ConvergenceType::XTolerance } );
        }
        x0 = x1;
        x1 = x2;
    }
    Err(SecantError::MaxIterationsExceeded)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_secant_simple_root() {
        // Root at x=2 for f(x)=x^2-4
        let f = |x: f64| x * x - 4.0;
        let res = secant(f, 1.0, 3.0, 1e-12, 1e-12, 50).expect("Expected convergence on trivial example.");
        assert!((res.root - 2.0).abs() < 1e-9, "root ≈ {}", res.root);
    }
}