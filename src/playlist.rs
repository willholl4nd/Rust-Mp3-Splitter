use std::fs;
use std::process::{Command, Stdio};

//Basically a result enum
pub enum Download {
    Failed,
    Success {
        filenames: Vec<String>
    }
}

/**
 * Runs the yt-dlp download command 
 *
 * Params:
 * link - the link to the YouTube video
 *
 * Returns:
 * This is essentially a result enum
 */
pub fn download_run(link: String) -> Download {
    let command: String = format_args!("yt-dlp -ix {}", link).to_string();

    let output = Command::new("sh")
        .arg("-c")
        .arg(&command)
        .output();
    match output.as_ref().unwrap().status.code().unwrap() {
        0 => {
            println!("Download successful: \n\t{}", command);

            let stdout: Vec<u8> = output.as_ref().unwrap().stdout.clone();
            let filenames: Vec<String> = parse_filenames(stdout);

            Download::Success { filenames }
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
fn parse_filenames(command_output: Vec<u8>) -> Vec<String> {
    println!("{:?}", command_output);
    let output_stdout: String = String::from_utf8(command_output).unwrap();
    let first_split: Vec<&str> = output_stdout.split("[ExtractAudio] Destination: ").collect();
    let second_split: Vec<&str> = first_split[1].split("\n").collect();
    //second_split[0].to_string()

    Vec::<String>::new()
}

