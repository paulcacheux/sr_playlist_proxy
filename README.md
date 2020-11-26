Media/Playlist Proxy - Technical Test
=====================================

*This project was built for a recruitment technical test*

This project is a media proxy between a media player (VLC for example) and the actual media server.

## Dependencies
- Rust

## Usage
```sh
cargo run --release -- BASE_URL
```

## Example
To stream the following playlist: https://bitdash-a.akamaihd.net/content/MI201109210084_1/m3u8s/f08e80da-bf1d-4e3d-8899-f0f6155f6efa.m3u8

you would run the proxy with
```sh
cargo run --release -- https://bitdash-a.akamaihd.net/
```

and use
```
http://localhost:3000/content/MI201109210084_1/m3u8s/f08e80da-bf1d-4e3d-8899-f0f6155f6efa.m3u8
```
as a media link in your media player (VLC for example).