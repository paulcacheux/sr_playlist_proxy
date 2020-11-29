Media/Playlist Proxy - Technical Test
=====================================

*This project was built for a recruitment technical test*

This project is a media proxy between a media player (VLC for example) and the actual media server.

## Absolute URLs rewriting

Absolute URLs would normally exit the proxy simply because the media player would connect directly to the new base URL.
To manage this, the proxy will rewrite the playlist URLs with relative ones, containing the original URL as a base64 path segment, following a trigger segment that should be unique enough (`relative_to_absolute_proxy_path` by default).

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