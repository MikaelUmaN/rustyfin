use crate::optimization::secant;
use crate::options::black_scholes;

/// Calculates implied volatility by solving it for the market price, using the Black-Scholes formula.
/// 
/// # Arguments
///  - p: market price of the option
///  - s: spot price (S)
///  - k: strike (K)
///  - t: time to maturity in years (T)
///  - r: continuously compounded risk-free rate
///  - is_call: true for call option, false for put option
/// # Returns 
/// (volatility)
pub fn implied_volatility(p: f64, s: f64, k: f64, t: f64, r: f64, is_call: bool) -> f64 {
    if is_call {
        let f = |sigma: f64| {
            let (call_price, _) = black_scholes(s, k, t, r, sigma);
            call_price - p
        };
        secant(f, 0.1, 0.3, 1e-6, 1e-6, 50).expect("Implied volatility calculation failed").root
    } else {
        let f = |sigma: f64| {
            let (_, put_price) = black_scholes(s, k, t, r, sigma);
            put_price - p
        };
        secant(f, 0.1, 0.3, 1e-6, 1e-6, 50).expect("Implied volatility calculation failed").root
    }
}

#[cfg(test)]
mod tests {
    use super::implied_volatility;
    use crate::options::black_scholes;
    use approx::assert_relative_eq;

    #[test]
    fn test_implied_volatility_call() {
        // Given market price, recover volatility
        let s = 100.0;
        let k = 100.0;
        let t = 1.0;
        let r = 0.05;
        let sigma = 0.2;
        let (call_price, _) = black_scholes(s, k, t, r, sigma);
        let implied_vol = implied_volatility(call_price, s, k, t, r, true);
        assert_relative_eq!(implied_vol, sigma, epsilon=1e-4);
    }

    #[test]
    fn test_implied_volatility_put() {
        // Given market price, recover volatility
        let s = 100.0;
        let k = 100.0;
        let t = 1.0;
        let r = 0.05;
        let sigma = 0.2;
        let (_, put_price) = black_scholes(s, k, t, r, sigma);
        let implied_vol = implied_volatility(put_price, s, k, t, r, false);
        assert_relative_eq!(implied_vol, sigma, epsilon=1e-4);
    }
}
