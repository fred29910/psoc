use psoc::{AppConfig, Application, LogConfig, LogFormat, LogLevel};
use std::process;

fn main() {
    // Handle application startup and errors
    if let Err(e) = run_application() {
        eprintln!("Application error: {}", e);
        eprintln!("Error category: {}", e.category());
        eprintln!("Recoverable: {}", e.is_recoverable());

        // Exit with appropriate code
        let exit_code = if e.is_recoverable() { 1 } else { 2 };
        process::exit(exit_code);
    }
}

fn run_application() -> psoc::Result<()> {
    // Create application configuration
    let log_config = LogConfig::from_env()
        .with_level(LogLevel::Info)
        .with_format(LogFormat::Pretty);

    let app_config = AppConfig {
        name: "PSOC Image Editor".to_string(),
        version: psoc::VERSION.to_string(),
        debug_mode: cfg!(debug_assertions),
        log_config,
    };

    // Create and run the application
    let app = Application::with_config(app_config)?;
    app.run()?;

    Ok(())
}
