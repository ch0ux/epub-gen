use std::path::{Path, PathBuf};

use tempfile::TempDir;

use crate::{
    CusResult,
    compress::{self, CompressOptions},
    epub, extract,
};

#[derive(Debug, Clone)]
pub struct ImgInfo {
    pub origin_path: PathBuf,
    pub output_path: PathBuf,
    // pub idx: usize,
}

impl ImgInfo {
    pub fn build<P: AsRef<Path>>(origin_path: P) -> Self {
        let o_path: PathBuf = PathBuf::from(origin_path.as_ref());
        Self {
            origin_path: o_path.clone(),
            output_path: o_path.with_extension("jpg"),
            // idx: idx,
        }
    }
}

pub fn run_pipeline(
    input_zip: &Path,
    output_epub: &Path,
    quality: u8,
    preserve_order: bool,
) -> CusResult<()> {
    let temp_dir = TempDir::new()?;
    let extracted_dir = temp_dir.path().join("extracted");
    let compress_dir = temp_dir.path().join("compressed");

    let file_name = output_epub
        .file_name()
        .and_then(|s| s.to_str())
        .expect("can not get file name");

    // 解压 提取图片
    let mut images: Vec<ImgInfo> =
        extract::extract_image_to_dir_with_progress(input_zip, &extracted_dir)?;

    let total_image = images.len();
    println!("find {} images...", total_image);

    // 排序
    if preserve_order {
        images.sort_by_key(|k| k.origin_path.clone());
    }

    // 批量解压
    let compress_options = CompressOptions { quality };
    // 并行 此处顺序不确定
    compress::compress_images_parallel_with_progress(&images, &compress_dir, &compress_options)?;

    // 构建epub
    epub::create_epub(&images, output_epub, file_name)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::pipeline::ImgInfo;

    #[test]
    pub fn test_sort() {
        let mut imgs = Vec::<ImgInfo>::new();

        imgs.push(ImgInfo::build("103.png"));
        imgs.push(ImgInfo::build("102.png"));
        imgs.push(ImgInfo::build("101.png"));

        println!("{:?}", imgs);

        imgs.sort_by_key(|k| k.origin_path.clone());
        println!("{:?}", imgs);
    }
}
