# unofficial-spotify-api

SpotifyをスクレイピングしてAPIで提供していない情報を取得するやつ

```toml:Cargo.toml
[dependencies.unofficial-spotify-api]
git = "https://github.com/ekuinox/unofficial-spotify-api-rs"
```

## cli

[cargo-make](https://github.com/sagiegurari/cargo-make)を使って実行する

```console
$ git clone https://github.com/ekuinox/unofficial-spotify-api-rs
$ cd unofficial-spotify-api-rs
$ makers cli
```

楽曲クレジットを取得するには`credit`サブコマンドに楽曲のURLを渡して実行する

```console
$ makers cli credits https://open.spotify.com/track/1v8QHnM1K1wa6qg3DuL2SO
Track: Elbow Grease
- Credit Subrole: Performers
  + Ray Volpe ... main artist
  + Soltan ... main artist
- Credit Subrole: Writers
  + Raymond Volpe ... author
  + Soltan ... author
```

インストールするには`cargo install`が使える

```console
$ cargo install unofficial-spotify-cli --git https://github.com/ekuinox/unofficial-spotify-api-rs
```
