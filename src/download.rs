use std::fs::{self, File};
use std::io::prelude::*;
use std::process::Command;

pub enum Download {
    Failed,
    Success {
        filename: String
    }
}

/**
 * Runs the yt-dlp download command 
 *
 * Params:
 * link - the link to the youtube video
 */
pub fn download_run(link: String) -> Download {
    let command: String = craft_command(&link);

    let output = Command::new("sh").arg("-c").arg(&command).output();
    //println!("Output from command:\n{:?}", output);
    let stdout: Vec<u8> = output.as_ref().unwrap().stdout.clone();
    match output.unwrap().status.code().unwrap() {
        0 => {
            println!("Download successful: \n\t{}", command);

            let filename: String = parse_filename(stdout);

            Download::Success { filename }
        }, 
        code => {
            eprintln!("Failed to execute file download: Exit code {}", code);
            eprintln!("Failed command string: {}", command);

            Download::Failed
        }
    }
}

/**
 * Parse the filename from the command output
 */
fn parse_filename(command_output: Vec<u8>) -> String {
    let output_stdout: String = String::from_utf8(command_output).unwrap();
    let first_split: Vec<&str> = output_stdout.split("[ExtractAudio] Destination: ").collect();
    let second_split: Vec<&str> = first_split[1].split("\n").collect();
    second_split[0].to_string()
}

/**
 * Create the command for downloading the youtube video
 *
 * Params:
 * link - the link to the youtube video
 */
fn craft_command(link: &String) -> String {
    format_args!("yt-dlp -ix {}", link).to_string()
}

/**
 * Rename the downloaded file to be the video id
 */
pub fn rename_download(file: String) -> String {
    let short: String = find_video_id(file.clone());
    match fs::rename(file, short.clone()) {
        Ok(_) => {
            short
        },
        Err(_) => {
            eprintln!("Failed to rename file to video id");
            panic!();
        }
    }
}

/**
 * Takes the existing default file name and find the video id from it
 */
fn find_video_id(mut file: String) -> String {
    let index_of_lbr = file.rfind('[').unwrap() + 1;
    let mut short: String = file.split_off(index_of_lbr); //Contains the id to the end of the file
    let index_of_rbr = short.rfind(']').unwrap();
    short.remove(index_of_rbr);

    short
}
