use std::{
    fs::{self, File},
    path::Path,
    process,
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
    thread,
    time::Duration,
};

use image::{ImageEncoder, codecs::jpeg::JpegEncoder};
use indicatif::ProgressBar;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::{CusResult, pipeline::ImgInfo};

#[derive(Debug)]
pub struct CompressOptions {
    pub quality: u8,
}

pub fn compress_img(input_path: &Path, output_path: &Path, quality: u8) -> CusResult<()> {
    let img = image::open(input_path)?;

    let mut buf = File::create(output_path)?;

    let encoder = JpegEncoder::new_with_quality(&mut buf, quality);
    encoder.write_image(
        img.as_bytes(),
        img.width(),
        img.height(),
        img.color().into(),
    )?;

    Ok(())
}

pub fn compress_images_parallel(
    images: &Vec<ImgInfo>,
    output_dir: &Path,
    options: &CompressOptions,
    completed: Arc<AtomicUsize>,
) -> CusResult<()> {
    fs::create_dir_all(output_dir)?;

    let results: Vec<CusResult<()>> = images
        .par_iter()
        .map(|img| {
            compress_img(&img.origin_path, &img.output_path, options.quality)?;

            completed.fetch_add(1, Ordering::Relaxed);
            Ok(())
        })
        .collect();

    for ele in results {
        ele?
    }

    Ok(())
}

pub fn compress_images_parallel_with_progress(
    images: &Vec<ImgInfo>,
    output_dir: &Path,
    options: &CompressOptions,
) -> CusResult<()> {
    let total_image = images.len();

    // 设置进度条
    let pb = ProgressBar::new(total_image as u64);
    pb.set_style(
        indicatif::ProgressStyle::default_bar()
            .template(
                "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})",
            )?
            .progress_chars("#>-"),
    );

    let completed = Arc::new(AtomicUsize::new(0));

    let pb_clone = pb.clone();
    let completed_clone = completed.clone();
    let pb_thread = thread::spawn(move || {
        loop {
            let current = completed_clone.load(Ordering::Relaxed);
            pb_clone.set_position(current as u64);
            if current >= total_image {
                break;
            }
            thread::sleep(Duration::from_millis(100));
        }
    });

    let _ = compress_images_parallel(images, output_dir, options, completed)?;

    if let Err(e) = pb_thread.join() {
        println!("error: {:?}", e);
        process::exit(1)
    }
    pb.finish_with_message("compression completed!");

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::compress::compress_img;

    #[test]
    fn test_compress() {
        let r = compress_img(
            &Path::new("test-jpg.jpg"),
            &Path::new("test-jpg-1.jpg"),
            80,
        );
        assert!(r.is_ok());
        let r = compress_img(
            &Path::new("test-png.png"),
            &Path::new("test-png-1.jpg"),
            80,
        );
        assert!(r.is_ok());
    }
}
