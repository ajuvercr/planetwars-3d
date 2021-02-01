use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub async fn fetch(url: &str) -> Result<String, String> {
    let url = format!("static/{}", url);
    let file = File::open(url).map_err(|e| format!("Fetch failed {:?}", e.kind()))?;
    let mut buf_reader = BufReader::new(file);

    let mut contents = String::new();
    buf_reader
        .read_to_string(&mut contents)
        .map_err(|e| format!("Fetch failed {:?}", e.kind()))?;

    Ok(contents)
}
