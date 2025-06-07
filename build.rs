use std::env;

fn main() {
    // Only run on Windows
    if env::var("CARGO_CFG_TARGET_OS").unwrap_or_default() == "windows" {
        // Set up Windows resources
        setup_windows_resources();
    }

    // Set up application metadata for all platforms
    setup_app_metadata();

    println!("cargo:rerun-if-changed=resources/");
    println!("cargo:rerun-if-changed=build.rs");
}

fn setup_windows_resources() {
    #[cfg(all(target_os = "windows", feature = "windows-resources"))]
    {
        use std::path::Path;
        use winres::WindowsResource;

        let mut res = WindowsResource::new();

        // Set application icon
        if Path::new("resources/icons/psoc.ico").exists() {
            res.set_icon("resources/icons/psoc.ico");
        }

        // Set version information
        res.set("ProductName", "PSOC Image Editor");
        res.set(
            "FileDescription",
            "Professional Simple Open-source image editor",
        );
        res.set("CompanyName", "PSOC Development Team");
        res.set("LegalCopyright", "Copyright © 2024 PSOC Development Team");
        res.set("ProductVersion", env!("CARGO_PKG_VERSION"));
        res.set("FileVersion", env!("CARGO_PKG_VERSION"));

        // Compile resources
        if let Err(e) = res.compile() {
            eprintln!("Warning: Failed to compile Windows resources: {}", e);
        }
    }

    #[cfg(not(all(target_os = "windows", feature = "windows-resources")))]
    {
        // No-op for non-Windows platforms or when feature is disabled
        println!("cargo:warning=Windows resources feature not enabled");
    }
}

fn setup_app_metadata() {
    // Set environment variables for the application
    println!("cargo:rustc-env=PSOC_VERSION={}", env!("CARGO_PKG_VERSION"));
    println!("cargo:rustc-env=PSOC_NAME={}", env!("CARGO_PKG_NAME"));
    println!(
        "cargo:rustc-env=PSOC_DESCRIPTION={}",
        env!("CARGO_PKG_DESCRIPTION")
    );
    println!("cargo:rustc-env=PSOC_AUTHORS={}", env!("CARGO_PKG_AUTHORS"));

    // Set build timestamp
    let build_time = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    println!("cargo:rustc-env=PSOC_BUILD_TIME={}", build_time);

    // Set target information
    println!(
        "cargo:rustc-env=PSOC_TARGET_OS={}",
        env::var("CARGO_CFG_TARGET_OS").unwrap_or_default()
    );
    println!(
        "cargo:rustc-env=PSOC_TARGET_ARCH={}",
        env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default()
    );
}
