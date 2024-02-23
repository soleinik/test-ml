# ml-cli

This is suplementary project for demo purposes, to provide some interactivity to [ml-lib](../ml-lib/) library. It uses [clap](https://crates.io/crates/clap) to systematize, integrity and validation of user supplied command line options, as well as self explanatory (hopefuly!) user help. 

## Status
There is no intention to cover all [linfa](https://crates.io/crates/linfa) ML algorithms... Just a few to get fill of "plumming" needed to connect data to Linfa algorithms. Currently, only the following Linfa's decision tree (`random forest`???)  works, that can be invoked as follows
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

