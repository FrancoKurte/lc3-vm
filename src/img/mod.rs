pub mod md;
pub mod dt;

#[allow(dead_code)]
pub struct Image {
    metadata : md::ImgMetadata,
    data: dt::ImgData,
}

impl Image {
    pub fn new_img() {

    }

    pub fn read_img() {
        todo!()
    }

    pub fn write_img() {
        todo!()
    }
}

pub fn verify_img(_img: Image) -> bool {
    todo!()
}
