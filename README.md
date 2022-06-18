# I guess just stuff I want to implement

## Current planned features
### Feature 1: Download the file within the rust program

1. Ask for the YouTube link within the program. From there, we will\
call yt-dlp and download the file, rename it, and exit.


### Feature 2: Add file chooser

1. Pick the input file that will be split into multiple mp3 files
2. Pick the timestamps file that will be used to define the titles\
and the location in the input file 


### Feature 3: Add support for different file types (yt-dlp may not give opus output)

1. This would go with feature 1. It would transform any different file type into\
an opus file to standardize it before splitting


### Feature 4: Add support for splitting the timestamps file differently

1. Makes it so the user can change what the format is within the timestamps file\
whether it is (start, end, filename) or (filename, start, end)


## Current roadmap

1. Feature 1 and 3 will be in the same rust script
2. The user would then add file name, start, end in the timestamps file 
3. Feature 2 and 4 would be merged with current main.rs and run to split the file
