use clap::Parser;

#[derive(Debug, Parser)]
pub struct Args {
    #[arg(
        short,
        long,
        help = "The path to the input compressed file (e.g., images.zip)"
    )]
    pub input: String,

    #[arg(
        short,
        long,
        default_value = "output.epub",
        help = "The output EPUB file path"
    )]
    pub output: String,

    #[arg(long, default_value_t = 80, help = "Image quality (1-100)")]
    pub quality: u8,

    #[arg(
        long,
        default_value_t = false,
        help = "keep the original file name order"
    )]
    pub preserve_order: bool,
}
