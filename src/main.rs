use psoc::Application;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    // Create and run the application
    let app = Application::new()?;
    app.run()?;

    Ok(())
}
