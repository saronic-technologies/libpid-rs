#[cfg(test)]
mod tests {
    use libpid::PID;
    use serial_test::serial;
    use test_log::test;


    #[test]
    #[serial]
    fn test_logging() {
        let mut pid = PID::new(1.0, 0.1, 0.1);
        pid.enable_debug();
        pid.set_sp(10.0);
        pid.set_pv(0.0);
        pid.step();
        pid.step();
        pid.step();
        pid.step();
        pid.step();
    }

    #[test]
    #[serial]
    fn test_labeled_logging() {
        let mut pid = PID::new(1.0, 0.1, 0.1);
        pid.enable_debug();
        pid.add_debug_label("TEST");
        pid.set_sp(20.0);
        pid.set_pv(0.0);
        pid.step();
        pid.step();
        pid.set_pv(5.0);
        pid.step();
        pid.set_pv(10.0);
        pid.step();
        pid.step();
    }
}
