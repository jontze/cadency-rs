# Cadency-rs

A discord bot written in **Rust** with the help of [serenity](https://github.com/serenity-rs/serenity) and [songbird](https://github.com/serenity-rs/songbird).
Initially this was intended to be a rust clone of [my typescript discord bot](https://github.com/jontze/Cadency) but currently **Cadency-rs** offers more features and is more up to date as it's already using the discord **slash commands**.

## Installation

The recommended way to install and run **Cadency-rs** is with Docker, as the Docker image installs several libraries and binaries that are required to use or build the bot.

1. Pull the image from the **ghcr.io** registry (`main` pulls the latest stable build, `develop` latest development build, `vX.X.X` pulls a fixed version)

```sh
docker pull ghcr.io/jontze/cadency_rs:main
```

2. Start a container and pass your discord bot token to the container by setting the `DISCORD_TOKEN` environment variable and optional specify the log level e.g. `RUST_LOG="cadency=info"`.

3. Invite the bot to your discord server, discord offers great [documentation](https://discord.com/developers/docs/getting-started) on how to do this.

## Example docker setup:  
First run (replace the TOKEN_HERE with your token):
```sh
docker run --name cadency_rs -e DISCORD_TOKEN=TOKEN_HERE ghcr.io/jontze/cadency_rs:main
```
This creates a new container named `cadency_rs`. After the first run you can use this container like so:
```sh
docker start -a cadency_rs
```
When you execute the command nothing seems to happen, but this is not the case. When in doubt, check if the bot is online in Discord.  
## Features

- **Play songs** from Youtube using a url or a search phrase
- Add complete Youtube **playlists** to the song queue
- **Maintain a queue** of songs which you can pause, skip and resume.
- Look for a phrase or word in the **Urban Dictionary**
- Let **Cadency-rs** say something **inspiring**
- Slap someone with a **trout**
- Calculate a number in the **fibonacci** sequence
- And play a classic game of **ping pong**

## Contributing

1. Install the latest stable rust toolchaun: https://www.rust-lang.org/tools/install
2. Intall [yt-dlp](https://github.com/yt-dlp/yt-dlp#installation)
3. Clone the repository
4. Install [ffmpeg](https://ffmpeg.org/download.html). Install the [yt-dlp version](https://github.com/yt-dlp/FFmpeg-Builds), as this version contains bugfixes and is used in the Docker file
4. Create a discord bot so you can run the bot local on your computer
5. Set the environment variables that are listed as example in [.env.example](./.env.example)
6. You should be able to run `cargo test` and `cargo run` successfully

## Note
You may have to install the **development** versions of Opus (opus-dev / opus-devel) and/or cmake (check your distribution for specific name). The best way to test if you have those installed, is to run `cargo test` or `cargo run` and see if any errors mentions either package.