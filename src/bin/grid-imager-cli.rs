use clap::Parser;
use grid_imager::{
    image_process::generate_grid_image,
    utils::{search_files, GridImageConfig},
};
use image::Rgba;
use std::path::PathBuf;
use std::process::exit;
use log::debug;

#[derive(Parser, Debug)]
#[command(author, version = "0.1.0")]
struct Args {
    /// 読み込む画像のあるディレクトリ
    #[arg(short, long, value_name = "ImagesDirectory")]
    directory: Option<PathBuf>,

    /// 出力ファイル名
    #[arg(short, long, value_name = "output" ,default_values = ["grid_image.png"])]
    output: String,

    /// 1コマの大きさ
    #[arg(short, long, value_name = "size", default_value_t = 200)]
    size: u32,

    /// 列数
    #[arg(short, long, value_name = "columns")]
    columns: Option<u32>,

    /// グリッド線の幅 px
    #[arg(long, value_name = "stroke", default_value_t = 0)]
    stroke: u32,

    /// グリッド線の赤色の値
    #[arg(short, long, value_name = "red", default_value_t = 0)]
    red: u8,

    /// グリッド線の緑色の値
    #[arg(short, long, value_name = "green", default_value_t = 0)]
    green: u8,

    /// グリッド線の青色の値
    #[arg(short, long, value_name = "blue", default_value_t = 0)]
    blue: u8,

    /// グリッド線のアルファ値
    #[arg(short, long, value_name = "alpha", default_value_t = 255)]
    alpha: u8,

    /// 読み込む画像
    #[arg(value_name = "File")]
    file: Vec<PathBuf>,
}

/* CLI */
fn main() {
    env_logger::init();

    let args = Args::parse();

    let mut paths = Vec::new();

    if let Some(directory) = args.directory {
        let search = search_files(directory);
        paths.extend(search);
    }
    
    debug!("arg files {:?}", args.file);
    paths.extend(args.file);
    debug!("found {} files", paths.len());

    let columns = match args.columns {
        Some(columns) => {
            if columns == 0 {
                eprintln!("columns must be greater than 0");
                exit(-1);
            }
            columns
        }
        None => paths.len().isqrt() as u32,
    };

    let color = Rgba([args.red, args.green, args.blue, args.alpha]);

    let config = GridImageConfig {
        cell_size: args.size,
        columns: columns,
        stroke: args.stroke,
        grid_color: color,
    };

    if let Ok(grid_image) = generate_grid_image(&config, paths) {
        grid_image.save(&args.output).unwrap();
        debug!("saved grid image to {:?}", args.output);
    } else {
        eprintln!("Error generating grid image.");
    }
}
