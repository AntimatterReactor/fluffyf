# Todo List

## The Library

- [ ] frustrating: implement methods and traits for API objects
- [x] coagulate all use statements
- [ ] implement Stream items for Pool and Post
- [x] original idea was correct: rename to remove plurality (pools => pool)
- [x] remove `Object` suffix from the objects
- [ ] create Search types per object and traits for those Search types
- [ ] create `Client`
    - [ ] custom useragent
    - [ ] custom auth
    - [ ] high level methods
- [ ] make async everything proper

### Documentation

- [ ] document thoroughly:
    - [ ] posts
    - [ ] pools
    - [ ] connect and connect::methods
    - [ ] favorites
    - [ ] search
    - [ ] tags
    - [ ] notes
    - [ ] post\_flags

## The Binaries

- [ ] complete fluffyget
    - [ ] expand to queries for other stuff than posts:
        - [ ] pools
        - [ ] favorites
        - [ ] tags and tag aliases
        - [ ] post\_flags
- [ ] complete fluffypost
    - [ ] posts
    - [ ] favorites
    - [ ] vote
    - [ ] pools
    - [ ] post\_flags
    - [ ] notes?
- [ ] create fluffymod

## Tests

- [ ] unit tests
- [ ] integration tests

## Rust Itself

- [x] rust 1.75 stabilized async fn traits in traits:
    - [x] remove async\_traits dependency
    - [x] add minimum supported rust version (MSRV) to 1.75
        - [x] document/explain why the MSRV is so high
    - [x] install rustup (forgot to do this)

## Secondaries

- [x] after some of the above: release to public + crates.io
- [x] sort this todo list
- [ ] add changelog

### Later Endeavours

- [ ] painful: switch from reqwest to hyper + rustls/native-tls
- [ ] extremely complex: add wasm support
