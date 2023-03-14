# Current planned features
## Feature 1 (Done): Download the file within the rust program

1. Ask for the YouTube link within the program. From there, we will call yt-dlp and download the file, rename it, and exit.


## Feature 2: Add file chooser

1. Pick the input file that will be split into multiple mp3 files
2. Pick the timestamps file that will be used to define the titles and the location in the input file 


## Feature 3 (Might currently work w/o standardization): Add support for different file types (yt-dlp may not give opus output)

1. This would go with feature 1. It would transform any different file type into an opus file to standardize it before splitting


## Feature 4 (Done): Add support for splitting the timestamps file differently

1. Makes it so the user can change what the format is within the timestamps file whether it is (start, end, filename) or (filename, start, end)

## Feature 5 (Done): Add proper command line parsing via rust lib [clap](https://docs.rs/clap/latest/clap/)
1. Added subcommands to the `cargo run` command
    - Subcommand *download* has a link flag
    - Subcommand *split* has an input and timestamps flag

## Feature 6: Add support for downloading playlists
1. Pick the YouTube public/unlisted playlist link you'd like to download
2. Run the *playlist* subcommand with the link or the part of the playlist url that has list={random string of letters and characters}
3. Edit the generated file with names 
4. Run the *rename* subcommand

## Feature 7 (Done): Add playlist and rename subcommands
1. Add subcommand *playlist* to main.rs
    - Give argument similar to *download*
2. Add subcommand *rename* to main.rs
    - Give argument of text rename file
    
## Feature 8: Add verbose option to each subcommand
1. Add flag *-v* to each command 
    - Show progress bar for downloading playlists
    - Show command output for *playlist* and *download* subcommands

## Feature 9 (Done): Add multithreading for ffmpeg command
1. Create argument for *split* subcommand 
    - Argument will take a number and will default to 5
2. Multithreading will run in batches of the argument number specified

Note: Accessing command line arguments of program go as follows:
```bash
cargo run -- (subcommand) (flags)
```
Ex:
```bash 
cargo run -- download -l xm3YgoEiEDc
```

