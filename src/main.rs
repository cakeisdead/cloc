use std::{env, path::{PathBuf, Path}, fs::File, io::{self, BufRead}};
use ignore::WalkBuilder;
use clap::Parser;

#[derive(Parser, Debug, Default)]
#[clap(author="cakeisdead", about = "Simple Lines of Code counter", long_about = None)]
struct Cli {
    // Directory of aouew
    #[arg(long,short = 'p')]
    path: Option<String>,
    #[arg(long, short = 'l')]
    log: bool,
}

fn main() {
    
    // Parse args
    let args = Cli::parse();
    
    // Define directory where LOCs will be counted
    let path: String = match args.path {
        Some(path) => path.to_string(),
        None => get_current_directory(),
    };

    let mut total_lines: u32 = 0;
    
    // Walker options: "ignore.g" includes a list of additional files to be excluded
    // TODO find a way to specify files to be included instead
    let mut walker_opts = WalkBuilder::new(&path);
    walker_opts.add_ignore("./ignore.g");
    let walker = walker_opts.build();

    // Iterate through all results looking for files
    for result in walker {
        match result {
            Ok(entry) => {
                if entry.path().is_file() {
                    // Count lines if is file
                    if let Ok(lines) = read_lines(entry.path()) {
                        // Consumes the iterator, returns an (Optional) String
                        let mut line_count: u32 = 0;
                        for _line in lines {
                            //TODO Validate if line should be counted (exclude blanks and comments)
                            line_count += 1;
                        }
                        total_lines += line_count;
                        println!("{}: {} lines", entry.file_name().to_string_lossy(), line_count);
                    }
                }
            },
            Err(err) => {
                println!("ERROR: {}", err)
            },
        }
    }
    
    // Save stats to db
    // Get parent folder name
    let target_folder = PathBuf::from(&path);

    match target_folder.file_name() {
        Some(parent)=> {
            if args.log {
                update_log(parent.to_os_string().to_string_lossy().to_string())
            }
            println!("LOC in {}: {}", parent.to_os_string().to_string_lossy().to_string(), total_lines);
        },    
        None => {
            println!("Unable to get the parent folder.")
        },
    };
}

fn update_log(path: String){
    println!("Line count for {} will be updated", path);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_current_directory() -> String {
    let cwd = env::current_dir();
    match cwd {
        Ok(cwd) => cwd.into_os_string().into_string().unwrap(),
        Err(_) => "Unable to get current working directory.".to_string()
    }
}