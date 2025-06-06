//! Tests for file I/O functionality

#[cfg(test)]
mod tests {
    use super::*;
    use crate::file_io::{
        FileManager, can_import, is_supported_import_extension, can_export, get_image_metadata,
        get_recommended_export_options, estimate_export_size,
        export_image_with_options, ExportOptions
    };
    use image::{ImageBuffer, Rgb};
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

        let export_extensions = FileManager::supported_export_extensions();
        assert!(export_extensions.contains(&"png"));
        assert!(export_extensions.contains(&"jpg"));
        assert!(export_extensions.contains(&"jpeg"));
    }

    #[test]
    fn test_file_manager_file_filters() {
        let import_filter = FileManager::import_file_filter();
        assert!(import_filter.contains("png"));
        assert!(import_filter.contains("jpg"));
        assert!(import_filter.contains("jpeg"));

        let export_filter = FileManager::export_file_filter();
        assert!(export_filter.contains("png"));
        assert!(export_filter.contains("jpg"));
        assert!(export_filter.contains("jpeg"));
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
        let jpeg_size = estimate_export_size(&dynamic_img, psoc_file_formats::SupportedFormat::Jpeg);

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
}
