use std::env;

mod compiler;

fn main() {
    let args: Vec<String> = env::args().collect();    

    if args.len() == 1 {println!("welcome home!")}

    if args.len() == 2 {
        let target_path: &String = &args[1];
        let content = compiler::compile(target_path);
        println!("{}", content);
    }

}