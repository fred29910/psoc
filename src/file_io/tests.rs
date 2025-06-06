//! Tests for file I/O functionality

#[cfg(test)]
#[allow(clippy::module_inception)]
mod tests {
    use crate::file_io::{
        can_export, estimate_export_size, export_image_with_options, get_image_metadata,
        get_recommended_export_options, is_supported_import_extension, ExportOptions, FileManager,
    };
    use image::{ImageBuffer, Rgb};
    use psoc_core::{Document, Layer, RgbaPixel};
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_file_manager_import_image() -> anyhow::Result<()> {
        let temp_dir = tempdir()?;
        let file_path = temp_dir.path().join("test.png");

        // Create and save a test image
        let img: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(100, 100);
        let dynamic_img = image::DynamicImage::ImageRgb8(img);
        dynamic_img.save(&file_path)?;

        // Test import
        let file_manager = FileManager::new();
        let imported = file_manager.import_image(&file_path).await?;

        assert_eq!(imported.width(), 100);
        assert_eq!(imported.height(), 100);

        Ok(())
    }

    #[tokio::test]
    async fn test_file_manager_export_image() -> anyhow::Result<()> {
        let temp_dir = tempdir()?;
        let file_path = temp_dir.path().join("test_export.png");

        // Create a test image
        let img: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(50, 50);
        let dynamic_img = image::DynamicImage::ImageRgb8(img);

        // Test export
        let file_manager = FileManager::new();
        file_manager.export_image(&dynamic_img, &file_path).await?;

        // Verify file exists
        assert!(file_path.exists());

        // Verify we can load it back
        let loaded = file_manager.import_image(&file_path).await?;
        assert_eq!(loaded.width(), 50);
        assert_eq!(loaded.height(), 50);

        Ok(())
    }

    #[test]
    fn test_file_manager_supported_extensions() {
        let import_extensions = FileManager::supported_import_extensions();
        assert!(import_extensions.contains(&"png"));
        assert!(import_extensions.contains(&"jpg"));
        assert!(import_extensions.contains(&"jpeg"));
        assert!(import_extensions.contains(&"psoc"));

        let export_extensions = FileManager::supported_export_extensions();
        assert!(export_extensions.contains(&"png"));
        assert!(export_extensions.contains(&"jpg"));
        assert!(export_extensions.contains(&"jpeg"));

        let project_extensions = FileManager::supported_project_extensions();
        assert!(project_extensions.contains(&"psoc"));
    }

    #[test]
    fn test_file_manager_file_filters() {
        let import_filter = FileManager::import_file_filter();
        assert!(import_filter.contains("png"));
        assert!(import_filter.contains("jpg"));
        assert!(import_filter.contains("jpeg"));
        assert!(import_filter.contains("psoc"));

        let export_filter = FileManager::export_file_filter();
        assert!(export_filter.contains("png"));
        assert!(export_filter.contains("jpg"));
        assert!(export_filter.contains("jpeg"));

        let project_filter = FileManager::project_file_filter();
        assert!(project_filter.contains("psoc"));
    }

    #[test]
    fn test_is_supported_import_extension() {
        // Test with supported extensions
        assert!(is_supported_import_extension("test.png"));
        assert!(is_supported_import_extension("test.jpg"));
        assert!(is_supported_import_extension("test.jpeg"));
        assert!(is_supported_import_extension("test.PNG"));
        assert!(is_supported_import_extension("test.JPG"));

        // Test with unsupported extensions
        assert!(!is_supported_import_extension("test.txt"));
        assert!(!is_supported_import_extension("test.pdf"));
        assert!(!is_supported_import_extension("test"));
    }

    #[tokio::test]
    async fn test_get_image_metadata() -> anyhow::Result<()> {
        let temp_dir = tempdir()?;
        let file_path = temp_dir.path().join("test_metadata.png");

        // Create and save a test image
        let img: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(200, 150);
        let dynamic_img = image::DynamicImage::ImageRgb8(img);
        dynamic_img.save(&file_path)?;

        // Get metadata
        let metadata = get_image_metadata(&file_path).await?;

        assert_eq!(metadata.width, 200);
        assert_eq!(metadata.height, 150);
        assert!(metadata.file_size > 0);
        assert!(metadata.format.is_some());

        let description = metadata.description();
        assert!(description.contains("200"));
        assert!(description.contains("150"));
        assert!(description.contains("PNG"));

        Ok(())
    }

    #[test]
    fn test_can_export() {
        // Test with supported extensions
        assert!(can_export("test.png"));
        assert!(can_export("test.jpg"));
        assert!(can_export("test.jpeg"));
        assert!(can_export("test.PNG"));
        assert!(can_export("test.JPG"));

        // Test with unsupported extensions
        assert!(!can_export("test.txt"));
        assert!(!can_export("test.pdf"));
        assert!(!can_export("test"));
    }

    #[test]
    fn test_get_recommended_export_options() {
        let png_options = get_recommended_export_options("test.png");
        assert!(matches!(png_options, Some(ExportOptions::Png(_))));

        let jpeg_options = get_recommended_export_options("test.jpg");
        assert!(matches!(jpeg_options, Some(ExportOptions::Jpeg(_))));

        let unknown_options = get_recommended_export_options("test.txt");
        assert!(unknown_options.is_none());
    }

    #[test]
    fn test_estimate_export_size() {
        let img: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(100, 100);
        let dynamic_img = image::DynamicImage::ImageRgb8(img);

        let png_size = estimate_export_size(&dynamic_img, psoc_file_formats::SupportedFormat::Png);
        let jpeg_size =
            estimate_export_size(&dynamic_img, psoc_file_formats::SupportedFormat::Jpeg);

        assert!(png_size > 0);
        assert!(jpeg_size > 0);
        // JPEG should generally be smaller than PNG for photos
        assert!(jpeg_size < png_size);
    }

    #[tokio::test]
    async fn test_export_image_with_options() -> anyhow::Result<()> {
        let temp_dir = tempdir()?;
        let file_path = temp_dir.path().join("test_options.jpg");

        // Create a test image
        let img: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(50, 50);
        let dynamic_img = image::DynamicImage::ImageRgb8(img);

        let options = ExportOptions::Jpeg(psoc_file_formats::JpegOptions::high_quality());

        // Export the image with options
        export_image_with_options(&dynamic_img, &file_path, options).await?;

        // Verify file exists
        assert!(file_path.exists());

        Ok(())
    }

    #[tokio::test]
    async fn test_load_document_from_image() -> anyhow::Result<()> {
        let temp_dir = tempdir()?;
        let file_path = temp_dir.path().join("test.png");

        // Create and save a test image
        let img: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(100, 100);
        let dynamic_img = image::DynamicImage::ImageRgb8(img);
        dynamic_img.save(&file_path)?;

        // Test loading as document
        let file_manager = FileManager::new();
        let document = file_manager.load_document(&file_path).await?;

        assert_eq!(document.size.width, 100.0);
        assert_eq!(document.size.height, 100.0);
        assert_eq!(document.layers.len(), 1);
        assert_eq!(document.layers[0].name, "Background");

        Ok(())
    }

    #[tokio::test]
    async fn test_save_and_load_project() -> anyhow::Result<()> {
        let temp_dir = tempdir()?;
        let project_path = temp_dir.path().join("test_project.psoc");

        // Create a test document with multiple layers
        let mut document = Document::new("Test Project".to_string(), 200, 150);

        let mut layer1 = Layer::new_pixel("Background".to_string(), 200, 150);
        layer1.fill(RgbaPixel::new(255, 0, 0, 255)); // Red background

        let mut layer2 = Layer::new_pixel("Foreground".to_string(), 100, 75);
        layer2.fill(RgbaPixel::new(0, 255, 0, 128)); // Semi-transparent green
        layer2.opacity = 0.8;
        layer2.visible = true;

        document.add_layer(layer1);
        document.add_layer(layer2);

        // Save as project
        let file_manager = FileManager::new();
        file_manager.save_project(&document, &project_path).await?;

        // Verify file exists
        assert!(project_path.exists());

        // Load back and verify
        let loaded_document = file_manager.load_document(&project_path).await?;

        assert_eq!(loaded_document.metadata.title, "Test Project");
        assert_eq!(loaded_document.size.width, 200.0);
        assert_eq!(loaded_document.size.height, 150.0);
        assert_eq!(loaded_document.layers.len(), 2);
        assert_eq!(loaded_document.layers[0].name, "Background");
        assert_eq!(loaded_document.layers[1].name, "Foreground");
        assert_eq!(loaded_document.layers[1].opacity, 0.8);
        assert!(loaded_document.layers[1].visible);

        Ok(())
    }

    #[tokio::test]
    async fn test_export_flattened() -> anyhow::Result<()> {
        let temp_dir = tempdir()?;
        let export_path = temp_dir.path().join("flattened.png");

        // Create a test document with multiple layers
        let mut document = Document::new("Test Document".to_string(), 100, 100);

        let mut layer1 = Layer::new_pixel("Background".to_string(), 100, 100);
        layer1.fill(RgbaPixel::new(255, 0, 0, 255)); // Red background

        let mut layer2 = Layer::new_pixel("Overlay".to_string(), 50, 50);
        layer2.fill(RgbaPixel::new(0, 255, 0, 255)); // Green overlay
        layer2.opacity = 0.5;

        document.add_layer(layer1);
        document.add_layer(layer2);

        // Export as flattened image
        let file_manager = FileManager::new();
        file_manager
            .export_flattened(&document, &export_path)
            .await?;

        // Verify file exists
        assert!(export_path.exists());

        // Load back and verify dimensions
        let loaded_image = file_manager.import_image(&export_path).await?;
        assert_eq!(loaded_image.width(), 100);
        assert_eq!(loaded_image.height(), 100);

        Ok(())
    }

    #[tokio::test]
    async fn test_project_roundtrip_with_complex_layers() -> anyhow::Result<()> {
        let temp_dir = tempdir()?;
        let project_path = temp_dir.path().join("complex_project.psoc");

        // Create a complex document
        let mut document = Document::new("Complex Project".to_string(), 300, 200);
        document.metadata.description = Some("A test project with multiple layers".to_string());

        // Add multiple layers with different properties
        for i in 0..3 {
            let mut layer = Layer::new_pixel(format!("Layer {}", i + 1), 100, 100);
            layer.opacity = 0.3 * (i + 1) as f32;
            layer.visible = i % 2 == 0; // Alternate visibility
            layer.fill(RgbaPixel::new(
                (i * 80) as u8,
                ((2 - i) * 80) as u8,
                128,
                255,
            ));
            document.add_layer(layer);
        }

        // Save and load
        let file_manager = FileManager::new();
        file_manager.save_project(&document, &project_path).await?;
        let loaded_document = file_manager.load_document(&project_path).await?;

        // Verify all properties are preserved
        assert_eq!(loaded_document.layers.len(), 3);
        assert_eq!(
            loaded_document.metadata.description,
            document.metadata.description
        );

        for (i, layer) in loaded_document.layers.iter().enumerate() {
            assert_eq!(layer.name, format!("Layer {}", i + 1));
            assert_eq!(layer.opacity, 0.3 * (i + 1) as f32);
            assert_eq!(layer.visible, i % 2 == 0);
        }

        Ok(())
    }
}
