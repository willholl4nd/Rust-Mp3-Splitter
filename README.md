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
    - Subcommand `download` has a link flag
    - Subcommand `split` has an input and timestamps flag

Note: Accessing command line arguments of program go as follows:
```bash
cargo run -- (subcommand) (flags)
```

