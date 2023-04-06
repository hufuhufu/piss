mod ocr;
mod scanner;

use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "pqh/images/items/"]
struct ItemAssets;

fn main() {
    
}
