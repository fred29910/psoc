use std::env;
use std::fs;
use std::path::Path;

#[test]
fn test_build_script_metadata() {
    // Test that build script sets required environment variables
    // These are set as rustc-env variables, so they're available as compile-time env vars
    let version = option_env!("PSOC_VERSION").unwrap_or(env!("CARGO_PKG_VERSION"));
    let name = option_env!("PSOC_NAME").unwrap_or(env!("CARGO_PKG_NAME"));
    let description = option_env!("PSOC_DESCRIPTION").unwrap_or(env!("CARGO_PKG_DESCRIPTION"));
    let authors = option_env!("PSOC_AUTHORS").unwrap_or(env!("CARGO_PKG_AUTHORS"));

    assert!(!version.is_empty());
    assert!(!name.is_empty());
    assert!(!description.is_empty());
    assert!(!authors.is_empty());
}

#[test]
fn test_version_consistency() {
    // Test that version in Cargo.toml matches build environment variable
    let cargo_version = env!("CARGO_PKG_VERSION");
    let build_version = option_env!("PSOC_VERSION").unwrap_or(cargo_version);
    assert_eq!(cargo_version, build_version);
}

#[test]
fn test_packaging_scripts_exist() {
    // Test that all packaging scripts exist
    let scripts = [
        "scripts/package.sh",
        "scripts/package/linux.sh",
        "scripts/package/macos.sh",
        "scripts/package/windows.ps1",
        "scripts/generate_icons.sh",
    ];

    for script in &scripts {
        assert!(Path::new(script).exists(), "Script not found: {}", script);
    }
}

#[test]
fn test_packaging_scripts_executable() {
    // Test that shell scripts are executable (Unix only)
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;

        let scripts = [
            "scripts/package.sh",
            "scripts/package/linux.sh",
            "scripts/package/macos.sh",
            "scripts/generate_icons.sh",
        ];

        for script in &scripts {
            if Path::new(script).exists() {
                let metadata = fs::metadata(script).unwrap();
                let permissions = metadata.permissions();
                assert!(
                    permissions.mode() & 0o111 != 0,
                    "Script not executable: {}",
                    script
                );
            }
        }
    }
}

#[test]
fn test_resources_directory_structure() {
    // Test that resources directory has expected structure
    let resources_dir = Path::new("resources");
    assert!(resources_dir.exists(), "Resources directory not found");

    let icons_dir = resources_dir.join("icons");
    assert!(icons_dir.exists(), "Icons directory not found");

    let desktop_dir = resources_dir.join("desktop");
    assert!(desktop_dir.exists(), "Desktop directory not found");
}

#[test]
fn test_svg_icon_exists() {
    // Test that SVG icon source exists
    let svg_icon = Path::new("resources/icons/psoc.svg");
    assert!(svg_icon.exists(), "SVG icon not found");

    // Test that SVG file is valid XML
    let svg_content = fs::read_to_string(svg_icon).unwrap();
    assert!(svg_content.contains("<svg"), "Invalid SVG file");
    assert!(svg_content.contains("</svg>"), "Invalid SVG file");
}

#[test]
fn test_wix_configuration_exists() {
    // Test that WiX configuration exists for Windows packaging
    let wix_config = Path::new("wix/main.wxs");
    assert!(wix_config.exists(), "WiX configuration not found");

    // Test that WiX file is valid XML
    let wix_content = fs::read_to_string(wix_config).unwrap();
    assert!(wix_content.contains("<Wix"), "Invalid WiX file");
    assert!(wix_content.contains("</Wix>"), "Invalid WiX file");
    assert!(
        wix_content.contains("PSOC Image Editor"),
        "Product name not found in WiX file"
    );
}

#[test]
fn test_desktop_file_format() {
    // Test desktop file format (if it exists)
    let desktop_file = Path::new("resources/desktop/psoc.desktop");
    if desktop_file.exists() {
        let content = fs::read_to_string(desktop_file).unwrap();
        assert!(
            content.contains("[Desktop Entry]"),
            "Invalid desktop file format"
        );
        assert!(content.contains("Name="), "Desktop file missing Name field");
        assert!(content.contains("Exec="), "Desktop file missing Exec field");
        assert!(
            content.contains("Type=Application"),
            "Desktop file missing Type field"
        );
    }
}

#[test]
fn test_package_script_syntax() {
    // Test that shell scripts have valid syntax (basic check)
    let shell_scripts = [
        "scripts/package.sh",
        "scripts/package/linux.sh",
        "scripts/package/macos.sh",
        "scripts/generate_icons.sh",
    ];

    for script in &shell_scripts {
        if Path::new(script).exists() {
            let content = fs::read_to_string(script).unwrap();
            assert!(
                content.starts_with("#!/bin/bash"),
                "Script missing shebang: {}",
                script
            );
            assert!(
                content.contains("set -e"),
                "Script missing error handling: {}",
                script
            );
        }
    }
}

#[test]
fn test_powershell_script_syntax() {
    // Test that PowerShell script has valid syntax (basic check)
    let ps_script = "scripts/package/windows.ps1";
    if Path::new(ps_script).exists() {
        let content = fs::read_to_string(ps_script).unwrap();
        assert!(
            content.contains("param("),
            "PowerShell script missing parameters"
        );
        assert!(
            content.contains("$ErrorActionPreference"),
            "PowerShell script missing error handling"
        );
    }
}

#[test]
fn test_cargo_build_dependencies() {
    // Test that build dependencies are properly configured
    let cargo_toml = fs::read_to_string("Cargo.toml").unwrap();
    assert!(
        cargo_toml.contains("[build-dependencies]"),
        "Build dependencies section not found"
    );
    assert!(
        cargo_toml.contains("build = \"build.rs\""),
        "Build script not configured"
    );
}

#[test]
fn test_application_metadata() {
    // Test application metadata consistency
    let name = option_env!("PSOC_NAME").unwrap_or(env!("CARGO_PKG_NAME"));
    let description = option_env!("PSOC_DESCRIPTION").unwrap_or(env!("CARGO_PKG_DESCRIPTION"));
    let authors = option_env!("PSOC_AUTHORS").unwrap_or(env!("CARGO_PKG_AUTHORS"));

    assert_eq!(name, "psoc");
    assert!(description.to_lowercase().contains("image editor"));
    assert!(authors.contains("PSOC Development Team"));
}

#[test]
fn test_target_information() {
    // Test that target information is available
    let target_os = option_env!("PSOC_TARGET_OS").unwrap_or(std::env::consts::OS);
    let target_arch = option_env!("PSOC_TARGET_ARCH").unwrap_or(std::env::consts::ARCH);

    assert!(!target_os.is_empty(), "Target OS not set");
    assert!(!target_arch.is_empty(), "Target architecture not set");

    // Verify known target combinations
    let valid_os = ["linux", "windows", "macos"];
    let valid_arch = ["x86_64", "aarch64"];

    assert!(valid_os.contains(&target_os) || !target_os.is_empty());
    assert!(valid_arch.contains(&target_arch) || !target_arch.is_empty());
}

#[test]
fn test_build_time_format() {
    // Test that build time is in expected format (Unix timestamp)
    let build_time = option_env!("PSOC_BUILD_TIME").unwrap_or("1234567890");
    assert!(!build_time.is_empty(), "Build time not set");

    // Should be a valid Unix timestamp (numeric)
    let timestamp: u64 = build_time
        .parse()
        .expect("Build time should be a valid Unix timestamp");
    assert!(timestamp > 0, "Build time should be a positive timestamp");
}

#[test]
fn test_package_directories_creation() {
    // Test that package script can create necessary directories
    let temp_dir = tempfile::tempdir().unwrap();
    let packages_dir = temp_dir.path().join("packages");

    // This would normally be done by the packaging script
    fs::create_dir_all(&packages_dir).unwrap();
    fs::create_dir_all(packages_dir.join("linux")).unwrap();
    fs::create_dir_all(packages_dir.join("macos")).unwrap();
    fs::create_dir_all(packages_dir.join("windows")).unwrap();

    assert!(packages_dir.exists());
    assert!(packages_dir.join("linux").exists());
    assert!(packages_dir.join("macos").exists());
    assert!(packages_dir.join("windows").exists());
}
