timestamps = "timestamps.txt"
input = "input.opus"
link = ""

all:
	cargo run --release ${timestamps} ${input}

download:
	yt-dlp -ix ${link}

test:
	cargo test -- --test-threads=10
