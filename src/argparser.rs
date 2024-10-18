pub struct Argparser {
    args: std::env::Args,
}

impl Argparser {
    pub fn new(args: std::env::Args) -> Argparser {
        Argparser { args: args }
    }

    pub fn parse(&mut self, flags: Vec<String>, error_msg: String) -> Option<String> {
        let mut found_flag: bool = false;
        println!("{:?}", self.args);
        println!("{:?}", self.args.nth(1));
        println!("{:?}", self.args.nth(1));
        for i in 0..self.args.len() {
            println!("{}", i);
            // let val: String = self.args.nth(i).expect(&error_msg);
            println!("{}", self.args.nth(i).expect(&error_msg));
            if found_flag {
                println!("EYOO");
                return Some(self.args.nth(i).expect(&error_msg))
            } else if flags.contains(&self.args.nth(i).expect(&error_msg)) {
                found_flag = true;
            }
        }
        return None
    }
}