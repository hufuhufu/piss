use crate::{assets::ItemAssets, ocr::Ocr};

use opencv::{
    core::{no_array, norm2, Point, Rect, Size, Vector, NORM_L2},
    imgcodecs, imgproc,
    prelude::*,
};

fn threshold(src: &Mat) -> Mat {
    let mut thresh = Mat::default();
    imgproc::threshold(src, &mut thresh, 130.0, 255.0, imgproc::THRESH_BINARY_INV).unwrap();
    thresh
}

fn find_contours(threshold: &Mat) -> Vector<Vector<Point>> {
    let mut contours: Vector<Vector<Point>> = Vector::new();
    imgproc::find_contours(
        &threshold,
        &mut contours,
        imgproc::RETR_EXTERNAL,
        imgproc::CHAIN_APPROX_SIMPLE,
        Point::new(0, 0),
    )
    .unwrap();
    contours
}

fn bounding_rect_from_contour(contour: Vector<Point>) -> Option<Rect> {
    match imgproc::bounding_rect(&contour) {
        Ok(r) => {
            if r.width > 50 && r.height > 50 {
                Some(r)
            } else {
                None
            }
        }
        Err(_) => None,
    }
}

fn crop_image_with_rect(image: &Mat) -> impl FnMut(Rect) -> Option<Mat> + '_ {
    |rect: Rect| {
        let roi = Mat::roi(image, rect);
        if let Ok(roi) = roi {
            Some(roi)
        } else {
            None
        }
    }
}

fn try_resize(mat: Mat) -> Mat {
    if mat.cols() == 128 || mat.rows() == 128 {
        return mat;
    }

    let mut resized = Mat::default();
    let result = imgproc::resize(
        &mat,
        &mut resized,
        Size::new(128, 128),
        0.0,
        0.0,
        imgproc::INTER_CUBIC,
    );
    if result.is_ok() {
        resized
    } else {
        mat
    }
}

// omg what the heck is this function return
fn compare_equip<'a, 'b>(
    equip: &Mat,
) -> impl FnMut((&'a str, f64), (&'b String, &'b Mat)) -> (&'a str, f64) + '_
where
    'b: 'a,
{
    |(acc_id, acc_score): (&'a str, f64),
     (equip_id, equip_mat): (&'b String, &'b Mat)|
     -> (&'a str, f64) {
        // Find similarity in two images with NORM_L2
        // https://stackoverflow.com/a/19708947/6466576
        let norm = norm2(equip, &equip_mat, NORM_L2, &no_array()).unwrap_or(0.0);
        let similarity = 1.0 - norm / (128.0 * 128.0);
        if acc_score >= similarity {
            (acc_id, acc_score)
        } else {
            (equip_id, similarity)
        }
    }
}

pub fn scan_file(filename: &str) {
    // let clip = get_clipboard(formats::Bitmap).expect("Get bitmap from clipboard");
    // let ss_vec: Vector<u8> = Vector::from(clip);
    // let mut ss_mat = imgcodecs::imdecode(&ss_vec, imgcodecs::IMREAD_GRAYSCALE)?;
    // let ss_mat = imgcodecs::imread("ss.png", imgcodecs::IMREAD_GRAYSCALE)?;

    let img_color = imgcodecs::imread(filename, imgcodecs::IMREAD_ANYCOLOR).unwrap();
    let img_gray = imgcodecs::imread(filename, imgcodecs::IMREAD_GRAYSCALE).unwrap();

    let item_assets = ItemAssets::global();
    let thresh = threshold(&img_gray);
    let contours = find_contours(&thresh);

    let rects: Vector<Rect> = contours
        .iter()
        .filter_map(bounding_rect_from_contour)
        .collect();

    let equips: Vector<Mat> = rects
        .iter()
        .filter_map(crop_image_with_rect(&img_color))
        .map(try_resize)
        .collect();

    let mut ocr = Ocr::init();

    for equip in equips {
        let items = item_assets.assets.lock().unwrap();
        let (id, score) = items.iter().fold(("", 0.0), compare_equip(&equip));
        if score < 0.5 {
            continue;
        }

        // TODO: Need to take into account that PriconneTL patch has slightly bigger font.
        // Maybe create option like a PriconneTL toggle? Or customizeable crop rect for ocr?
        let crop_rect = Rect::from_points((9, 99).into(), (124, 119).into());
        let cropped_eq = Mat::roi(&equip.clone(), crop_rect).unwrap();

        let amount = ocr
            .get_utf8_text_from_mat(&cropped_eq)
            .matches(char::is_numeric)
            .collect::<Vec<&str>>()
            .join("");

        println!(
            "Nearest match:\nid: {}\nscore: {}\namount: {:?}",
            id, score, amount
        );
    }
}
