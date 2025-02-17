// use serde::{Deserialize, Serialize};
use pcre2::bytes::Regex;
use std::io::{Error, ErrorKind};
use std::path::Path;
use std::fs;
use urlencoding::decode;

fn generate_srcset(url: &String) -> Result<String, Error> {
    
    let decoded_url =
        decode(&url).map_err(|_| Error::new(ErrorKind::Other, "Failed to decode url string"))?;
    let path = Path::new(decoded_url.as_ref());
    let parent = path
        .parent()
        .ok_or_else(|| Error::new(ErrorKind::NotFound, "No parent found."))?;
    let parent_str = parent
        .to_str()
        .ok_or_else(|| Error::new(ErrorKind::Other, "Couldn't convert parent to string."))?;
    let regex = Regex::new(r"([^/]+?)$")
        .map_err(|_| Error::new(ErrorKind::Other, "Failed to compile regex."))?;
    let re_res = regex
        .find(parent_str.as_bytes())
        .map_err(|_| Error::new(ErrorKind::Other, "Regex search failed."))?;
    let re_match = re_res.ok_or_else(|| Error::new(ErrorKind::Other, "Regex match failed."))?;
    let s = std::str::from_utf8(re_match.as_bytes())
        .map_err(|_| Error::new(ErrorKind::Other, "String conversion failed."))?;
    let mut dir = std::env::current_dir()?;
    dir.push("public/");
    dir.push(s);
//    let dir = dir.join(s);
    let dir_str = dir.to_str().ok_or_else(|| { Error::new(ErrorKind::Other, "Failed to convert directory path to string.")})?;
    println!("dir_str: {}", dir_str);
    
    if !dir.exists() {
        fs::create_dir_all(dir.as_path()).map_err(|e| { Error::from(e)})?;
    }

    let srcset: String = "".to_string();

    Ok(srcset)
}

fn main() {
    let url: String = r#"https://images.ctfassets.net/jisgw3tevuum/aRYoVzIwh9B9EAYAhcEOe/60da1db184468e60a1d7df35b5e5b110/pexels-pixabay-416160.jpg"#.to_string();
    generate_srcset(&url);
}

//https://blog-clone-delta.vercel.app/_next/image?url=https%3A%2F%2Fimages.ctfassets.net%2Fjisgw3tevuum%2FaRYoVzIwh9B9EAYAhcEOe%2F60da1db184468e60a1d7df35b5e5b110%2Fpexels-pixabay-416160.jpg&w=3840&q=75
