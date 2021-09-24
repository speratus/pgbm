# pgbm
pgbm is a tiny wrapper around the command to boot PostgreSQL as installed by Linuxbrew.

Linuxbrew is the Linux variant of Homebrew, and as a result, installing PostgreSQL
from Linuxbrew does not have some of the handy features that come built in to
Homebrew. 

This binary is intended to add a simple way to boot a Linuxbrew installation of
PostgreSQL.

In a normal Linuxbrew installation of PostgreSQL, one must enter the following command 
to boot Postgres:
`pg_ctl -D /home/linuxbrew/.linuxbrew/var/postgres start`.

This binary adds a simple command to do the same thing:
`pgbm start` or `pgbm u`.

Additionally, the commands `pgm stop` and `pgbm d` shutdown the PostgreSQL server
by executing the command:
`pg_ctl -D /home/linuxbrew/.linuxbrew/var/postgres stop`.

## Why not use bash?
There are two reason I chose to write this binary in Rust instead of bash:
1. I already know bash reasonably well, and I wanted to use this as an opportunity
    to help familiarize myself with Rust.
2. It is easier for me to install a Rust binary than a bash script on my linux systems. 
   Installing a bash script requires manually copying it to the correct location
   and then making sure it has the correct permissions. Installing a Rust binary
   requires executing a simple `cargo` command.

## Installation
There are a couple of easy ways to install pgm at the moment:

1. Install it straight from github: `cargo install --git https://github.com/speratus/pgbm.git`
2. Clone the repository and then install it from the local file system:
   `git clone https://github.com/speratus/pgbm.git && cd $_`, and then run
   `cargo install --path .`.

In the future, I may decide to add this library to [crates.io](https://crates.io),
but for the time being, these are the two best ways to get pgbm.