timestamps = "timestamps.txt"
input = "input.mp3"
link = ""

all:
	cargo run --release ${timestamps} ${input}

download:
	yt-dlp -ix ${link}

test:
	cargo test -- --test-threads=10
