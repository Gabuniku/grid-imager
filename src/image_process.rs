use crate::utils::GridImageConfig;
use image::imageops::FilterType;
use image::{DynamicImage, ImageReader, RgbaImage, imageops};
use log::debug;
use rayon::prelude::*;
use std::path::PathBuf;

pub fn generate_grid_image(
    config: &GridImageConfig,
    paths: Vec<PathBuf>,
) -> Result<RgbaImage, String> {
    let proc_images: Vec<Result<RgbaImage, String>> = paths
        .into_par_iter()
        .map(|path| {
            let image = match load_image(path) {
                Ok(img) => img,
                Err(e) => {
                    return Err(e);
                }
            };
            Ok(cropping_image(&config, image))
        })
        .collect();

    // いったんエラーは無視
    let crop_images = proc_images
        .into_iter()
        .filter_map(|img| img.ok())
        .collect::<Vec<_>>();

    let grid_image = composite_grid_image(&config, &crop_images);

    Ok(grid_image)
}

fn load_image(path: PathBuf) -> Result<DynamicImage, String> {
    debug!("loading image {}", path.display());
    match ImageReader::open(&path) {
        Ok(reader) => match reader.decode() {
            Ok(img) => Ok(img),
            Err(e) => {
                let error_msg = format!("{} [{}]", e, path.display());
                debug!("{}", error_msg);
                Err(error_msg)
            }
        },
        Err(e) => {
            let error_msg = format!("{} [{}]", e, path.display());
            debug!("{}", error_msg);
            Err(error_msg)
        }
    }
}

fn cropping_image(config: &GridImageConfig, image: DynamicImage) -> RgbaImage {
    let crop_size = image.width().min(image.height());
    let x = (image.width() / 2) - (crop_size / 2);
    let y = (image.height() / 2) - (crop_size / 2);

    let crop_img = image.crop_imm(x, y, crop_size, crop_size);

    let resized_img =
        crop_img.resize_exact(config.cell_size, config.cell_size, FilterType::Gaussian);

    resized_img.to_rgba8()
}

fn composite_grid_image(config: &GridImageConfig, crop_images: &Vec<RgbaImage>) -> RgbaImage {
    let rows = (crop_images.len() as u32 + config.columns) / config.columns;

    let mut grid_image = RgbaImage::from_pixel(
        config.cell_size * config.columns + (config.stroke * (config.columns + 1)),
        config.cell_size * rows + (config.stroke * (rows + 1)),
        config.grid_color,
    );

    for (i, img) in crop_images.iter().enumerate() {
        let index = i as u32;
        let grid_x = ((index % config.columns) * (config.cell_size + config.stroke) + config.stroke) as i64;
        let grid_y = ((index / config.columns) * (config.cell_size + config.stroke) + config.stroke) as i64;
        imageops::overlay(&mut grid_image, img, grid_x, grid_y);
    }
    grid_image
}
