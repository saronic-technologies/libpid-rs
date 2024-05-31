#[cfg(test)]
mod tests {
    use libpid::PID;
    use serial_test::serial;
    use test_log::test;

    #[test]
    #[serial]
    fn test_logging() {
        let timer = std::time::Instant::now();
        let mut pid = PID::new(1.0, 0.1, 0.1);
        pid.set_sp(10.0);
        pid.set_pv(0.0);
        let mut dt = timer.elapsed().as_secs_f64();
        pid.step(Some(dt));
        dt = timer.elapsed().as_secs_f64();
        pid.step(Some(dt));
        dt = timer.elapsed().as_secs_f64();
        pid.step(Some(dt));
        dt = timer.elapsed().as_secs_f64();
        pid.step(Some(dt));
        dt = timer.elapsed().as_secs_f64();
        pid.step(Some(dt));
    }

    #[test]
    #[serial]
    fn test_labeled_logging() {
        let timer = std::time::Instant::now();
        let mut pid = PID::new(1.0, 0.1, 0.1);
        pid.add_debug_label("TEST");
        pid.set_sp(20.0);
        pid.set_pv(0.0);
        let mut dt = timer.elapsed().as_secs_f64();
        pid.step(Some(dt));
        dt = timer.elapsed().as_secs_f64();
        pid.step(Some(dt));
        pid.set_pv(5.0);
        dt = timer.elapsed().as_secs_f64();
        pid.step(Some(dt));
        pid.set_pv(10.0);
        dt = timer.elapsed().as_secs_f64();
        pid.step(Some(dt));
        dt = timer.elapsed().as_secs_f64();
        pid.step(Some(dt));
    }
}
