use tracing::debug;

pub struct PID {
    // PID Gains
    kp: f64,
    ki: f64,
    kd: f64,
    // Setpoint
    sp: f64,
    // Process Variable
    pv: f64,
    // Continuous input range
    continuous_input: bool,
    input_min: f64,
    input_max: f64,
    // Integral Error
    err_sum: f64,
    // Previous Error
    errf_prev: f64,
    // Output clamp
    min: f64,
    max: f64,
    // add an optional label (for debugging)
    debug_label: String,
}

impl PID {
    pub fn new(kp: f64, ki: f64, kd: f64) -> PID {
        PID {
            kp,
            ki,
            kd,
            sp: 0.0,
            pv: 0.0,
            err_sum: 0.0,
            errf_prev: 0.0,
            min: f64::MIN,
            max: f64::MAX,
            continuous_input: false,
            input_min: 0.0,
            input_max: 0.0,
            debug_label: String::from(""),
        }
    }

    pub fn add_debug_label(&mut self, s: &str) {
        self.debug_label = String::from("(");
        self.debug_label.push_str(s);
        self.debug_label.push_str(") ");
    }

    pub fn enable_continuous_input(&mut self, min: f64, max: f64) {
        self.continuous_input = true;
        self.input_min = min;
        self.input_max = max;
    }

    pub fn disable_continuous_input(&mut self) {
        self.continuous_input = false;
    }

    pub fn reset(&mut self) {
        self.sp = 0.0;
        self.pv = 0.0;
        self.err_sum = 0.0;
        self.errf_prev = 0.0;
    }

    pub fn set_gains(&mut self, kp: f64, ki: f64, kd: f64) {
        self.kp = kp;
        self.ki = ki;
        self.kd = kd;
    }

    pub fn set_output_range(&mut self, min: f64, max: f64) {
        self.min = min;
        self.max = max;
    }

    pub fn set_sp(&mut self, sp: f64) {
        self.sp = sp;
    }

    pub fn set_pv(&mut self, pv: f64) {
        self.pv = pv;
    }

    pub fn step(&mut self, dt: Option<f64>) -> f64 {
        // Calculate Error; normalize if input is continuous
        let mut err = self.sp - self.pv;
        if self.continuous_input {
            err = self.normalize_error(err);
        }
        let mut errf = 0.05 * err + (1. - 0.05) * self.errf_prev;
        if self.continuous_input {
            errf = self.normalize_error(errf);
        }
        // Error summation for integral portion
        self.err_sum += err;
        // Error rate of change for derivative portion
        let mut errf_dt: f64 = 0.0;
        // Ignore D value on first step (dt will be None)
        if let Some(dt) = dt {
            errf_dt = (errf - self.errf_prev) / dt;
        }
        self.errf_prev = errf;

        // Calculate output
        let mut output = (err * self.kp) + (self.err_sum * self.ki) + (errf_dt * self.kd);
        // Clamp output within desired range
        output = self.clamp_output(output);

        debug!(
            "{}SP: {}, PV: {}, ERR: {}, ERR_SUM: {}, ERR_DT: {}, OUT: {}",
            self.debug_label, self.sp, self.pv, err, self.err_sum, errf_dt, output
        );

        output
    }

    fn clamp_output(&self, val: f64) -> f64 {
        val.max(self.min).min(self.max)
    }

    fn normalize_error(&self, error: f64) -> f64 {
        let err_bound: f64 = (self.input_max - self.input_min) / 2.0;
        let min_input: f64 = -err_bound;
        let max_input: f64 = err_bound;
        let mut input: f64 = error;
        let modulus: f64 = max_input - min_input;
        let num_max: i32 = ((input - min_input) / modulus) as i32;
        input -= num_max as f64 * modulus;
        let num_min: i32 = ((input - max_input) / modulus) as i32;
        input -= num_min as f64 * modulus;

        input
    }
}

#[cfg(test)]
mod tests {
    use crate::PID;
    use proptest::prelude::*;

    #[test]
    fn test_norm_error_accuracy() {
        let mut pid = PID::new(0.0, 0.0, 0.0);
        pid.enable_continuous_input(-180.0, 180.0);
        let tests = [(275.3, -84.7), (60.0, 60.0)];
        let fp_error = 0.001;
        for (t, a) in tests {
            let nt = pid.normalize_error(t);
            assert!((a - fp_error) <= nt);
            assert!((a + fp_error) >= nt);
        }
    }

    proptest! {
        #![proptest_config(ProptestConfig {
        cases: 100000, .. ProptestConfig::default()
        })]
        #[test]
        fn test_norm_error_bounds(a in -1000000.0f64..1000000.0) {
            let mut pid = PID::new(0.0, 0.0, 0.0);
            pid.enable_continuous_input(-180.0, 180.0);
            // NOTE: This prop test will verify that norm_error will always
            // return a value between -180.0 and 180.0
            let na = pid.normalize_error(a);
            prop_assert!((-180.0..=180.0).contains(&na),
                "Error not within bounds: {} => {}", a, na);
        }
    }
}
