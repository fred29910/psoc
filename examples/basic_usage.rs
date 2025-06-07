use psoc::Application;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("PSOC Basic Usage Example");

    // Initialize logging
    tracing_subscriber::fmt::init();

    // Create application
    let _app = Application::new()?;
    println!("Application created successfully");

    // In a real example, we would run the app
    // app.run()?;

    println!("Example completed");
    Ok(())
}
