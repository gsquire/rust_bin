# `rust_bin` API

## `GET /`
Return this page.

## `GET /ip`
Return the origin IP of the user.

## `GET /user-agent`
Return the `User-Agent` header or 400 if it was not found.

## `GET /stream-bytes/<n>`
Stream `n` number of bytes in chunks.

## `GET /status/<s>`
Return the specified HTTP status `s` or 400 if it is invalid. 
