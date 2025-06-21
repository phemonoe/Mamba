use screenshots::Screen;
use image::{ImageFormat, DynamicImage, ImageBuffer, Rgba};
use base64::{Engine as _, engine::general_purpose};
use std::io::Cursor;

pub async fn capture_screen() -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    log::info!("Starting screen capture...");
    
    // Get the primary screen
    let screens = Screen::all()?;
    let screen = screens.into_iter().next()
        .ok_or("No screens found")?;
    
    log::info!("Capturing from screen: {:?}", screen.display_info);
    
    // Capture the screenshot
    let image = screen.capture()?;
    
    let width = image.width();
    let height = image.height();
    let raw_buffer = image.buffer();
    
    log::info!("Image captured: {}x{}, {} bytes", width, height, raw_buffer.len());
    log::info!("Expected RGBA size would be: {} bytes", width * height * 4);
    log::info!("Bytes per pixel: {:.2}", raw_buffer.len() as f64 / (width * height) as f64);
    
    // The screenshots crate on macOS might return compressed data or different format
    // Let's try to convert it directly to a proper image format
    
    // Check if this looks like raw pixel data
    let expected_rgba_size = (width * height * 4) as usize;
    let expected_rgb_size = (width * height * 3) as usize;
    
    let dynamic_image = if raw_buffer.len() == expected_rgba_size {
        // Standard RGBA format
        log::info!("Detected RGBA format");
        let rgba_image = ImageBuffer::<Rgba<u8>, Vec<u8>>::from_raw(width, height, raw_buffer.to_vec())
            .ok_or("Failed to create RGBA image from buffer")?;
        DynamicImage::ImageRgba8(rgba_image)
        
    } else if raw_buffer.len() == expected_rgb_size {
        // RGB format - convert to RGBA
        log::info!("Detected RGB format, converting to RGBA");
        let mut rgba_buffer = Vec::with_capacity(expected_rgba_size);
        for chunk in raw_buffer.chunks_exact(3) {
            rgba_buffer.push(chunk[0]); // R
            rgba_buffer.push(chunk[1]); // G
            rgba_buffer.push(chunk[2]); // B
            rgba_buffer.push(255);      // A (fully opaque)
        }
        let rgba_image = ImageBuffer::<Rgba<u8>, Vec<u8>>::from_raw(width, height, rgba_buffer)
            .ok_or("Failed to create RGBA image from RGB buffer")?;
        DynamicImage::ImageRgba8(rgba_image)
        
    } else {
        // Try to decode as compressed image data (PNG, JPEG, etc.)
        log::info!("Attempting to decode as compressed image data");
        match image::load_from_memory(raw_buffer) {
            Ok(img) => {
                log::info!("Successfully decoded compressed image");
                img
            }
            Err(e) => {
                log::error!("Failed to decode as compressed image: {}", e);
                
                // As a last resort, try BGRA format (common on macOS)
                if raw_buffer.len() == expected_rgba_size {
                    log::info!("Trying BGRA format");
                    let mut rgba_buffer = Vec::with_capacity(expected_rgba_size);
                    for chunk in raw_buffer.chunks_exact(4) {
                        rgba_buffer.push(chunk[2]); // R (was B)
                        rgba_buffer.push(chunk[1]); // G
                        rgba_buffer.push(chunk[0]); // B (was R)
                        rgba_buffer.push(chunk[3]); // A
                    }
                    let rgba_image = ImageBuffer::<Rgba<u8>, Vec<u8>>::from_raw(width, height, rgba_buffer)
                        .ok_or("Failed to create RGBA image from BGRA buffer")?;
                    DynamicImage::ImageRgba8(rgba_image)
                } else {
                    return Err(format!(
                        "Unsupported image format: {} bytes for {}x{} image", 
                        raw_buffer.len(), width, height
                    ).into());
                }
            }
        }
    };
    
    // Resize image if it's too large (OpenAI has size limits)
    let resized_image = if dynamic_image.width() > 2000 || dynamic_image.height() > 2000 {
        dynamic_image.resize(2000, 2000, image::imageops::FilterType::Lanczos3)
    } else {
        dynamic_image
    };
    
    // Convert to RGB (JPEG doesn't support alpha channel)
    let rgb_image = resized_image.to_rgb8();
    let final_image = DynamicImage::ImageRgb8(rgb_image);
    
    // Convert to JPEG for better compression
    let mut buffer = Vec::new();
    {
        let mut cursor = Cursor::new(&mut buffer);
        final_image.write_to(&mut cursor, ImageFormat::Jpeg)?;
    }
    
    // Encode to base64
    let base64_string = general_purpose::STANDARD.encode(&buffer);
    
    log::info!("Screenshot captured and encoded, size: {} bytes", buffer.len());
    
    Ok(base64_string)
}

#[allow(dead_code)]
pub async fn capture_region(x: i32, y: i32, width: u32, height: u32) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    log::info!("Starting region capture at ({}, {}) with size {}x{}", x, y, width, height);
    
    // For now, just capture full screen and crop
    let base64_full = capture_screen().await?;
    
    // Decode the base64 image
    let image_data = general_purpose::STANDARD.decode(base64_full)?;
    let full_image = image::load_from_memory(&image_data)?;
    
    // Crop the image
    let cropped = full_image.crop_imm(x as u32, y as u32, width, height);
    
    // Convert to RGB and then to JPEG
    let rgb_image = cropped.to_rgb8();
    let final_image = DynamicImage::ImageRgb8(rgb_image);
    
    // Convert back to JPEG and base64
    let mut buffer = Vec::new();
    {
        let mut cursor = Cursor::new(&mut buffer);
        final_image.write_to(&mut cursor, ImageFormat::Jpeg)?;
    }
    
    let base64_string = general_purpose::STANDARD.encode(&buffer);
    
    log::info!("Region screenshot captured and encoded, size: {} bytes", buffer.len());
    
    Ok(base64_string)
} 