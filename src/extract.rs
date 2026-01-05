use std::{fs, io, path::Path};

use indicatif::ProgressBar;
use zip::ZipArchive;

use crate::{CusResult, pipeline::ImgInfo};

pub fn extract_image_to_dir_with_progress<P: AsRef<Path>>(
    zip_path: P,
    output_dir: P,
) -> CusResult<Vec<ImgInfo>> {
    let file = fs::File::open(&zip_path)?;
    let mut archive = ZipArchive::new(file)?;

    fs::create_dir_all(&output_dir)?;

    let mut images = Vec::<ImgInfo>::new();

    // 获取图片数量
    let total_files = archive.len();

    // 设置进度条
    let pb = ProgressBar::new(total_files as u64);
    pb.set_style(indicatif::ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta}) Extracting images...")?
        .progress_chars("#>-"));

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let out_path = output_dir.as_ref().join(file.name());

        if is_image_file(&out_path) {
            if (&*file.name()).ends_with('/') {
                fs::create_dir_all(&out_path)?;
            } else {
                if let Some(p) = out_path.parent() {
                    fs::create_dir_all(p)?;
                }
                let mut outfile = fs::File::create(&out_path)?;
                io::copy(&mut file, &mut outfile)?;
                images.push(ImgInfo::build(out_path));
            }
        }
        // 更新进度：每处理一个 ZIP 条目（无论是否图片）都 +1
        pb.inc(1);
    }

    pb.finish_with_message("extraction completed!");

    Ok(images)
}

fn is_image_file(path: &Path) -> bool {
    matches!(
        path.extension().and_then(|e| e.to_str()),
        Some("jpg") | Some("jpeg") | Some("png") | Some("webp")
    )
}
