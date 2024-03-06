# DBCop

## Usage

1.  Clone it.
```
    git clone git@gitlab.math.univ-paris-diderot.fr:ranadeep/dbcop.git
```

2.  Compile and install using `cargo` and run.
    Make sure `~/.cargo/bin` is in your system path.
```
    cd dbcop
    dbcop install --path .
    dbcop --help
```
---

There are a few `docker-compose` files in `docker` directory to create docker cluster.

The workflow goes like this,

1. Generate a bunch of histories to execute on a database.
2. Execute those histories on a database using provided `traits`. (see in `examples`).
3. Verify the executed histories for `--cc`(causal consistency), `--si`(snapshot isolation), `--ser`(serialization).  

---

## Build and Run on Ubuntu 22.04

```bash
# 0. clone this repo and init sub modules

# 1. install dependencies, including rust, cargo >= 1.70.0, libssl-dev, docker-compose

# 2. build and start docker service of postgres
docker-compose up -d -f docker/postgres/docker-compose.yml
# the above command could not be parsed correctly, so the below command was used instead
# (-f means --file, -d is an option for up)
sudo docker-compose -f docker/postgres/docker-compose.yml up -d

# 3. build dbcop
cargo build --release
# There's an inherent bug in Cargo.toml, that is package funty@1.2.0 was not supported any more, a solution was found in https://stackoverflow.com/questions/74556708/internal-dependency-issue-in-rust, and Cargo.toml was updated.
```

Dbcop is used to generate histories. For example:

```bash
# generated histories are stored in ./hist
# see 'dbcop genrate --help' and 'dbcop run --help'
cd dbcop
# generate a gen constrainted by UniqueValue
./target/release/dbcop generate -d ./gen -e 2 -n 10 -t 3 -v 2 # arg --key-distrib [uniform | zipf | hotspot] is acceptable, default uniform
# generate a gen in which values may repeat
./target/release/dbcop generate -d ./gen -e 2 -n 8 -t 4 -v 3 -r 0.5

./target/release/dbcop run -d ./gen --db postgres-ser -o ./hist 127.0.0.1:5432

# print a gen
./target/release/dbcop print -d ./hist/hist-00000/
```
