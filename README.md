# A Machine Learning test project

## About
The project's goal is to implement start to end file load with [polars](https://crates.io/crates/polars) and use [linfa](https://crates.io/crates/linfa) crates to learn and predict financial time series data, with [ndarray](https://crates.io/crates/ndarray) in between.
Project is build from two parts [CLI](/ml-cli/) and [library](/ml-lib/). For now [data](/data/) is local folder that shoud be substituted, in future, with a financial data fetching API (i.e. [yahoo_finance_api](https://crates.io/crates/yahoo_finance_api))


## Status
Work-in-progress. At this moment only (list what is implemented) 

Currently, only the following Linfa's decision tree (`random forest`???)  works, that can be invoked as follows
```
$ ./target/debug/ml-cli -Ccdr -h
[r]andom forest learning algorithm

Usage: ml-cli AlgoCategory SL DT {RF|-r}

Options:
  -h, --help  Print help (see more with '--help')
```
actual run
```
./target/debug/ml-cli -Ccdr
accuracy:1
predicted:[1, 1, 1]
```

Progam will use time series data from [IBM, 5 years](./data/IBM-5y.csv) as base, [S&P500, 5 years](./data/^GSPC-5y.csv) as suplementary and attempt to learn and predict closing price trend for next 3 days.

NOTE: the goal is not actually make correct prediction, rather understand `polars`, `ndarray` and `linfa` interaction 


## Compiling from source code (via git)

`test-ml` can be compiled directly from the git repository source code using the following method.


```
$ git clone https://github.com/soleinik/test-ai.git && cd test-ai
[...]
$ cargo build --release
```

If successful, this will produce a `ml-cli` executable located at `./target/release/ml-cli`

Executable (for now) expects `./data` folder to be available relative to execution root `.`

## Running:

### `Cargo`
```
$ cargo run -- -h

Demo CLI wrapper

Usage: ml-cli [OPTIONS] <COMMAND>

Commands:
  AlgoCategory, -C  Algorithm's [C]ategory
  help              Print this message or the help of the given subcommand(s)

Options:
  -v, --verbosity...  log level
  -h, --help          Print help (see more with '--help')
  -V, --version       Print version

```
and follow help prompts... `help` is context sencitive. The above snippet demonstrates generic help prompt. The following snippet demonstrates help items for `Algorithm's category` 

```
$ cargo run -- -C -h
Algorithm's [C]ategory

Usage: ml-cli {AlgoCategory|-C} <COMMAND>

Commands:
  SL, -c             [s]upervised learning algorithms class
  USL, -u            [u]nSupervised learning algorithm class
  partial-fit, -f    Partial [f]it ML algorithms class
  pre-processor, -p  [p]re Processors ML algorithms class
  help               Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help

```
### Executable `ml-cli`
When successfuly built, executable will be located in either `./target/debug` or `./target/release` folder, depends on `--debug`(default) or `--release` flag passed to `cargo build`. Move executable `ml-cli` together with `./data/*` forder into same location. After that invoke it as follows

```
$ ./ml-cli -h
Demo CLI wrapper

Usage: ml-cli [OPTIONS] <COMMAND>

Commands:
  AlgoCategory, -C  Algorithm's [C]ategory
  help              Print this message or the help of the given subcommand(s)

Options:
  -v, --verbosity...  log level
  -h, --help          Print help (see more with '--help')
  -V, --version       Print version
```



## Development pre-requisites

To _build & run_ this project you need to have the following installed on your system:

- Install **Rust** (latest stable) – [How to install Rust](https://www.rust-lang.org/en-US/install.html)
- Install **rustfmt**: `rustup component add rustfmt`
- Install **clippy**: `rustup component add clippy`


### Format checking (rustfmt)

Make sure your code is well-formatted by running:

```
cargo fmt
```

### Lint (clippy)

Lint your code (i.e. check it for common issues) with:

```
cargo clippy
```
