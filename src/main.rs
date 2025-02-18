use http_client::HttpClient;
use http_client::{h1::H1Client, Request};
use image::{imageops::FilterType, ImageFormat, ImageReader};
use pcre2::bytes::Regex;
use std::fs;
use std::io::{Cursor, Error, ErrorKind::*};
use std::path::Path;
use tokio::main;
use urlencoding::decode;

async fn generate_srcset(url: &str) -> Result<String, Error> {
    let decoded_url = decode(url).map_err(|_| Error::new(Other, "Failed to decode url string"))?;
    let path = Path::new(decoded_url.as_ref());
    let parent = path
        .parent()
        .ok_or_else(|| Error::new(NotFound, "No parent found."))?;
    let parent_str = parent
        .to_str()
        .ok_or_else(|| Error::new(Other, "Couldn't convert parent to string."))?;
    let regex =
        Regex::new(r"([^/]+?)$").map_err(|_| Error::new(Other, "Failed to compile regex."))?;
    let re_res = regex
        .find(parent_str.as_bytes())
        .map_err(|_| Error::new(Other, "Regex search failed."))?;
    let re_match = re_res.ok_or_else(|| Error::new(Other, "Regex match failed."))?;
    let s = std::str::from_utf8(re_match.as_bytes())
        .map_err(|_| Error::new(Other, "String conversion failed."))?;
    let mut dir = std::env::current_dir()?;
    dir.push("public/");
    dir.push(s);

    if !dir.exists() {
        fs::create_dir_all(dir.as_path()).map_err(Error::from)?;
    }

    let req = Request::get(decoded_url.as_ref());

    let client = H1Client::new();
    let mut res = client
        .send(req)
        .await
        .map_err(|_| Error::new(Other, "error fetching image"))?;
    let bytes = res.body_bytes().await.map_err(|e| Error::new(Other, e))?;
    println!("bytes created");
    let reader = ImageReader::new(Cursor::new(bytes.as_slice()))
        .with_guessed_format()
        .map_err(|e| Error::new(Other, format!("{:?}", e)))?;

    let image = reader
        .decode()
        .map_err(|e| Error::new(Other, format!("error generating image: {:?}", e)))?;
    let [width, height] = [image.width(), image.height()];
    let screenwidths = [1536, 1280, 1024, 768, 640];
    let mut srcset: String = "\"".to_string();
    for w in screenwidths {
        let h: u32 = (w as f32 / width as f32 * height as f32) as u32;
        let resized_image = image.resize(w, h, FilterType::CatmullRom);
        let stem = path
            .file_stem()
            .ok_or_else(|| Error::new(Other, "Error retrieving file stem."))?
            .to_str()
            .ok_or_else(|| Error::new(Other, "Couldn't convert file stem to string"))?;
        let filename = &format!("{}-{}x{}.webp", String::from(stem), w, h)[..];
        let file_path = dir.join(filename);
        resized_image
            .save_with_format(file_path, ImageFormat::WebP)
            .map_err(|e| Error::new(Other, format!("{:?}", e)))?;
        srcset.push_str(&format!("/{}/{} {}w", &s, &filename, &w)[..]);
        if w != screenwidths[screenwidths.len() - 1] {
            srcset.push(' ');
        }
    }

    srcset.push('"');

    Ok(srcset)
}

#[main]
async fn main() {
    let url: String = r#"https://images.ctfassets.net/jisgw3tevuum/aRYoVzIwh9B9EAYAhcEOe/60da1db184468e60a1d7df35b5e5b110/pexels-pixabay-416160.jpg"#.to_string();
    let res = generate_srcset(&url).await;
    if let Ok(srcset) = res {
        println!("srcset: {}", srcset);
    };
}

//https://blog-clone-delta.vercel.app/_next/image?url=https%3A%2F%2Fimages.ctfassets.net%2Fjisgw3tevuum%2FaRYoVzIwh9B9EAYAhcEOe%2F60da1db184468e60a1d7df35b5e5b110%2Fpexels-pixabay-416160.jpg&w=3840&q=75
