# Cadency-rs

A discord bot that's using **slash commands** written in **Rust** with the help of [serenity](https://github.com/serenity-rs/serenity) and [songbird](https://github.com/serenity-rs/songbird).

> WARNING: Cadency is not production ready! It might eat your laundry.

## Features

- **Play songs** from Youtube using a url or a search phrase
- Add complete Youtube **playlists** to the song queue
- **Maintain a queue** of songs which you can **pause**, **skip**, **resume**, **loop**, ...
- Look something up in the **Urban Dictionary**
- Let **Cadency-rs** say something **inspiring**
- Slap someone with a **trout** _(old IRC gag)_
- Calculate a number in the **fibonacci** sequence
- Roll a **dice** e.g. `2d6+3`
- And play a classic game of **ping pong**

## Installation

The recommended way to install and run **Cadency-rs** is with Docker, as the Docker image installs several libraries and executables that are required to use the bot.

1. Pull the image from the **ghcr.io** registry (`main` pulls the latest stable build, `develop` latest development build, `vX.X.X` pulls a fixed version)

```sh
docker pull ghcr.io/jontze/cadency_rs:main
```

2. Start a container and pass your discord bot token to the container by setting the `DISCORD_TOKEN` environment variable and optional specify the log level e.g. `RUST_LOG="cadency=debug"`.
3. Invite the bot to your discord server, discord offers great [documentation](https://discord.com/developers/docs/getting-started) on how to do this.

## Permissions

Cadency requires the `bot` **scope** and several permissions on a server to work properly. Therefore, ensure to set these in the developer portal during the creation of the invite link:

- `Send Messages`
- `Connect`
- `Speak`

## Example docker setup

Replace the TOKEN_HERE with your token:

```sh
docker run --name cadency_rs -d -it --rm -e DISCORD_TOKEN=DISCORD_TOKEN_HERE ghcr.io/jontze/cadency_rs:main
```

This runs the container in detatched mode (background) and removes it when it stops running. This makes sure that you are always using an up-to-date version of cadency-rs.  
To stop the container, execute the following command and substitute ID for the id that was returned when running the docker run command:

```sh
docker stop ID
```

## Environment Variables

| Variable Name                 | Default Value  | Description                                               | Required |
| ----------------------------- | -------------- | --------------------------------------------------------- | -------- |
| `DISCORD_TOKEN`               |                | Discord bot token                                         | `true`   |
| `RUST_LOG`                    | `cadency=info` | Log Level                                                 | `false`  |
| `CADENCY_PLAYLIST_SONG_LIMIT` | `30`           | Maximum amount of songs that can be added from a playlist | `false`  |
| `CADENCY_SONG_LENGTH_LIMIT`   | `600.00`       | Maximum allowed song length in seconds                    | `false`  |

## Contributing

1. Install the latest stable [rust toolchain](https://www.rust-lang.org/tools/install)
2. Intall [yt-dlp](https://github.com/yt-dlp/yt-dlp#installation)
3. Clone the repository
4. Create a [discord bot in the discord developer portal](https://discord.com/developers/docs/getting-started) so you can run the bot local on your computer
5. Set the environment variables in your shell that are listed as example in [.env.example](./.env.example)
6. You should be able to run `cargo test` and `cargo run` successfully

## Note

You may have to install the **development** versions of Opus (opus-dev / opus-devel) and/or cmake (check your distribution for specific name). The best way to test if you have those installed, is to run `cargo test` or `cargo run` and see if any errors mentions either package.
