use crate::{
    common::{BitMatrix, DetectorRXingResult},
    RXingResultPoint,
};

pub struct QRCodeDetectorResult {
    bit_source: BitMatrix,
    result_points: Vec<RXingResultPoint>,
}

impl QRCodeDetectorResult {
    pub fn new(bit_source: BitMatrix, result_points: Vec<RXingResultPoint>) -> Self {
        Self {
            bit_source,
            result_points,
        }
    }
}

impl DetectorRXingResult for QRCodeDetectorResult {
    fn getBits(&self) -> &crate::common::BitMatrix {
        &self.bit_source
    }

    fn getPoints(&self) -> &[crate::RXingResultPoint] {
        &self.result_points
    }
}
