pub struct Argparser {
    args: Vec<String>,
}

impl Argparser {
    pub fn new(args: &mut std::env::Args) -> Argparser {
        let mut argvec: Vec<String> = vec![];
        while let Some(arg) = args.next() {
            argvec.push(arg);
        }
        Argparser { args: argvec }
    }

    // pub fn get(&self) -> &Vec<String> {
    //     return &self.args
    // }

    pub fn parse(&mut self, flags: Vec<String>, toggle: bool) -> Option<String> {
        let mut found_flag: bool = false;
        let mut iter: std::slice::Iter<'_, String> = self.args.iter();
        while let Some(arg) = iter.next() {
            if found_flag {
                return Some(arg.to_string())
            } else if flags.contains(&arg.to_string()) {
                if toggle {
                    return Some("true".to_string())
                }
                found_flag = true;
            }
            
        }
        return None
    }
}