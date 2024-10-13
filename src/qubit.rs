use std::f64::consts::SQRT_2;

#[derive(Clone)]
pub struct Qubit {
    pub alpha: f64,
    pub beta: f64,
}

impl Default for Qubit {
    fn default() -> Self {
        Qubit {
            alpha: 1.0 / SQRT_2,
            beta: 1.0 / SQRT_2,
        }
    }
}

impl Qubit {
    pub fn update(&mut self, theta: f64, mod_signal: i32) {
        let new_alpha = self.alpha * (theta * mod_signal as f64).cos()
            - self.beta * (theta * mod_signal as f64).sin();
        let new_beta = self.beta * (theta * mod_signal as f64).sin()
            + self.alpha * (theta * mod_signal as f64).cos();

        self.alpha = new_alpha;
        self.beta = new_beta;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let qubit = Qubit::default();
        assert_eq!(qubit.alpha, 1.0 / SQRT_2);
        assert_eq!(qubit.beta, 1.0 / SQRT_2);
    }
}
