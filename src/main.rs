mod app_config;
use app_config::AppConfig;
use env_logger::Env;

fn main() {
    //launch the app and override a TOML value from an env variable (need to use 2x_ for the section prefix)
    //i.e. RUST_LOG=debug DEBUG=1 APP_APPLICATION__HOST=wibble APP_APPLICATION__SOME_VALUE="hello back!" cargo run
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let app_config = AppConfig::new().expect("configuration error");
    // Print out test settings
    log::info!("app_config={:?}", app_config);
    log::info!(
        "app_config.application.host={}",
        app_config.application.host
    );

    to_test(true);
}

fn to_test(output: bool) -> bool {
    output
}

#[cfg(test)] // The module is only compiled when testing.
mod test {
    use super::to_test;

    // This function is a test function. It will be executed and the test will succeed if the function exits cleanly.
    #[test]
    fn test_to_test_ok() {
        assert!(to_test(true));
    }

    // That test on the other hand will only succeed when the function panics.
    #[test]
    #[should_panic]
    fn test_to_test_fail() {
        assert!(!to_test(true));
    }
}
