use std::ffi::CStr;

use leptonica_plumbing::{
    leptonica_sys::{pixCreate, pixSetRGBPixel},
    Pix,
};
use opencv::{core::Vec3b, prelude::*};
use tesseract_plumbing::TessBaseApi;

pub struct Ocr {
    tesseract_api: TessBaseApi,
}

impl Ocr {
    pub fn init() -> Self {
        let mut tba = TessBaseApi::create();
        let lang = CStr::from_bytes_with_nul(b"eng\0").unwrap();
        tba.init_2(None, Some(lang))
            .expect("Initialize tesseract base api");
        tba.set_page_seg_mode(7);

        Ocr { tesseract_api: tba }
    }

    pub fn mat_to_pix(&self, mat: &Mat) -> Pix {
        let width = mat.cols();
        let height = mat.rows();
        let depth = 32;

        println!("{}", depth);

        let pix = unsafe { pixCreate(width, height, depth) };

        for (pos, val) in mat.iter::<Vec3b>().unwrap() {
            unsafe {
                pixSetRGBPixel(
                    pix,
                    pos.x,
                    pos.y,
                    val[0].into(),
                    val[1].into(),
                    val[2].into(),
                );
            }
        }

        unsafe { Pix::new_from_pointer(pix) }
    }

    pub fn set_image(&mut self, mat: &Mat) {
        let pix = self.mat_to_pix(mat);
        self.tesseract_api.set_image_2(&pix);
    }

    pub fn get_utf8_text(&mut self) -> String {
        let text = self.tesseract_api.get_utf8_text().unwrap();
        let cstr = text.as_ref();
        cstr.to_owned().into_string().unwrap()
    }

    pub fn get_utf8_text_from_mat(&mut self, mat: &Mat) -> String {
        self.set_image(mat);
        self.get_utf8_text()
    }
}
