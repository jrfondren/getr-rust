mod getrusage;
use std::env::args;
use std::process::Command;

fn exe() -> Option<String> {
    Some(std::env::current_exe().ok()?.file_name()?.to_str()?.to_string())
}

fn usage() -> ! {
    eprintln!("Usage: {} <#runs> <command> [<args> ...]",
              exe().unwrap_or("getr".to_string()));
    std::process::exit(1)
}

fn main() {
    if args().len() < 3 { usage() }
    let runs: i32 = match args().nth(1).unwrap().parse() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("not a number: {}", args().nth(1).unwrap());
            usage()
        }
    };
    let cmd = args().nth(2).unwrap();
    let args: Vec<String> = args().skip(3).collect();
    for _ in 0 .. runs {
        match Command::new(&cmd).args(&args).spawn() {
            Ok(mut child) => { child.wait().unwrap(); },
            Err(err) => {
                eprintln!("failed to spawn command : {:?}", err);
                std::process::exit(1);
            }
        }
    }
    eprintln!("{}", getrusage::RUsage::new().report(runs));
}
