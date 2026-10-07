use arboard::Clipboard;
use image::{GrayImage, ImageBuffer, Rgb};
use minifb::{Key, Window, WindowOptions};
use nokhwa::{
    pixel_format::RgbFormat,
    utils::{CameraIndex, RequestedFormat, RequestedFormatType},
    Camera,
};
use rqrr::PreparedImage;

/// Parses a WiFi QR code string in the format:
/// WIFI:T:<security>;S:<ssid>;P:<password>;;
fn parse_wifi_qr(data: &str) -> Option<WifiInfo> {
    if !data.starts_with("WIFI:") {
        return None;
    }

    let mut ssid = String::new();
    let mut password = String::new();
    let mut security = String::from("Unknown");

    let content = data[5..].trim_end_matches(';');

    for field in content.split(';') {
        if let Some(val) = field.strip_prefix("S:") {
            ssid = val.to_string();
        } else if let Some(val) = field.strip_prefix("P:") {
            password = val.to_string();
        } else if let Some(val) = field.strip_prefix("T:") {
            security = val.to_string();
        }
    }

    if ssid.is_empty() {
        None
    } else {
        Some(WifiInfo {
            ssid,
            password,
            security,
        })
    }
}

struct WifiInfo {
    ssid: String,
    password: String,
    security: String,
}

fn main() {
    println!("╔══════════════════════════════════════╗");
    println!("║      📷  WiFi QR Code Scanner        ║");
    println!("╚══════════════════════════════════════╝");
    println!();
    println!("Opening camera... Hold a WiFi QR code up to the camera.");
    println!("Press ESC or close the window to quit.\n");

    // Open camera at index 0
    let index = CameraIndex::Index(0);
    let requested =
        RequestedFormat::new::<RgbFormat>(RequestedFormatType::AbsoluteHighestFrameRate);

    let mut camera = match Camera::new(index, requested) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("❌ Failed to open camera: {}", e);
            eprintln!("   Make sure your camera is connected and not in use by another app.");
            std::process::exit(1);
        }
    };

    if let Err(e) = camera.open_stream() {
        eprintln!("❌ Failed to start camera stream: {}", e);
        std::process::exit(1);
    }

    let fmt = camera.camera_format();
    let width = fmt.width() as usize;
    let height = fmt.height() as usize;

    // Create preview window
    let mut window = Window::new(
        "WiFi QR Scanner — ESC to quit",
        width,
        height,
        WindowOptions {
            resize: false,
            ..WindowOptions::default()
        },
    )
    .expect("Failed to create preview window");

    // ~30 fps cap
    window.limit_update_rate(Some(std::time::Duration::from_millis(33)));

    let mut last_non_wifi_msg: Option<String> = None;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // --- Capture frame ---
        let frame = match camera.frame() {
            Ok(f) => f,
            Err(_) => continue,
        };

        let rgb_data = match frame.decode_image::<RgbFormat>() {
            Ok(d) => d,
            Err(_) => continue,
        };

        let img_buffer: ImageBuffer<Rgb<u8>, Vec<u8>> =
            match ImageBuffer::from_raw(rgb_data.width(), rgb_data.height(), rgb_data.into_vec()) {
                Some(b) => b,
                None => continue,
            };

        // --- Build display buffer (RGB → 0x00RRGGBB) ---
        let display_buffer: Vec<u32> = img_buffer
            .pixels()
            .map(|p| ((p[0] as u32) << 16) | ((p[1] as u32) << 8) | (p[2] as u32))
            .collect();

        window
            .update_with_buffer(&display_buffer, width, height)
            .ok();

        // --- QR decode ---
        let gray: GrayImage = image::imageops::grayscale(&img_buffer);
        let mut prepared = PreparedImage::prepare(gray);
        let grids = prepared.detect_grids();

        for grid in grids {
            if let Ok((_, content)) = grid.decode() {
                if let Some(wifi) = parse_wifi_qr(&content) {
                    // ✅ Found a WiFi QR code
                    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
                    println!("✅ WiFi QR Code Detected!");
                    println!("   📶 Network   : {}", wifi.ssid);
                    println!("   🔐 Security  : {}", wifi.security);

                    if wifi.password.is_empty() {
                        println!("   🔓 Password  : (open network — no password)");
                    } else {
                        println!("   🔑 Password  : {}", wifi.password);

                        // Copy to clipboard
                        match Clipboard::new() {
                            Ok(mut clipboard) => {
                                match clipboard.set_text(wifi.password.clone()) {
                                    Ok(_) => println!("   📋 Copied to clipboard!"),
                                    Err(e) => eprintln!("   ⚠️  Clipboard error: {}", e),
                                }
                            }
                            Err(e) => eprintln!("   ⚠️  Could not open clipboard: {}", e),
                        }
                    }
                    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

                    // Keep window open so user can read output
                    println!("\nPress ESC or close the window to exit.");
                    while window.is_open() && !window.is_key_down(Key::Escape) {
                        window.update();
                        std::thread::sleep(std::time::Duration::from_millis(33));
                    }

                    camera.stop_stream().ok();
                    return;
                } else {
                    // QR found but not WiFi — print once per unique content
                    if last_non_wifi_msg.as_deref() != Some(&content) {
                        println!("⚠️  QR detected (not a WiFi QR): {}", content);
                        last_non_wifi_msg = Some(content.clone());
                    }
                }
            }
        }
    }

    camera.stop_stream().ok();
    println!("\nScanner closed.");
}
