//! Debug test for embedded resources

use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "resources/i18n/"]
struct TestTranslationAssets;

#[test]
fn test_embedded_resources() {
    println!("Testing embedded resources...");
    
    // List all embedded files
    println!("Available embedded files:");
    for file_path in TestTranslationAssets::iter() {
        println!("  - {}", file_path);
    }
    
    // Try to get specific files
    let files_to_test = ["en.ftl", "zh-cn.ftl"];
    
    for filename in &files_to_test {
        println!("\nTesting file: {}", filename);
        if let Some(file) = TestTranslationAssets::get(filename) {
            let content = std::str::from_utf8(&file.data).unwrap();
            println!("  Found! Size: {} bytes", content.len());
            println!("  First 100 chars: {}", &content[..content.len().min(100)]);
        } else {
            println!("  NOT FOUND!");
        }
    }
    
    // Ensure we have at least the basic files
    assert!(TestTranslationAssets::get("en.ftl").is_some(), "en.ftl should be embedded");
    assert!(TestTranslationAssets::get("zh-cn.ftl").is_some(), "zh-cn.ftl should be embedded");
}
