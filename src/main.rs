mod argparser;
mod picture;

fn main() {
    // Parse args
    let args: std::env::Args = std::env::args();
    let mut arg_parser: argparser::Argparser = argparser::Argparser::new(args);
    let pic_path: Option<String> = argparser::Argparser::parse(&mut arg_parser, vec!["--image".to_string(), "-i".to_string()], "ERROR: No input image specified...".to_string());

    println!("{:?}", pic_path);
    // Parse image and generate ASCII
    let pic: picture::MyPicture = picture::MyPicture::new(&pic_path.unwrap());
    let ascii: String = picture::MyPicture::to_ascii(&pic);
    std::fs::write("test.txt", &ascii).expect("Unable to write file");
}
