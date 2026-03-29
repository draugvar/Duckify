use std::fs;
use std::io::{BufWriter, Write};

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=assets/icon_source.png");

    fs::create_dir_all("assets").expect("failed to create assets dir");

    let (rgba, src_w, _src_h) = load_png("assets/icon_source.png");

    // 256x256 base icon used at runtime and on Linux
    let icon_256 = scale(&rgba, src_w, 256);
    save_png("assets/icon.png", &icon_256, 256, 256);

    generate_icns(&rgba, src_w);
    generate_ico(&rgba, src_w);

    // Windows: embed icon in the executable
    #[cfg(target_os = "windows")]
    {
        let mut res = winres::WindowsResource::new();
        res.set_icon("assets/icon.ico");
        res.compile().expect("failed to compile Windows resources");
    }
}

// ── macOS .icns ──────────────────────────────────────────────────────────────

fn generate_icns(src: &[u8], src_w: u32) {
    let iconset = "assets/icon.iconset";
    fs::create_dir_all(iconset).expect("failed to create iconset dir");

    let entries: &[(u32, &str)] = &[
        (16,   "icon_16x16.png"),
        (32,   "icon_16x16@2x.png"),
        (32,   "icon_32x32.png"),
        (64,   "icon_32x32@2x.png"),
        (128,  "icon_128x128.png"),
        (256,  "icon_128x128@2x.png"),
        (256,  "icon_256x256.png"),
        (512,  "icon_256x256@2x.png"),
        (512,  "icon_512x512.png"),
        (1024, "icon_512x512@2x.png"),
    ];

    for &(size, name) in entries {
        let scaled = scale(src, src_w, size);
        save_png(&format!("{iconset}/{name}"), &scaled, size, size);
    }

    // Run iconutil if building on macOS
    let host = std::env::var("HOST").unwrap_or_default();
    if host.contains("apple") {
        let status = std::process::Command::new("iconutil")
            .args(["-c", "icns", iconset, "-o", "assets/icon.icns"])
            .status();
        if let Ok(s) = status {
            if s.success() {
                println!("cargo:warning=icon.icns generated successfully");
            }
        }
    }
}

// ── Windows .ico ──────────────────────────────────────────────────────────────

fn generate_ico(src: &[u8], src_w: u32) {
    let sizes: &[u32] = &[16, 32, 48, 64, 128, 256];

    // Encode each size as PNG bytes (ICO can embed PNG since Vista)
    let entries: Vec<(u32, Vec<u8>)> = sizes
        .iter()
        .map(|&s| {
            let scaled = scale(src, src_w, s);
            let png = encode_png_to_vec(&scaled, s, s);
            (s, png)
        })
        .collect();

    let file = fs::File::create("assets/icon.ico").expect("failed to create icon.ico");
    let mut out = BufWriter::new(file);
    let count = entries.len() as u16;

    // ICONDIR header (6 bytes)
    out.write_all(&[0, 0]).unwrap(); // reserved
    out.write_all(&[1, 0]).unwrap(); // type = 1 (icon)
    out.write_all(&count.to_le_bytes()).unwrap();

    // ICONDIRENTRY (16 bytes each) — data starts after header + all entries
    let data_start = 6u32 + count as u32 * 16;
    let mut offset = data_start;
    for (size, png) in &entries {
        let w = if *size >= 256 { 0u8 } else { *size as u8 };
        out.write_all(&[w, w, 0, 0]).unwrap(); // width, height, color count, reserved
        out.write_all(&[1, 0]).unwrap();        // planes
        out.write_all(&[32, 0]).unwrap();       // bit depth
        out.write_all(&(png.len() as u32).to_le_bytes()).unwrap();
        out.write_all(&offset.to_le_bytes()).unwrap();
        offset += png.len() as u32;
    }

    // Image data
    for (_, png) in &entries {
        out.write_all(png).unwrap();
    }
}

// ── PNG helpers ───────────────────────────────────────────────────────────────

fn load_png(path: &str) -> (Vec<u8>, u32, u32) {
    let file = fs::File::open(path).unwrap_or_else(|_| panic!("cannot open {path}"));
    let decoder = png::Decoder::new(file);
    let mut reader = decoder.read_info().unwrap();
    let mut buf = vec![0u8; reader.output_buffer_size()];
    let info = reader.next_frame(&mut buf).unwrap();
    let bytes = buf[..info.buffer_size()].to_vec();

    // Ensure RGBA
    let rgba = match info.color_type {
        png::ColorType::Rgba => bytes,
        png::ColorType::Rgb  => bytes.chunks(3)
            .flat_map(|c| [c[0], c[1], c[2], 255u8])
            .collect(),
        _ => panic!("unsupported PNG color type"),
    };
    (rgba, info.width, info.height)
}

fn save_png(path: &str, rgba: &[u8], w: u32, h: u32) {
    let file = fs::File::create(path).unwrap_or_else(|_| panic!("cannot create {path}"));
    let mut enc = png::Encoder::new(file, w, h);
    enc.set_color(png::ColorType::Rgba);
    enc.set_depth(png::BitDepth::Eight);
    enc.write_header().unwrap().write_image_data(rgba).unwrap();
}

fn encode_png_to_vec(rgba: &[u8], w: u32, h: u32) -> Vec<u8> {
    let mut buf = Vec::new();
    let mut enc = png::Encoder::new(&mut buf, w, h);
    enc.set_color(png::ColorType::Rgba);
    enc.set_depth(png::BitDepth::Eight);
    enc.write_header().unwrap().write_image_data(rgba).unwrap();
    buf
}

// ── Scaling (box filter down, nearest-neighbor up) ────────────────────────────

fn scale(src: &[u8], src_s: u32, dst_s: u32) -> Vec<u8> {
    if src_s == dst_s {
        return src.to_vec();
    }
    let mut dst = vec![0u8; (dst_s * dst_s * 4) as usize];
    let ratio = src_s as f32 / dst_s as f32;

    for dy in 0..dst_s {
        for dx in 0..dst_s {
            if ratio > 1.0 {
                // Box filter downscale
                let x0 = (dx as f32 * ratio) as u32;
                let x1 = ((dx + 1) as f32 * ratio).ceil() as u32;
                let y0 = (dy as f32 * ratio) as u32;
                let y1 = ((dy + 1) as f32 * ratio).ceil() as u32;
                let (mut r, mut g, mut b, mut a, mut n) = (0u32, 0, 0, 0, 0);
                for sy in y0..y1.min(src_s) {
                    for sx in x0..x1.min(src_s) {
                        let i = ((sy * src_s + sx) * 4) as usize;
                        r += src[i] as u32;
                        g += src[i + 1] as u32;
                        b += src[i + 2] as u32;
                        a += src[i + 3] as u32;
                        n += 1;
                    }
                }
                if n > 0 {
                    let i = ((dy * dst_s + dx) * 4) as usize;
                    dst[i]     = (r / n) as u8;
                    dst[i + 1] = (g / n) as u8;
                    dst[i + 2] = (b / n) as u8;
                    dst[i + 3] = (a / n) as u8;
                }
            } else {
                // Nearest-neighbor upscale
                let sx = (dx as f32 * ratio) as u32;
                let sy = (dy as f32 * ratio) as u32;
                let si = ((sy * src_s + sx) * 4) as usize;
                let di = ((dy * dst_s + dx) * 4) as usize;
                dst[di..di + 4].copy_from_slice(&src[si..si + 4]);
            }
        }
    }
    dst
}
