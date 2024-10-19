mod argparser;
mod picture;

fn main() {
    // Parse args
    let mut arg_parser: argparser::Argparser = argparser::Argparser::new(&mut std::env::args());
    let pic_path: Option<String> = arg_parser.parse(vec!["--image".to_string(), "-i".to_string()]);

    // Parse image and generate ASCII
    let pic: picture::MyPicture = picture::MyPicture::new(&pic_path.unwrap());
    let ascii: String = picture::MyPicture::to_ascii(&pic);
    std::fs::write("test.txt", &ascii).expect("Unable to write file");
}
