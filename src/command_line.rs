use std::error::Error;
use crate::reader_interact::ReaderInteract;
use crate::reader_file::ReaderFile;
const HELP_FILE: &str = "-i | --interract 交互式\n-h | --help 打开该说明文档\n-f | --file 文件模式, 并且附上输入文件路径\n输入> 后接文件名可将输出导入至该文件.";
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    match (config.interract_mode, config.open_help, config.input_file_path, config.output_file_path) {
        (true, false, None, None) => {
            let mut reader_interact: ReaderInteract = ReaderInteract::new();
            reader_interact.call();
            Ok(())
        },
        (false, true, None, None) => {
            println!("{}", HELP_FILE);
            Ok(())
        },
        (false, false, Some(in_path), None) => {
            let mut reader_file: ReaderFile = ReaderFile::new(Some(in_path), None);
            reader_file.call();
            Ok(())
        },
        (false, false, Some(in_path), Some(out_path)) => {
            let mut reader_file: ReaderFile = ReaderFile::new(Some(in_path), Some(out_path));
            reader_file.call();
            Ok(()) 
        },
        _ => return Err("Conflict occur.\nPlease use 'minilisp -h' or 'minilisp --help' to check the usage".into()),
    }
}

pub struct Config {
    pub interract_mode: bool,
    pub open_help: bool,
    pub input_file_path: Option<String>,
    pub output_file_path: Option<String>,
}
impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        let mut interract_mode: bool = false;
        let mut open_help: bool = false;
        let mut input_file_path: Option<String> = None;
        let output_file_path: Option<String> = None;
        args.next();
        loop {
            match args.next() {
                None => break,
                Some(s) if s == "-i".to_string() || s == "--interract".to_string() => interract_mode = true,
                Some(s) if s == "-h".to_string() || s == "--help".to_string() => open_help = true,
                Some(s) if s == "-f".to_string() || s == "--file".to_string() => {
                    match args.next() {
                        None => return Err("Should give an input file path"),
                        Some(path) => input_file_path = Some(path),
                    }
                },
                /*Some(s) if s == "-o".to_string() || s == "--output".to_string() => {
                    match args.next() {
                        None => return Err("Should give an output file path"),
                        Some(path) => output_file_path = Some(path),
                    }
                },*/
                Some(s) if s == "-o".to_string() || s == "--output".to_string() => return Err("Unknown"),
                _ => return Err("Fail to parse the command, please retry"),
            }
        }
        Ok(Config { interract_mode, open_help, input_file_path, output_file_path })
    }
}