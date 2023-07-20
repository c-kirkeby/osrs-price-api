# OSRS Wiki Pricing Wrapper API

This is intended to be a wrapper API around the [RuneScape:Real-time Prices](https://oldschool.runescape.wiki/w/RuneScape:Real-time_Prices) API. The API should extend the upstream API with some caching, ability to add favourites, and defining recipes.

## Run

```sh
USER_AGENT="osrs-price-api - <discord/github username/email>" cargo run
```

## Alternative run with cargo watch

```sh
USER_AGENT="osrs-price-api - <discord/github username/email>" cargo watch -x run
```

## Build

```sh
USER_AGENT="osrs-price-api - <discord/github username/email>" cargo build
```

**Note:** the upstream API kindly requests that you make yourself contactable when using their API, so fill in `<discord/github username/email>` with appropriate contact details.
