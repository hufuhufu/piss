use once_cell::sync::OnceCell;
use opencv::{
    core::{Mat, Vector},
    imgcodecs,
};
use rust_embed::RustEmbed;
use std::{collections::HashMap, sync::Mutex};

#[derive(RustEmbed)]
#[folder = "pqh/images/items/"]
struct ItemAssetsEmbed;

#[derive(Debug)]
pub struct ItemAssets {
    pub assets: Mutex<HashMap<String, Mat>>,
}
pub static ITEM_ASSETS: OnceCell<ItemAssets> = OnceCell::new();

impl ItemAssets {
    pub fn new() -> ItemAssets {
        let mut assets = HashMap::new();

        for filename in ItemAssetsEmbed::iter() {
            let embed_file = ItemAssetsEmbed::get(filename.as_ref()).unwrap();
            let buf: Vector<u8> = embed_file.data.into_owned().into();
            let id = filename.trim_end_matches(|c: char| !c.is_numeric());
            let flags = imgcodecs::IMREAD_ANYCOLOR;

            let mat = imgcodecs::imdecode(&buf, flags).unwrap();
            assets.insert(id.to_owned(), mat);
        }

        let assets = Mutex::new(assets);
        Self { assets }
    }

    pub fn global() -> &'static ItemAssets {
        ITEM_ASSETS.get().expect("item assets is not initialized")
    }
}
