use console::{style, Term};

use std::{
    env,
    error::Error,
    fs,
    path::{Path, PathBuf},
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut term = Term::stdout();
    print_horizontal_line(&mut term)?;
    match || -> Result<(), Box<dyn Error>> {
        let args = get_args()?;
        let files = get_files(&args)?;
        let mut total_lines = 0;
        for file in files {
            let lines = lines_of_code(&file)?;
            term.write_line(format!(
                "{}: {}",
                style(file.display()).green(),
                style(&lines).cyan()
            ).as_str())?;
            total_lines += lines;
        }
        print_horizontal_line(&mut term)?;
        term.write_line(format!(
            "{}: {}",
            style("Total lines").green(),
            style(total_lines).cyan()
        ).as_str())?;
        print_horizontal_line(&mut term)?;
        Ok(())
    }() {
        Ok(_) => Ok(()),
        Err(e) => {
            print_horizontal_line(&mut term)?;
            term.write_line(format!("{}: {}", style("Error").red(), style(e).white()).as_str())?;
            print_horizontal_line(&mut term)?;
            std::process::exit(1);
        }
    }
}

fn print_horizontal_line(term: &mut Term) -> Result<(), std::io::Error> {
    term.write_line(&"─".repeat(80))
}

struct Args {
    directory: PathBuf,
    file_ext: Vec<String>,
}

fn get_args() -> Result<Args, Box<dyn Error>> {
    let mut args = env::args().skip(1);
    let directory = args.next().unwrap_or(".".to_string());
    let file_ext = args.next().map(|ext| ext.split(',').map(|s| s.to_string()).collect()).unwrap_or(Vec::new());
    Ok(Args {
        directory: PathBuf::from(directory),
        file_ext,
    })
}
/// Recursively get all files in a directory, filtering by file extension if provided.
fn get_files(args: &Args) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    let mut files = Vec::new();
    visit_directory(&args.directory, args.file_ext.clone(), &mut files)?;
    Ok(files)
}

fn visit_directory(
    directory: &Path,
    file_ext: Vec<String>,
    files: &mut Vec<PathBuf>,
) -> Result<(), Box<dyn Error>> {
    if directory.is_dir() {
        for entry in fs::read_dir(directory)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                visit_directory(&path, file_ext.clone(), files)?;
            } else if let Some(ext) = path.extension() {
                if file_ext.contains(&ext.to_string_lossy().to_string()) || file_ext.is_empty() {
                    files.push(path);
                }
            }
        }
    }

    Ok(())
}

fn lines_of_code(file: &Path) -> Result<usize, Box<dyn Error>> {
    let contents = fs::read_to_string(file)?;
    Ok(contents.lines().count())
}
