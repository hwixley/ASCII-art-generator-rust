mod argparser;
mod picture;

fn main() {
    // 1. -- Parse args --
    let mut arg_parser: argparser::Argparser = argparser::Argparser::new(&mut std::env::args());

    // INPUT IMAGE
    let pic_path: String = arg_parser.parse(vec!["--image".to_string(), "-i".to_string()], false).unwrap_or("building.jpeg".to_string());

    // TARGET WIDTH
    let target_width: u32 = arg_parser.parse(vec!["--twidth".to_string(), "-tw".to_string()], false).unwrap_or(512.to_string()).parse().unwrap();

    // STDOUT
    let stdoutput: bool = arg_parser.parse(vec!["--stdoutput".to_string(), "-s".to_string()], true).unwrap_or(true.to_string()) == true.to_string();

    // FILE OUTPUT
    let foutput: bool = arg_parser.parse(vec!["--foutput".to_string(), "-f".to_string()], true).unwrap_or(false.to_string()) == true.to_string();


    // 2. -- Parse image and generate ASCII --
    let pic: picture::MyPicture = picture::MyPicture::new(&pic_path);
    let ascii: String = picture::MyPicture::to_ascii(&pic, target_width);


    // 3. -- Perform actions --
    if foutput {
        let mut fname: String = pic_path.clone();
        fname.push_str(".ASCII.txt");
        println!("Saving generated art to '{}'...", fname);
        std::fs::write(fname, &ascii).expect("Unable to write file");
    }
    
    if stdoutput {
        println!("{}", ascii);
    }
}
