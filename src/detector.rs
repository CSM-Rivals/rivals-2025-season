use apriltag::{Detector, DetectorBuilder, Family, Image, Detection};
use image::GenericImageView;
use std::path::AsRef;
use std::path::Path;

pub struct TagDetector {
    detector: Detector,
}

impl TagDetector {
    /// Initializes the detector with the standard 36h11 family.
    pub fn new(bits: usize) -> Self {
        let family = Family::new_tag36h11();
        let detector = DetectorBuilder::new()
            .add_family_bits(family, bits)
            .build()
            .expect("Failed to initialize AprilTag detector");

        Self { detector }
    }

    /// Takes a filename/path, loads it, and returns detection results.
    pub fn detect_from_file<P: AsRef<Path>>(&mut self, path: P) -> anyhow::Result<Vec<Detection>> {
        // 1. Load image from disk
        let img = image::open(path)?;
        let (width, height) = img.dimensions();
        
        // 2. Convert to grayscale (Luma8)
        let luma = img.to_luma8();

        // 3. Create the AprilTag image view
        let apriltag_img = Image::from_raw_buffer(
            width as usize,
            height as usize,
            &luma,
            width as usize,
        ).map_err(|_| anyhow::anyhow!("Failed to create AprilTag buffer"))?;

        // 4. Return the detections
        Ok(self.detector.detect(&apriltag_img))
    }

    pub fn detect_from_raw(&mut self, data: Vec<u8>, width: i32, height: i32) -> anyhow::Result<Vec<Detection>> {
        // Create the AprilTag image view directly from the bytes provided by Python
        let apriltag_img = Image::from_raw_buffer(
            width as usize,
            height as usize,
            &data,
            width as usize, // stride is usually just width for grayscale
        ).map_err(|_| anyhow::anyhow!("Failed to create AprilTag buffer from Python data"))?;

        Ok(self.detector.detect(&apriltag_img))
    }
}