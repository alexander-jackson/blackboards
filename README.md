# Sessions

Sessions is a basic website for booking training times for members of Warwick
Barbell. It was used initially for taster sessions, where the club would run
short introduction to lifting events and people who were interested could book
a space.

Due to social distancing measures, we needed to be able to cap the number of
signups, and ensure that people were signing up with valid emails. We also only
wanted prospective members to come to a single session, so that we would have
as much space as possible.

## Dependencies

Sessions runs on the nightly version of Rust and requires `sqlite3` to be
installed currently.

Rust can be installed from `https://www.rust-lang.org/learn/get-started`, at
which point you can run `rustup default nightly` to get the latest nightly
compiler.

## Usage

Setting up the website locally is designed to be quite simple, first clone the
repository and enter the directory:

```bash
git clone git@github.com:alexander-jackson/sessions.git
cd sessions
```

Then set up the database and run the project:

```bash
make database
cargo run
```

You should then be able to go to `http://localhost:8000/sessions` to see the
website.
