use psoc::{AppConfig, Application, LogConfig, LogFormat, LogLevel};
use std::process;

fn main() {
    #[cfg(feature = "gui")]
    {
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

    #[cfg(not(feature = "gui"))]
    {
        println!("PSOC Image Editor - Command Line Mode");
        println!("GUI features are disabled. To enable GUI, compile with --features gui");
        println!("File I/O functionality is available through the library API.");
    }
}

#[cfg(feature = "gui")]
fn run_application() -> psoc::Result<()> {
    // Create application configuration
    let log_config = LogConfig::from_env()
        .with_level(LogLevel::Info)
        .with_format(LogFormat::Pretty);

    let app_config = AppConfig {
        name: "PSOC Image Editor".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        debug_mode: cfg!(debug_assertions),
        log_config,
    };

    // Create and run the application
    let app = Application::with_config(app_config)?;
    app.run()?;

    Ok(())
}
