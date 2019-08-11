# Cosmic
[![Travis](https://img.shields.io/travis/Sreyas-Sreelal/Cosmic.svg)](https://travis-ci.org/Sreyas-Sreelal/Cosmic)
[![GitHub issues](https://img.shields.io/github/issues/Sreyas-Sreelal/Cosmic.svg)]() [![GitHub pull requests](https://img.shields.io/github/issues-pr-raw/sreyas-sreelal/Cosmic.svg)]() [![license](https://img.shields.io/github/license/sreyas-sreelal/Cosmic.svg)]()
![alt text](https://github.com/Sreyas-Sreelal/Cosmic/blob/master/images/cosmic_banner.png?raw=true "Cosmic")
<p align="center">
Logo by <a href="https://github.com/ssnjrthegr8">ssnjrthegr8</a>
</p>

>Cosmic is a mutli purpose discord bot written in rust.The bot can help you in various tasks.Treat him nice and he will treat you the same way.Basic NLP capability is achieved with the help of [eliza-rs](https://github.com/arosspope/eliza-rs)

## Features
1. Basic AI Chat system
2. Searches for lyrics of provided song
3. Music player
4. Searches and finds torrents as per the need
5. Searches wiki 
6. Random meme generator
and more...

## Building
1. Clone this repository
    ```
    git clone https://www.github.com/sreyas-sreelal/Cosmic.git
    ```
2. Use cargo to build
    ```
    cargo build --release
    ```
3. Add `COSMIC_TOKEN` to environment variables with your token as value

4. Run
    ```
    ./cosmic
    ```
## Notes
Cosmic requires ffmpeg and youtubedl for audio processing.So install them before using the bot.
