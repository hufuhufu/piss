use crate::ocr::Ocr;
use crate::ItemAssets;

use std::collections::HashMap;
// use clipboard_win::{formats, get_clipboard};
use opencv::{
    core::{no_array, norm2, Point, Rect, Size, Vector, NORM_L2},
    highgui, imgcodecs,
    imgproc::{
        self, bounding_rect, find_contours, resize, threshold, CHAIN_APPROX_SIMPLE, RETR_EXTERNAL,
        THRESH_BINARY_INV,
    },
    prelude::*,
};

pub fn scan() {
    // let clip = get_clipboard(formats::Bitmap).expect("Get bitmap from clipboard");
    // let ss_vec: Vector<u8> = Vector::from(clip);
    // let mut ss_mat = imgcodecs::imdecode(&ss_vec, imgcodecs::IMREAD_GRAYSCALE)?;
    // let ss_mat = imgcodecs::imread("ss.png", imgcodecs::IMREAD_GRAYSCALE)?;

    let ss = imgcodecs::imread("sss.png", imgcodecs::IMREAD_ANYCOLOR).unwrap();
    let ss_gray = imgcodecs::imread("sss.png", imgcodecs::IMREAD_GRAYSCALE).unwrap();

    let mut equip_hmap: HashMap<String, Mat> = HashMap::new();
    for filename in ItemAssets::iter() {
        let file: Vector<u8> = ItemAssets::get(filename.as_ref())
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

    let mut cntr: Vector<Vector<Point>> = Vector::new();
    find_contours(
        &thresh,
        &mut cntr,
        RETR_EXTERNAL,
        CHAIN_APPROX_SIMPLE,
        Point::new(0, 0),
    )
    .unwrap();

    let rects: Vector<Rect> = cntr
        .iter()
        .filter_map(|c| match bounding_rect(&c) {
            Ok(r) => {
                if r.width > 50 && r.height > 50 {
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

    let equips: Vector<Mat> = rects
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

    let mut ocr = Ocr::init();

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

        let crop_rect = Rect::from_points((9, 99).into(), (124, 119).into());
        let cropped_eq = Mat::roi(&eq.clone(), crop_rect).unwrap();

        let amount = ocr
            .get_utf8_text_from_mat(&cropped_eq)
            .matches(char::is_numeric)
            .collect::<Vec<&str>>()
            .join("");

        println!(
            "Nearest match:\nid: {}\nscore: {}\namount: {:?}",
            id, score, amount
        );

        loop {
            highgui::imshow("window", &eq).unwrap();
            let key = highgui::wait_key(1).unwrap();
            if key == 113 {
                // Press q
                break;
            }
        }
    }
}
