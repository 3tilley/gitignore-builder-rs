# gitignore-builder-rs
> Fetch and merge .gitignore files for your project

## Usage

The easiest way to interact with this project is through the REST service hosted by [shuttle.rs](https://www.shuttle.rs/). Alternatively you
can use the CLI locally - though it will still reach out to Github.

### Server

The app is hosted at https://gitig.shuttleapp.rs. There is a method at:

`GET /ignores?lang=python&lang=rust`

E.g.

`curl "https://gitig.shuttleapp.rs/ignores?lang=python&lang=rust"`

Note for Windows users:

Depending where curl picks its CA certs from the certificate may not be recognised. Your favourite HTTP client
should work though:

httpie

`https gitig.shuttleapp.rs/ignores lang==python lang==rust` 

xh

`xh https://gitig.shuttleapp.rs/ignores lang==python lang==rust`

Or while it's not recommended, you can also use curl's `--insecure` flag.
### CLI

Clone the repo and run:
`cargo run --bin cli -- --help`

In the same vein you can launch the server locally with:

`cargo run --bin server`

## Differences to other tools

_TODO_

## Notes

Originally inspired by https://github.com/colesnodgrass/gitignore-builder, though I've since seen other tools
