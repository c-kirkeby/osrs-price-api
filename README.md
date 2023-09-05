# OSRS Wiki Pricing Wrapper API

This is intended to be a wrapper API around the [RuneScape:Real-time Prices](https://oldschool.runescape.wiki/w/RuneScape:Real-time_Prices) API. The API should extend the upstream API with some caching, ability to add favourites, and defining recipes.

## Get started

1. Clone the project
```sh
git@github.com:c-kirkeby/osrs-price-api.git
```
2. Install
```sh
cargo install
```
3. Copy and modify the .env file:
```sh
cp .env.example .env`
```

**Note:** the upstream API kindly requests that you make yourself contactable when using their API, so fill in `<discord/github username/email>` with appropriate contact details.

## Run

```sh
cargo run
```

By default it's set to run on port 3400, but this can be easily changed by modifying the `PORT` environment variable.

## Alternative run with cargo watch

```sh
cargo watch -x run
```

## Build

```sh
cargo build
```


