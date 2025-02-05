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
}
