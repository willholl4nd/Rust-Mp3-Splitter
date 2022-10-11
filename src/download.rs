use std::fs;
use std::process::Command;

//Basically a result enum
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
 * link - the link to the YouTube video
 *
 * Returns:
 * This is essentially a result enum
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
            println!("{}", filename);

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
 * Create the command for downloading the YouTube video
 *
 * Params:
 * link - the link to the YouTube video
 */
fn craft_command(link: &String) -> String {
    format_args!("yt-dlp -ix {}", link).to_string()
}

/**
 * Rename the downloaded file to be the video id
 *
 * Returns:
 * The name of the new file
 */
pub fn rename_download(file: String) -> String {
    let (id, ext): (String, String) = find_video_id(file.clone());
    let short: String = format_args!("{}.{}", id, ext).to_string();
    
    //Handle the creation of the directory with the name of the video id
    match fs::create_dir(id.clone()) {
        Ok(_) => {
            println!("Successfully created directory");
        }, 
        Err(_) => {
            eprintln!("Failed to create directory");
            panic!();
        }
    };

    //Handle the creation of the timestamps file 
    let timestamps: String = format_args!("timestamps-{}.txt", id).to_string();
    match fs::write(timestamps, "start end name") {
        Ok(_) => {
            println!("Successfully created timestamps file");
        },
        Err(_) => {
            eprintln!("Failed to created timestamps file");
            panic!();
        }
    };

    //Handle the renaming of the audio file
    match fs::rename(file, short.clone()) {
        Ok(_) => {
            println!("File successfully renamed to {}", short);
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
 *
 * Returns:
 * a tuple of strings with the first element being the video id, and 
 * the second element being the extension of the current file
 */
fn find_video_id(mut file: String) -> (String, String) {
    let index_of_lbr = file.rfind('[').unwrap() + 1;
    let mut short: String = file.split_off(index_of_lbr); //Contains the id to the end of the file
    let index_of_rbr = short.rfind(']').unwrap();
    short.remove(index_of_rbr);
    let final_split: Vec<&str> = short.split(".").collect();

    (final_split[0].to_string(), final_split[1].to_string())
}
