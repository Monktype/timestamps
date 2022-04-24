# Discord Timestamp Generation Tool

This tool provides a small front-end to convert a time and a format into a Discord-friendly timestamp string.

In general, the numerical result in the output string is the UNIX epoch timestamp -- the number of non-leap seconds since 01 January 1970. However, these strings also allow a format value, which are generated with this tool.

Please feel free to open PRs with improvements! I tend to write lower-level tools, so this project is me learning small WASM frontends!

### Building
1. You'll need a current version of [Rust](https://www.rust-lang.org/) (non-nightly should be OK; rustup may be a useful addition for step #2), [Cargo](https://github.com/rust-lang/cargo), and [Trunk](https://github.com/thedodd/trunk) (to make things easier).

2. Ensure that you have the `wasm32-unknown-unknown` Rust target installed.

3. Remove the index\* files in the root of the codebase.

4. In code/, simply run `trunk build --release --public-url "timestamps/"` (if you're hosting this at a host:port/timestamps/ location, for example). Let dependencies download and compiling take place. The Yew crate and wasm targets in general have quite a few dependencies.

5. (Optionally) Copy the files in code/dist/ to the root of the codebase. You may need to update the paths in index.html to relative paths if you didn't use --public-url in #4.
