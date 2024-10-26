use std::cmp::min;

use image::{GenericImageView, ImageBuffer, Rgba, RgbaImage};
use rand::Rng;

pub fn bytes_to_rgba_image(bytes: &[u8], width: u32, height: u32) -> RgbaImage {
    ImageBuffer::from_raw(width, height, bytes.to_vec())
        .expect("Failed to create RgbaImage from raw bytes")
}

pub fn grayscale_filter(img_data: &[u8], width: u32, height: u32) -> Vec<u8> {
    let img = RgbaImage::from_raw(width, height, img_data.to_vec()).unwrap();

    // RGBAフォーマットに準じたベクタを準備
    let mut rgba_data: Vec<u8> = vec![0; (width * height * 4) as usize];

    for (i, pixel) in img.pixels().enumerate() {
        // グレースケール値を計算
        let gray_value =
            (0.299 * pixel[0] as f32 + 0.587 * pixel[1] as f32 + 0.114 * pixel[2] as f32) as u8;

        // RGBAデータのフォーマットに合わせる（R=G=B=gray_value, A=255）
        rgba_data[i * 4] = gray_value; // R
        rgba_data[i * 4 + 1] = gray_value; // G
        rgba_data[i * 4 + 2] = gray_value; // B
        rgba_data[i * 4 + 3] = 255; // A
    }

    rgba_data
}

pub fn original_pixcel_filter(
    img: RgbaImage,
    complex_coefficient: f32,
    mut depth: u32,
    max_depth: u32,
) -> RgbaImage {
    depth = depth + 1;
    let (width, height) = img.dimensions();
    let is_comlex_image = is_complex(&img, complex_coefficient);
    let is_minimum = min(width, height) < 2;
    if !is_comlex_image || depth >= max_depth || is_minimum {
        return average_color_image(&img);
    }

    let images_list = split_image(img, width, height);
    let mut filterd_image_list: [RgbaImage; 4] = Default::default();
    for (i, splited_img) in images_list.iter().enumerate() {
        filterd_image_list[i] =
            original_pixcel_filter(splited_img.clone(), complex_coefficient, depth, max_depth);
    }
    let merged_img = merge_images(filterd_image_list, width, height);

    return merged_img;
}

/// フィルム風エフェクトを画像に適用する関数
pub fn film_effect(mut img: RgbaImage) -> RgbaImage {
    // コントラストを下げる
    for pixel in img.pixels_mut() {
        let Rgba(data) = *pixel;
        let contrasted = [
            (data[0] as f32 * 0.95) as u8, // 赤
            (data[1] as f32 * 0.95) as u8, // 緑
            (data[2] as f32 * 0.95) as u8, // 青
            data[3],                       // アルファ
        ];
        *pixel = Rgba(contrasted);
    }

    // 画像中央を若干明るくする（ビネット効果）
    let (width, height) = img.dimensions();
    let center_x = width / 2;
    let center_y = height / 2;
    let radius = width.min(height) / 2;

    for y in 0..height {
        for x in 0..width {
            let dx = (x as i32 - center_x as i32).abs();
            let dy = (y as i32 - center_y as i32).abs();
            let distance = ((dx.pow(2) + dy.pow(2)) as f32).sqrt();
            if distance < radius as f32 {
                let factor = 1.0 - (distance / radius as f32);
                let pixel = img.get_pixel_mut(x, y);
                let Rgba(data) = *pixel;
                let brightened = [
                    (data[0] as f32 + 30.0 * factor).min(255.0) as u8,
                    (data[1] as f32 + 30.0 * factor).min(255.0) as u8,
                    (data[2] as f32 + 30.0 * factor).min(255.0) as u8,
                    data[3],
                ];
                *pixel = Rgba(brightened);
            }
        }
    }

    // 色補正を若干暖色よりに（青の量を減らす）
    for pixel in img.pixels_mut() {
        let Rgba(data) = *pixel;
        let warmed = [
            data[0],                       // 赤はそのまま
            data[1],                       // 緑もそのまま
            (data[2] as f32 * 0.98) as u8, // 青の量を減らす
            data[3],
        ];
        *pixel = Rgba(warmed);
    }

    // 色被り補正を若干緑よりに
    for pixel in img.pixels_mut() {
        let Rgba(data) = *pixel;
        let green_tinted = [
            data[0],
            (data[1] as f32 * 1.03).min(255.0) as u8, // 緑を強める
            data[2],
            data[3],
        ];
        *pixel = Rgba(green_tinted);
    }

    // 画像の粒度を若干荒く（ノイズ追加）
    let mut rng = rand::thread_rng();
    for pixel in img.pixels_mut() {
        let noise: i32 = rng.gen_range(-3..3); // -10から10のノイズを追加
        let Rgba(data) = *pixel;
        let noisy = [
            (data[0] as i32 + noise).clamp(0, 255) as u8,
            (data[1] as i32 + noise).clamp(0, 255) as u8,
            (data[2] as i32 + noise).clamp(0, 255) as u8,
            data[3],
        ];
        *pixel = Rgba(noisy);
    }

    img
}

fn split_image(img: RgbaImage, width: u32, height: u32) -> [RgbaImage; 4] {
    let half_width = (width + 1) / 2; // 奇数の場合の対策
    let half_height = (height + 1) / 2; // 奇数の場合の対策

    // 左上
    let top_left = img
        .view(0, 0, half_width.min(width), half_height.min(height))
        .to_image();
    // 右上
    let top_right = img
        .view(
            half_width.min(width),
            0,
            (width - half_width).min(half_width),
            half_height.min(height),
        )
        .to_image();
    // 左下
    let bottom_left = img
        .view(
            0,
            half_height.min(height),
            half_width.min(width),
            (height - half_height).min(half_height),
        )
        .to_image();
    // 右下
    let bottom_right = img
        .view(
            half_width.min(width),
            half_height.min(height),
            (width - half_width).min(half_width),
            (height - half_height).min(half_height),
        )
        .to_image();

    [top_left, top_right, bottom_left, bottom_right]
}

fn merge_images(images: [RgbaImage; 4], width: u32, height: u32) -> RgbaImage {
    let half_width = (width + 1) / 2; // 奇数の場合の対策
    let half_height = (height + 1) / 2; // 奇数の場合の対策

    let mut merged_image = ImageBuffer::new(width, height);

    // 左上の画像をコピー
    for (x, y, pixel) in images[0].enumerate_pixels() {
        merged_image.put_pixel(x, y, *pixel);
    }

    // 右上の画像をコピー
    for (x, y, pixel) in images[1].enumerate_pixels() {
        if x + half_width < width {
            merged_image.put_pixel(x + half_width, y, *pixel);
        }
    }

    // 左下の画像をコピー
    for (x, y, pixel) in images[2].enumerate_pixels() {
        if y + half_height < height {
            merged_image.put_pixel(x, y + half_height, *pixel);
        }
    }

    // 右下の画像をコピー
    for (x, y, pixel) in images[3].enumerate_pixels() {
        if x + half_width < width && y + half_height < height {
            merged_image.put_pixel(x + half_width, y + half_height, *pixel);
        }
    }

    merged_image
}

fn is_complex(image: &RgbaImage, threshold: f32) -> bool {
    let (width, height) = image.dimensions();
    let mut brightness_values = Vec::with_capacity((width * height) as usize);

    // ピクセルの明度を計算
    for pixel in image.pixels() {
        let Rgba([r, g, b, _a]) = *pixel; // アルファ値は無視
                                          // 明度を計算 (相対明度の計算方法)
        let brightness = 0.2126 * r as f32 + 0.7152 * g as f32 + 0.0722 * b as f32;
        brightness_values.push(brightness);
    }

    // 明度の最大値と最小値を計算
    let max_brightness = brightness_values.iter().cloned().fold(0. / 0., f32::max); // -inf からの最大値
    let min_brightness = brightness_values
        .iter()
        .cloned()
        .fold(f32::INFINITY, f32::min); // +inf からの最小値

    // コントラストを評価 (明度の差がしきい値を超えた場合に高コントラストと判断)
    (max_brightness - min_brightness) > threshold
}

fn average_color_image(image: &RgbaImage) -> RgbaImage {
    let (width, height) = image.dimensions();
    let mut r_sum = 0u32;
    let mut g_sum = 0u32;
    let mut b_sum = 0u32;
    let mut count = 0u32;

    // 画像のすべてのピクセルの平均色を計算
    for pixel in image.pixels() {
        let Rgba([r, g, b, _a]) = *pixel;
        r_sum += r as u32;
        g_sum += g as u32;
        b_sum += b as u32;
        count += 1;
    }

    // 平均色を計算
    let r_avg = (r_sum / count) as u8;
    let g_avg = (g_sum / count) as u8;
    let b_avg = (b_sum / count) as u8;

    // 平均色で塗りつぶされた画像を作成
    let average_color = Rgba([r_avg, g_avg, b_avg, 255]);
    ImageBuffer::from_fn(width, height, |_, _| average_color)
}
