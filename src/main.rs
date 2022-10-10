//use std::io::Error;
//use std::io;
//use std::path::PathBuf;
use clap::Parser;
use std::env;
use std::fs::{self, File};
use std::io::prelude::*;
use std::process::Command;
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    action: Action
}

#[derive(clap::Subcommand, Debug)]
enum Action {
    Download {

        //Youtube link to download
        #[arg(short, long)]
        link: String
    }, 
    Split {
        //The input file name
        #[arg(short, long, default_value = "input.opus")]
        input_name: String,

        //The timestamp file name
        #[arg(short, long, default_value = "timestamps.txt")]
        timestamps_name: String
    }
}
    

/**
 * Used to hold the parsing method needed
 * NONE used for default
 * Format used for formats other than default
 */
enum ParseMethod {
    NONE,
    Format((Vec<String>, bool))
}

/**
 * Used to hold the contents of the timestamps file as well as the 
 * format in which that file is in
 */
struct FileContents {
    format: Vec<String>,
    data: Vec<Vec<String>>
}


/**
 * Runs all split commands after the timestamps file gets parsed
 *
 * Params:
 * f - the parsed contents of the timestamps file
 * input_path - the path to the input opus file 
 */
fn run_split_commands(f: FileContents, input_path: String) {
    let start_index: usize = f.format.iter().position(|x| x == "start").unwrap();
    let end_index: usize = f.format.iter().position(|x| x == "end").unwrap();
    let name_index: usize = f.format.iter().position(|x| x == "name").unwrap();

    for point in f.data {
        let start_time = &point[start_index];
        let end_time = &point[end_index];
        let name = &point[name_index];

        let command: String = construct_command(&input_path, name, start_time, end_time);
        run_ffmpeg_commands(command);
    }
}

/**
 * Constructs the ffmpeg command for converting and trimming opus file to size
 * 
 * Params:
 * input_path - the path to the opus file
 * filename - the name of the file we want to output from ffmpeg
 * beginning - the starting time in the opus file
 * end - the ending time in the opus file
 */
fn construct_command(input_path: &String, filename: &String,
                     beginning: &String, end: &String) -> String {
    //ffmpeg -i inputmp3 -acodec copy -ss hh:mm:ss -to hh:mm:ss newname
    let string_command: String = 
        format_args!("ffmpeg -i {} -acodec libmp3lame -ss {} -to {} file:\"{}\" -y",
                     input_path, beginning, end, filename).to_string();

    return string_command;
}

#[warn(dead_code)]
fn print_directory_contents() {
    for i in fs::read_dir(".").unwrap() {
        println!("Files in dir: {}", i.unwrap().path().display());
    }

}


/**
 * The starting function where the timestamps.txt file is parsed
 *
 * Params: 
 * file_path - the path to the timestamps file
 *
 * Returns:
 * the parsed file contents 
 */
fn separate_file_contents(file_path: String) -> FileContents {
    let file_res = File::open(&file_path);
    
    //Handle the opening of the file
    let mut file = match file_res {
        Ok(file_opened) => {
            file_opened
        },
        Err(error) => {
            eprintln!("Error opening file: {}", error);
            eprintln!("File path {:?}", file_path);
            panic!()
        }
    };
    
    //Read all contents of file
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let file: FileContents = parse_file(contents);

    file 
}

/**
 * Determines the format of the file. If the file does not include the format 
 * string as the first line in the program, it defaults to using "name start end"
 * as the format
 *
 * Params:
 * file_contents - the contents of the entire timestamps.txt file
 *
 * Returns:
 * the format of the timestamps.txt file
 */
fn find_file_format(file_contents: &String) -> ParseMethod {
    //Split overall string by newline characters
    let split_contents: Vec<String> = 
        file_contents.split("\n").filter(|x| x.len() > 0).map(|x| x.to_string()).collect();
    let lower = split_contents[0].to_lowercase();
    
    let has_format_line: bool = lower.contains("start") && 
        lower.contains("end") && lower.contains("name");

    //Default format for file
    let mut format: ParseMethod = ParseMethod::NONE;
    if has_format_line {
        let mut expected_strings: Vec<String> = ["start", "end", "name"].iter().map(|x| x.to_string()).collect();

        //Format line split
        let split_lower: Vec<String> = lower.split(" ").map(|x| x.to_string()).collect();
        let mut format_vec: Vec<String> = Vec::new(); //Find format vector

        //Remove strings from expected_strings in order they appear in format line
        //Makes repeats impossible
        for split in split_lower {
            if expected_strings.contains(&split) {
                let index = expected_strings.iter().position(|x| x == &split).unwrap();
                format_vec.push(expected_strings.remove(index));
            }
        }

        //Check that we have all strings like we thought it would
        if format_vec.len() != 3 {
            eprintln!("File format line contains incorrect text.\nAny permutation of the following will work:\n\tstart end name");
            eprintln!("Separate format line with spaces only.");
            panic!();
        }
        format = ParseMethod::Format((format_vec, true))
    }

    format
}

/**
 * Given the file contents, determine the format of the contents inside the file 
 * and parse the file according to that format.
 *
 * Params:
 * file_contents - The contents of the timestamps.txt file
 *
 * Returns: 
 * The contents of the file parsed correctly
 */
fn parse_file(file_contents: String) -> FileContents {
    //The format the timestamps file is in
    let mut format: ParseMethod = find_file_format(&file_contents);
    let (format_strings, has_format) = match format {
        ParseMethod::NONE => {
            let format_strings: Vec<String> = ["name", "start", "end"].iter().map(|x| x.to_string()).collect();
            (format_strings, false)
        },
        ParseMethod::Format((format_strings, has_format)) => {
            (format_strings, has_format)
        }
    };

    println!("Format for file: {:?}\nNow parsing file...", format_strings);

    //Split overall string by newline characters
    let mut split_contents: Vec<String> = 
        file_contents.split("\n").filter(|x| x.len() > 0).map(|x| x.to_string()).collect();

    //Adjust file_contents if format string at start 
    if has_format {
        split_contents = split_contents.split_off(1);
    }
    //println!("{:?}", split_contents);

    //Checks if we have the bare minimum number of time stamps to split overall 
    //file into
    if split_contents.len() < 2 {
        eprintln!("More than two lines are needed in time stamp file");
        panic!();
    }

    let mut ret: Vec<Vec<String>> = Vec::new();

    //Split if the name is the first in the format
    if format_strings[0] == "name" {
        for line in split_contents {
            //Name of file is everything before ".mp3"
            let temp: Vec<String> = line.split(".mp3").into_iter()
                .map(|x| x.to_string()).collect();

            //Checks if the names have the extension on them
            if temp.len() < 2 {
                eprintln!("Missing file extension on time stamp file");
                panic!();
            }

            //Splits up line further: name temp[0], start and end time temp[1]
            let f: String = temp[0].clone()+".mp3";
            let rest: String = temp[1].clone();

            //Split start and end time into two strings
            let rest_split: Vec<String> = rest.trim_start().split(" ")
                .map(|x| x.to_string()).collect();

            //Checks if a time argument is missing
            if rest_split.len() < 2 {
                eprintln!("Missing arguments in the time stamp file: <filename> <start time> <end time>");
                panic!();
            } 

            let mut vec: Vec<String> = Vec::new();
            vec.push(f);
            vec.push(rest_split[0].clone());
            vec.push(rest_split[1].clone());

            ret.push(vec);
        }
    } else if format_strings[1] == "name" {
        for line in split_contents {
            let (first, second) = line.split_once(' ').unwrap();

            //Name of file is everything before ".mp3"
            let temp: Vec<String> = second.to_string().split(".mp3 ").into_iter()
                .map(|x| x.to_string()).collect();
            let name: String = temp[0].clone()+".mp3";

            //print!("{:?} \"{}\"\n", temp, first);
            let mut vec: Vec<String> = Vec::new();
            vec.push(first.to_string());
            vec.push(name);
            vec.push(temp[1].to_string());

            ret.push(vec);
        }
    } else if format_strings[2] == "name" {
        for line in split_contents {
            let (first, second) = line.split_once(' ').unwrap();
            let (second, name) = second.split_once(' ').unwrap();
            let mut vec: Vec<String> = Vec::new();
            vec.push(first.to_string());
            vec.push(second.to_string());
            vec.push(name.to_string());
            ret.push(vec);
        }
    }

    //println!("Final: {:?}", ret);

    FileContents { format: format_strings.clone(), data: ret }
}

/**
 * Runs all commands in a shell 
 *
 * Panics if the command fails
 */
fn run_ffmpeg_commands(command: String) {
    let output = Command::new("sh").arg("-c").arg(&command).output();
    match output.unwrap().status.code().unwrap() {
       0 => {
            println!("Conversion successful: \n\t{}", command);
       }, 
       code => {
           eprintln!("Failed to execute file conversion: Exit code {}", code);
           eprintln!("Failed command string: {}", command);
           panic!();
       }
    }
}

/**
 * Checks if a filename exist and returns the path if so
 */
fn validate_files(file: String) -> String {
    match fs::canonicalize(&file) {
        Ok(pathbuf) => {
            return format_args!("{:?}",pathbuf).to_string().replacen("\"","",2);
        }, 
        Err(_) => {
            eprintln!("Error: File {} does not exists", file);
            panic!();    
        }
    }
}

/**
 * Checks if the input file exists
 *
 * Returns a boolean of true if so
 */
fn check_input_file_existance() -> bool {
    let mut ret: bool = false;
    
    let filename: String = "./input.opus".to_string(); 
    let paths = fs::read_dir("./").unwrap();
    for path in paths {
        if path.unwrap().path().to_str().unwrap() == filename {
            ret = true;
        }
    }

    ret
}

fn download(link: String) {
    todo!();
}

fn split(input_file: String, timestamps_file: String) {
    let timestamp_path: String = timestamps_file;
    let mut inputmp3_path: String = input_file;
    let input_flag: bool = check_input_file_existance();

    //Replace input opus with default name
    if input_flag {
        inputmp3_path = "input.opus".to_string();
    }

    //Validate that the files exist and finds the path
    let full_inputmp3_path: String = validate_files(inputmp3_path);
    let full_timestamp_path: String = validate_files(timestamp_path);

    println!("timestamp_path: {}", full_timestamp_path);
    println!("inputmp3_path: {}", full_inputmp3_path);


    //Splits all lines of timestamp file into name, start time, and end time
    let split_contents: FileContents = separate_file_contents(full_timestamp_path);
    println!("Length of new_split_contents = {}", split_contents.data.len());

    //Constructs all commands for the ffmpeg conversions
    run_split_commands(split_contents, full_inputmp3_path);


    //Command:
    //Run commands for splitting into multiple files
    //ffmpeg -i BIG_FILE -acodec libmp3lame -ss START_TIME -to END_TIME LITTLE_FILE
    //ffmpeg -i input.opus -acodec libmp3lame -ss hh:mm:ss -to hh:mm:ss newname

}


fn main() {
    let args = Args::parse();

    match args.action {
        Action::Split { input_name, timestamps_name } => {
            println!("Here for split: {} {}", input_name, timestamps_name);
            split(input_name, timestamps_name);
        }, 
        Action::Download { link } => {
            println!("Here for download: {}", link);
            download(link);
        }
    }

}
