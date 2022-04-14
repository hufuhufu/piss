use std::collections::HashMap;
// use clipboard_win::{formats, get_clipboard};
use opencv::{
    core::{no_array, norm2, Point, Rect, Size, Vector as cVec, NORM_L2},
    highgui, imgcodecs,
    imgproc::{
        self, bounding_rect, find_contours, resize, threshold, CHAIN_APPROX_SIMPLE, RETR_EXTERNAL,
        THRESH_BINARY_INV,
    },
    prelude::*,
};
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "pqh/images/items/"]
struct ItemAssets;

fn main() {
    // let clip = get_clipboard(formats::Bitmap).expect("Get bitmap from clipboard");
    // let ss_vec: cVec<u8> = cVec::from(clip);
    // let mut ss_mat = imgcodecs::imdecode(&ss_vec, imgcodecs::IMREAD_GRAYSCALE)?;
    // let ss_mat = imgcodecs::imread("ss.png", imgcodecs::IMREAD_GRAYSCALE)?;

    let ss = imgcodecs::imread("sss.png", imgcodecs::IMREAD_ANYCOLOR).unwrap();
    let ss_gray = imgcodecs::imread("sss.png", imgcodecs::IMREAD_GRAYSCALE).unwrap();

    let mut equip_hmap: HashMap<String, Mat> = HashMap::new();
    for filename in ItemAssets::iter() {
        let file: cVec<u8> = ItemAssets::get(filename.as_ref())
            .unwrap()
            .data
            .into_owned()
            .into();
        let mat = imgcodecs::imdecode(&file, imgcodecs::IMREAD_ANYCOLOR).unwrap();
        let id = filename.trim_end_matches(|c: char| !c.is_numeric());

        equip_hmap.insert(id.to_owned(), mat);
    }

    let mut thresh = Mat::default();
    threshold(&ss_gray, &mut thresh, 130.0, 255.0, THRESH_BINARY_INV).unwrap();

    let mut cntr: cVec<cVec<Point>> = cVec::new();
    find_contours(
        &thresh,
        &mut cntr,
        RETR_EXTERNAL,
        CHAIN_APPROX_SIMPLE,
        Point::new(0, 0),
    ).unwrap();

    let mut rect_freq: HashMap<(i32, i32), u32> = HashMap::new();

    let rects: cVec<Rect> = cntr
        .iter()
        .filter_map(|c| match bounding_rect(&c) {
            Ok(r) => {
                if r.width > 50 && r.height > 50 {
                    *rect_freq.entry((r.width, r.height)).or_insert(0) += 1;
                    Some(r)
                } else {
                    None
                }
            }
            Err(_) => None,
        })
        .collect();

    // For debugging purpose. Draws all bounding rectangles.
    // for r in &rects {
    //     // println!("{:?}", rect);
    //     rectangle(&mut ss, r, Scalar::new(0.0, 0.0, 255.0, 1.0), 2, LINE_8, 0)?;
    // }

    let equips: cVec<Mat> = rects
        .iter()
        .filter_map(|rec| {
            if let Ok(m) = Mat::roi(&ss, rec) {
                Some(m)
            } else {
                None
            }
        })
        .map(|mat| {
            if mat.cols() != 128 || mat.rows() != 128 {
                let mut rm = Mat::default();
                if resize(
                    &mat,
                    &mut rm,
                    Size::new(128, 128),
                    0.0,
                    0.0,
                    imgproc::INTER_CUBIC,
                )
                .is_ok()
                {
                    rm
                } else {
                    mat
                }
            } else {
                mat
            }
        })
        .collect();

    highgui::named_window("window", highgui::WINDOW_FULLSCREEN).unwrap();
    for eq in equips {
        let (id, score) = equip_hmap
            .iter()
            .fold(("", 0.0), |(acc_id, acc_score), (eq_k, eq_v)| {
                let norm = norm2(&eq, &eq_v, NORM_L2, &no_array()).unwrap_or(0.0);
                let sim = 1.0 - norm / (128.0 * 128.0);
                if acc_score >= sim {
                    (acc_id, acc_score)
                } else {
                    (eq_k, sim)
                }
            });
        if score < 0.5 {
            continue;
        }
        println!("Nearest match:\nid: {}\nscore: {}", id, score);

        loop {
            highgui::imshow("window", &eq).unwrap();
            let key = highgui::wait_key(1).unwrap();
            if key == 113 {
                break;
            }
        }
    }
}
