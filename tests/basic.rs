use libpid::PID;

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_proportional_gain() {
        let mut pid = PID::new(0.5, 0.0, 0.0);
        pid.set_sp(10.0);
        pid.set_pv(0.0);
        let dt = 1.0;
        let out = pid.step(Some(dt));
        assert!(out == 5.0, "output is {out}, expected 5.0");
        let out = pid.step(Some(dt));
        assert!(out == 5.0, "output is {out}, expected 5.0");
    }

    #[test]
    fn test_integral_gain() {
        let mut pid = PID::new(0.0, 0.5, 0.0);
        pid.set_sp(10.0);
        pid.set_pv(0.0);
        let dt = 1.0;
        let out = pid.step(Some(dt));
        assert!(out == 5.0, "output is {out}, expected 5.0");
        let out = pid.step(Some(dt));
        assert!(out == 10.0, "output is {out}, expected 10.0");
        pid.set_pv(20.0);
        let out = pid.step(Some(dt));
        assert!(out == 5.0, "output is {out}, expected 5.0");
        let out = pid.step(Some(dt));
        assert!(out == 0.0, "output is {out}, expected 0.0");
        let out = pid.step(Some(dt));
        assert!(out == -5.0, "output is {out}, expected -5.0");
    }

    #[test]
    fn test_derivative_gain() {
        let mut pid = PID::new(0.0, 0.0, 0.5);
        pid.enable_continuous_input(-20.0, 20.0);
        let error = 0.025;
        let dt = 1.0;
        // Test normal operation
        pid.set_sp(10.0);
        pid.set_pv(0.0);
        // First step, derivative is ignored
        let out = pid.step(None);
        assert!(out == 0.0, "output is {out}, expected 0.0");
        pid.set_pv(5.0);
        let out = pid.step(Some(dt));
        assert!(
            (out >= (-2.5 - error)) && (out <= (-2.5 + error)),
            "output is {out}, expected -2.5 +/- {error}"
        );
        // Test derivative kick
        pid.set_pv(10.0);
        pid.set_sp(15.0);
        let out2 = pid.step(Some(dt));
        assert!(
            (out2 >= (-2.5 - error)) && (out2 <= (-2.5 + error)),
            "output is {out2}, expected -2.5 +/- {error}"
        );
        // Test wrap
        pid.set_pv(20.0);
        pid.set_sp(-15.0);
        let out3 = pid.step(Some(dt));
        assert!(
            (out3 >= (-5.0 - error)) && (out3 <= (-5.0 + error)),
            "output is {out3}, expected -5.0 +/- {error}"
        );
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
    }

