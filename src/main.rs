use std::fs::{self, File};
//use std::path::PathBuf;
use std::env;
//use std::io;
//use std::io::Error;
use std::io::prelude::*;
use std::process::Command;



fn construct_command(input_path: String, filename: String,
                     beginning: String, end: String) -> String {
    //ffmpeg -i inputmp3 -acodec copy -ss hh:mm:ss -to hh:mm:ss newname
    let string_command: String = 
        format_args!("ffmpeg -i {} -acodec copy -ss {} -to {} \"{}\" -y",
                     input_path, beginning, end, filename).to_string();

    return string_command;
}

#[warn(dead_code)]
fn print_directory_contents() {
    for i in fs::read_dir(".").unwrap() {
        println!("Files in dir: {}", i.unwrap().path().display());
    }

}

fn separate_file_contents(file_path: String) -> Vec<(String, String, String)> {
    let file_res = File::open(&file_path);
    
    let mut file = match file_res {
        Ok(file_opened) => {
            file_opened
        },
        Err(error) => {
            println!("Error opening file: {}", error);
            println!("File path {:?}", file_path);
            panic!()
        }
    };

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();


    let split_contents: Vec<String> = 
        contents.split("\n").filter(|x| x.len() > 0).map(|x| x.to_string()).collect();

    if split_contents.len() < 2 {
        eprintln!("More than two lines are needed in time stamp file");
        panic!();
    }
    let mut new_split_contents: Vec<(String, String, String)> = Vec::new(); 

    for line in split_contents {
        let temp: Vec<String> = line.split(".mp3").into_iter()
            .map(|x| x.to_string()).collect();

        if temp.len() < 2 {
            eprintln!("Missing file extension on time stamp file");
            panic!();
        }

        let f: String = temp[0].clone()+".mp3";
        let rest: String = temp[1].clone();
        let rest_split: Vec<String> = rest.trim_start().split(" ")
            .map(|x| x.to_string()).collect();

        if rest_split.len() < 2 {
            eprintln!("Missing arguments in the time stamp file: <filename> <start time> <end time>");
            panic!();
        } 

        let tup: (String, String, String) = 
            (f, rest_split[0].clone(), rest_split[1].clone());

        new_split_contents.push(tup);
    }
    
    return new_split_contents;
}

fn convert_overall_file(mut path: String) {
    let newpath = path.as_mut_str().to_string();
    let command: String = 
        format_args!("ffmpeg -i {} -acodec libmp3lame input.mp3 -y", newpath).to_string();

    run_ffmpeg_commands(command);
}

fn run_ffmpeg_commands(command: String) {
    let output = Command::new("sh").arg("-c").arg(&command).output();
    match output.unwrap().status.code().unwrap() {
       0 => {
            println!("Conversion successful: \n\t{}", command);
       }, 
       code => {
           println!("Failed to execute file conversion: Exit code {}", code);
           println!("Failed command string: {}", command);
           panic!();
       }
    }
}

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

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Usage: <time stamp file> <input mp3 file>\n\
        Please put files in outer directory with Makefile.");
        panic!();
    }

    let timestamp_path: String = args[1].to_string();
    let inputmp3_path: String = args[2].to_string();

    let full_inputmp3_path: String = validate_files(inputmp3_path);
    let full_timestamp_path: String = validate_files(timestamp_path);

    println!("timestamp_path: {}", full_timestamp_path);
    println!("inputmp3_path: {}", full_inputmp3_path);


    let split_contents = separate_file_contents(full_timestamp_path);
    println!("Length of new_split_contents = {}", split_contents.len());

    convert_overall_file(full_inputmp3_path);

    let full_input_path: String = validate_files("input.mp3".to_string());
    let convert_commands: Vec<String> = split_contents.iter()
        .map(|tuple| 
            construct_command(full_input_path.clone(),tuple.0.clone(),
            tuple.1.clone(),tuple.2.clone()))
        .collect();

    //Run all split commands
    for string in convert_commands {
        run_ffmpeg_commands(string);
    }



    //Command:
    //Run before splitting into multiple files
    //ffmpeg -i inputmp3 -acodec libmp3lame input.mp3
    //
    //Run commands for splitting into multiple files
    //ffmpeg -i BIG_FILE -acodec copy -ss START_TIME -to END_TIME LITTLE_FILE
    //ffmpeg -i inputmp3 -acodec copy -ss hh:mm:ss -to hh:mm:ss newname
}
