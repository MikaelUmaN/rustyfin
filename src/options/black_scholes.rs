use statrs::distribution::{Normal, ContinuousCDF};

fn d1_f(s: f64, k: f64, t: f64, r: f64, sigma: f64) -> f64 {
    let sqrt_t = t.sqrt();
    let sigma_sqrt_t = sigma * sqrt_t;
    ((s / k).ln() + (r + 0.5 * sigma * sigma) * t) / sigma_sqrt_t
}

/// Compute d2 = d1 - sigma * sqrt(T)
fn d2_f(s: f64, k: f64, t: f64, r: f64, sigma: f64) -> f64 {
    d1_f(s, k, t, r, sigma) - sigma * t.sqrt()
}

/// Compute European call and put prices.
/// Formula (risk-neutral):
///   d1 = [ln(S/K) + (r + 0.5*sigma^2) * T] / (sigma * sqrt(T))
///   d2 = d1 - sigma*sqrt(T)
///   Call = S * N(d1) - K * exp(-r*T) * N(d2)
///   Put  = K * exp(-r*T) * N(-d2) - S * N(-d1)
/// where N(x) is the standard normal CDF.
///
/// # Arguments
///  - s: spot price (S)
///  - k: strike (K)
///  - t: time to maturity in years (T)
///  - r: continuously compounded risk-free rate
///  - sigma: volatility (annualized)
/// 
/// # Returns 
/// (call_price, put_price)
pub fn black_scholes(s: f64, k: f64, t: f64, r: f64, sigma: f64) -> (f64, f64) {
    // Basic edge handling: if T=0 or sigma=0 treat as intrinsic value.
    if t <= 0.0 || sigma <= 0.0 {
        let call = (s - k).max(0.0);
        let put = (k - s).max(0.0);
        return (call, put);
    }

    let d1 = d1_f(s, k, t, r, sigma);
    let d2 = d2_f(s, k, t, r, sigma);

    let stdn = Normal::new(0.0, 1.0).unwrap();
    let nd1 = stdn.cdf(d1);
    let nd2 = stdn.cdf(d2);
    let nmd1 = stdn.cdf(-d1);
    let nmd2 = stdn.cdf(-d2);
    let df = (-r * t).exp();

    let call = s * nd1 - k * df * nd2;
    let put = k * df * nmd2 - s * nmd1;
    (call, put)
}

#[cfg(test)]
mod tests {
    use super::black_scholes;

    #[test]
    fn test_basic_pricing() {
        // Classic example: S=100, K=100, T=1, r=0.05, sigma=0.2
        let (call, put) = black_scholes(100.0, 100.0, 1.0, 0.05, 0.2);
        // Reference values ~10.4506 (call), ~5.5735 (put)
        assert!((call - 10.45).abs() < 0.02, "call={} not close", call);
        assert!((put - 5.57).abs() < 0.02, "put={} not close", put);
    }

    #[test]
    fn test_intrinsic_edge() {
        let (call, put) = black_scholes(120.0, 100.0, 0.0, 0.05, 0.2);
        assert_eq!(call, 20.0);
        assert_eq!(put, 0.0);
    }
}