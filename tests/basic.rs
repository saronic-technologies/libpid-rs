use libpid::PID;

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_proportional_gain() {
        let mut pid = PID::new(0.5, 0.0, 0.0);
        pid.set_sp(10.0);
        pid.set_pv(0.0);
        let timer = std::time::Instant::now();
        let mut dt = timer.elapsed().as_secs_f64();
        let out = pid.step(Some(dt));
        assert!(out == 5.0, "output is {out}, expected 5.0");
        dt = timer.elapsed().as_secs_f64();
        let out = pid.step(Some(dt));
        assert!(out == 5.0, "output is {out}, expected 5.0");
    }

    #[test]
    fn test_integral_gain() {
        let mut pid = PID::new(0.0, 0.5, 0.0);
        pid.set_sp(10.0);
        pid.set_pv(0.0);
        let timer = std::time::Instant::now();
        let mut dt = timer.elapsed().as_secs_f64();
        let out = pid.step(Some(dt));
        assert!(out == 5.0, "output is {out}, expected 5.0");
        dt = timer.elapsed().as_secs_f64();
        let out = pid.step(Some(dt));
        assert!(out == 10.0, "output is {out}, expected 10.0");
        pid.set_pv(20.0);
        dt = timer.elapsed().as_secs_f64();
        let out = pid.step(Some(dt));
        assert!(out == 5.0, "output is {out}, expected 5.0");
        dt = timer.elapsed().as_secs_f64();
        let out = pid.step(Some(dt));
        assert!(out == 0.0, "output is {out}, expected 0.0");
        dt = timer.elapsed().as_secs_f64();
        let out = pid.step(Some(dt));
        assert!(out == -5.0, "output is {out}, expected -5.0");
    }

    #[test]
    fn test_derivative_gain() {
        let mut pid = PID::new(0.0, 0.0, 0.5);
        pid.set_sp(10.0);
        pid.set_pv(0.0);
        let timer = std::time::Instant::now();
        // First step, derivative is ignored
        let out = pid.step(None);
        assert!(out == 0.0, "output is {out}, expected 0.0");
        pid.set_pv(5.0);
        std::thread::sleep(std::time::Duration::from_secs(1));
        let dt = timer.elapsed().as_secs_f64();
        let out = pid.step(Some(dt));
        let error = 0.001;
        assert!(
            (out >= (-2.5 - error)) && (out <= (-2.5 + error)),
            "output is {out}, expected -2.5 +/- {error}"
        );
    }
}
