use std::{
    fs::File,
    io::{Cursor, Read},
    path::Path,
};

use epub_builder::{EpubBuilder, EpubContent, ReferenceType, ZipLibrary};
use indicatif::ProgressBar;
use rust_embed::Embed;

use crate::{CusResult, pipeline::ImgInfo};

#[derive(Embed)]
#[folder = "template/"]
struct EpubAsset;

pub fn create_epub<P: AsRef<Path>>(
    images: &Vec<ImgInfo>,
    output_path: P,
    file_name: &str,
) -> CusResult<()> {
    let mut epub_builder = EpubBuilder::new(ZipLibrary::new()?)?;

    epub_builder.set_title(file_name);

    let css_data = EpubAsset::get("book.css")
        .expect("can not load epub template")
        .data
        .into_owned();

    epub_builder.add_resource("book.css".to_string(), Cursor::new(css_data), "text/css")?;

    // progressbar
    let pb = ProgressBar::new(images.len() as u64);
    pb.set_style(indicatif::ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta}) generate epub...")?
        .progress_chars("#>-"));

    for img_info in images {
        let image_path = &img_info.output_path;
        let image_name = image_path
            .file_name()
            .expect("can not get image filename")
            .to_str()
            .expect("can not convert filename to str");

        epub_builder.add_resource(
            format!("images/{}.jpg", image_name),
            File::open(&image_path)?,
            "image/jpeg",
        )?;

        // 填充html模板
        let content =
            replace_template_val(image_name, format!("../images/{}.jpg", image_name).as_str())?;

        epub_builder.add_content(
            EpubContent::new(format!("text/{}.html", image_name), content.as_bytes())
                .title(image_name)
                .reftype(ReferenceType::Text),
        )?;
        // 更新进度
        pb.inc(1);
    }

    let write = File::create(output_path)?;
    epub_builder.generate(&write)?;

    pb.finish_with_message("generate completed!");

    Ok(())
}

fn replace_template_val(title: &str, image: &str) -> std::io::Result<String> {
    let mut content = read_template_file_to_string()?;

    content = content.replace("#{title}", title);
    content = content.replace("#{image}", image);

    Ok(content)
}
fn read_template_file_to_string() -> std::io::Result<String> {
    let mut contents = String::new();
    let mut file = EpubAsset::get("book.html")
        .expect("can not load epub template")
        .data
        .into_owned();

    Cursor::new(&mut file).read_to_string(&mut contents)?;

    Ok(contents)
}
