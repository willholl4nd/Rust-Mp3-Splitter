//use std::io::Error;
//use std::io;
//use std::path::PathBuf;
use clap::Parser;

mod split;
mod download;

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


fn download_command(link: String) {
    let res: download::Download = download::download_run(link);
    let file: String = match res {
        download::Download::Failed => {
            panic!();
        },
        download::Download::Success { filename } => {
            filename
        }
    };
    
    let new_file: String = download::rename_download(file); 
}

fn split_command(input_file: String, timestamps_file: String) {
    let timestamp_path: String = timestamps_file;
    let mut inputmp3_path: String = input_file;
    let input_flag: bool = split::check_input_file_existance();

    //Replace input opus with default name
    if input_flag {
        inputmp3_path = "input.opus".to_string();
    }

    //Validate that the files exist and finds the path
    let full_inputmp3_path: String = split::validate_files(inputmp3_path);
    let full_timestamp_path: String = split::validate_files(timestamp_path);

    println!("timestamp_path: {}", full_timestamp_path);
    println!("inputmp3_path: {}", full_inputmp3_path);


    //Splits all lines of timestamp file into name, start time, and end time
    let split_contents: split::FileContents = split::separate_file_contents(full_timestamp_path);
    println!("Length of new_split_contents = {}", split_contents.data.len());

    //Constructs all commands for the ffmpeg conversions
    split::run_split_commands(split_contents, full_inputmp3_path);


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
            split_command(input_name, timestamps_name);
        }, 
        Action::Download { link } => {
            println!("Here for download: {}", link);
            download_command(link);
        }
    }

}
