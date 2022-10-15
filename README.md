# Cadency-rs

A discord bot written in **Rust** with the help of [serenity](https://github.com/serenity-rs/serenity) and [songbird](https://github.com/serenity-rs/songbird).
Initially this was intended to be a rust clone of [my typescript discord bot](https://github.com/jontze/Cadency) but currently **Cadency-rs** offers more features and is more up to date as it's already using the discord **slash commands**.

## Installation

The recommended way to install and run **Cadency-rs** is with docker, as the docker image installs several libraries and binaries that are required to use or build the bot.

1. Pull the image from the **ghcr.io** registry (`main` pulls the latest stable build, `develop` latest development build or `vX.X.X` pulls a fix version)

```sh
docker pull ghcr.io/jontze/cadency_rs:main
```

2. Start a container and pass your discord bot token to the container by setting the `DISCORD_TOKEN` environment variable and optional specify the log level e.g. `RUST_LOG="cadency=info"`.
3. Invite the bot to your discord server, discord offers a great [documentation](https://discord.com/developers/docs/getting-started) how to do this

## Example docker commands:
First run. replace TOKEN_HERE with your bot token:
```sh
docker run --name cadency_rs -e DISCORD_TOKEN=TOKEN_HERE ghcr.io/jontze/cadency_rs:main
```

Subsequent runs:
```sh
docker start -a cadency_rs
```

Do remember to remove the container and rerun the first command after an update.
## Features

- **Play songs** from youtube by url and search phrase
- Add complete youtube **playlists** to the song queue
- **Maintain a queue** of songs where you can pause, skip and resume songs in the queue
- Look for a phrase / word in the **urban dictionary**
- Let **Cadency-rs** say something **inspiring**
- Slap someone with a **trout**
- Calculate a number in the **fibonacci** order
- And the classic **ping pong** game

## Contributing

1. Install the latest stable rust toolchaun: https://www.rust-lang.org/tools/install
2. Intall [yt-dlp](https://github.com/yt-dlp/yt-dlp#installation)
3. Clone the repository
4. Install [ffmpeg](https://ffmpeg.org/). It is reccomended you install the [yt-dlp build](https://github.com/yt-dlp/FFmpeg-Builds), as this is used in the Docker image as well and includes additional bug-fixes.
5. Create a discord bot so you can run the bot local on your computer
5. Set the environment variables that are listed as example in [.env.example](./.env.example)
6. You should be able to run `cargo test` and `cargo run` successfully
