<br />
<div align="center">
<h3 align="center">Rusty-Key</h3>

  <p align="center">
    A simple key/value store</a>
    <br />
    <br />
    <br />
    ·
    <a href="https://github.com/kjoedicker/rusty-key/issues">Report Issue</a>
    ·
    <a href="https://github.com/kjoedicker/rusty-key/issues">Request Feature</a>
  </p>
</div>

<!-- ABOUT THE PROJECT -->
## About The Project

`rusty-key` leverages `actix_web` for communicating requests to the native underlying  <a href="https://doc.rust-lang.org/std/collections/struct.HashMap.html">HashMap</a> that hosts everything. Data is persisted by default with a AOF system that helps keep and rebuild an event log of the data as it populates.

Like a key, this Rust based Key/Value store that aims to be simple and efficient, even if its a little rough around the edges.

## Usage

```
#  Returns `201` indicating the provided key was assigned the provided value
curl --location --request PUT 'http://localhost:8080/someKey' \
--header 'Content-Type: application/json' \
--data-raw '{
    "value": "someValue"
}'

# Returns `200` and the associated value or `404` if the key does not exist.
curl --location --request GET 'http://localhost:8080/1'

# Return `200` indicating the key was deleted or `404` if the key does not exist
curl --location --request DELETE 'http://localhost:8080/someKey'
```

[![forthebadge](https://forthebadge.com/images/badges/made-with-rust.svg)](https://forthebadge.com)