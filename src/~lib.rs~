use image::{imageops::FilterType, ImageFormat, ImageReader};
use regex::Regex;
use std::fs;
use std::io::Cursor;
use std::path::Path;
use urlencoding::decode;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures;

#[wasm_bindgen(js_name = generateSrcset)]
pub async fn generate_srcset(url_str: &str) -> Result<String, JsValue> {
    let decoded_url = decode(url_str).map_err(|_| JsValue::from("Failed to decode url string"))?;
    let path = Path::new(decoded_url.as_ref());
    let parent = path
        .parent()
        .ok_or_else(|| JsValue::from("No parent found."))?;
    let parent_str = parent
        .to_str()
        .ok_or_else(|| JsValue::from("Couldn't convert parent to string."))?;
    let regex = Regex::new(r"([^/]+?)$").map_err(|_| JsValue::from("Failed to compile regex."))?;
    let re_res = regex
        .find(parent_str)
        .ok_or_else(|| JsValue::from("Regex search failed."))?;
    //        .map_err(|_| JsValue::from("Regex search failed."))?;
    let re_match = re_res;
    //    .ok_or_else(|| JsValue::from("Regex match failed."))?;
    let s = std::str::from_utf8(re_match.as_str().as_bytes())
        .map_err(|_| JsValue::from("String conversion failed."))?;
    let mut dir =
        std::env::current_dir().map_err(|_| JsValue::from("Failed to create directory path."))?;
    dir.push("public/");
    dir.push(s);

    if !dir.exists() {
        fs::create_dir_all(dir.as_path())
            .map_err(|_| JsValue::from("Failed to create directory."))?;
    }

    let bytes = reqwest::get(url_str)
        .await
        .map_err(|_| JsValue::from("Failed to download image"))?
        .bytes()
        .await
        .map_err(|_| JsValue::from("Failed to convert image to bytes"))?;

    println!("bytes created");
    let reader = ImageReader::new(Cursor::new(bytes.slice(..)))
        .with_guessed_format()
        .map_err(|e| JsValue::from(format!("{:?}", e)))?;

    let image = reader
        .decode()
        .map_err(|e| JsValue::from(format!("error generating image: {:?}", e)))?;
    let [width, height] = [image.width(), image.height()];
    let screenwidths = [1536, 1280, 1024, 768, 640];
    let mut srcset: String = "\"".to_string();
    for w in screenwidths {
        let h: u32 = (w as f32 / width as f32 * height as f32) as u32;
        let resized_image = image.resize(w, h, FilterType::CatmullRom);
        let stem = path
            .file_stem()
            .ok_or_else(|| JsValue::from("Error retrieving file stem."))?
            .to_str()
            .ok_or_else(|| JsValue::from("Couldn't convert file stem to string"))?;
        let filename = &format!("{}-{}x{}.webp", String::from(stem), w, h)[..];
        let file_path = dir.join(filename);
        resized_image
            .save_with_format(file_path, ImageFormat::WebP)
            .map_err(|e| JsValue::from(format!("{:?}", e)))?;
        srcset.push_str(&format!("/{}/{} {}w", &s, &filename, &w)[..]);
        if w != screenwidths[screenwidths.len() - 1] {
            srcset.push(' ');
        }
    }

    srcset.push('"');

    Ok(srcset)
}
