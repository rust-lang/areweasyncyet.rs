# Are we async yet

[![Build Status](https://travis-ci.org/rust-lang/areweasyncyet.rs.svg?branch=master)](https://travis-ci.org/rust-lang/areweasyncyet.rs)

areweasyncyet.rs is a website for tracking development progress of
`async`/`await` syntax of Rust programming language in
the language itself as well as its ecosystem.

It's implemented as a static page generator,
and deployed to GitHub Pages via Travis.

## Building

To build the site locally,
you would need a GitHub personal access token for
fetching data from GitHub.
You can refer to [this article](https://help.github.com/articles/creating-a-personal-access-token-for-the-command-line/) for how to create such token.

Once you get the token,
put it into `.env` file like this:
```
GITHUB_TOKEN={your token}
```
and then execute `cargo run`.

You may also want to enable logs by adding
```
RUST_LOG=areweasyncyet=debug
```
to the `.env` file.

From there,
the generated HTML will be in the `_site` directory.
You can use any web server to check it out in your browser:
```
cd _site
python3 -m http.server
```

## Development

The Rust code handles issue data from GitHub.
Content mainly resides in `data.yml` and `templates` directory.

After the first execution,
fetched data will be stored in `cache.json` file in the current directory
to avoid repeatedly fetching data when updating `data.yml` and `templates`.
If the latest data from GitHub is needed,
simply remove the `cache.json` file.
