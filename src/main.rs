use clap::Parser;
use num_cpus;

mod split;
mod download;
mod rename;
mod playlist;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    action: Action
}

#[derive(clap::Subcommand, Debug)]
enum Action {
    Download {
        //YouTube link to download
        #[arg(short, long)]
        link: String
    }, 
    Split {
        //The input file name
        #[arg(short, long, default_value = "input.opus")]
        input_name: String,

        //The timestamp file name
        #[arg(short, long, default_value = "timestamps.txt")]
        timestamps_name: String,
            
        //The thread count for multithreading
        #[arg(short = 'x', long, default_value_t = get_cpu_default() )]
        thread_count: usize
    },
    Playlist {
        //YouTube link to download
        #[arg(short, long)]
        link: String
    },
    Rename {
        #[arg(short, long, default_value = "rename.txt")]
        rename_file: String
    }
}

fn get_cpu_default() -> usize {
    let num = num_cpus::get();
    let rem = num % 4;
    if rem != 0 {
        (num - rem) / 4 + 1            
    } else {
        num / 4
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
    
    let _new_file: String = download::rename_download(file); 
}

fn split_command(input_file: String, timestamps_file: String, thread_count: usize) {
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
    split::run_split_commands(split_contents, full_inputmp3_path, thread_count);
}

fn rename_command(rename_file: String) {


}

fn playlist_command(link: String) {
    let res: playlist::Download = playlist::download_run(link);
    let files: Vec<String> = match res {
        playlist::Download::Failed => {
            panic!();
        },
        playlist::Download::Success { filenames } => {
            filenames
        }
    };
    
    //let _new_file: String = download::rename_download(file); 

}

fn main() {
    let args = Args::parse();

    match args.action {
        Action::Split { input_name, timestamps_name, thread_count } => {
            println!("Splitting {} with timestamps {} using {} threads", input_name, timestamps_name, thread_count); 
            split_command(input_name, timestamps_name, thread_count);
        }, 
        Action::Download { link } => {
            println!("Here for download: {}", link);
            download_command(link);
        },
        Action::Rename { rename_file } => {
            
        },
        Action::Playlist { link } => {
            playlist_command(link)
        }
    }

}
