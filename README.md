# bitvec-perf-regression

Quick'n'dirty attemp to reproduce https://github.com/rust-lang/rust/issues/79246

## Introduction

This small crate goal is to measure number of retired instructions when doing a call to `BitVec::extend` in a similar way to what has been observed in the previously mentionned issue.

## Instructions

First, let's decompress the provided `results.json.bz2` file:

```
$ bunzip2 results.json.bz2
```

The decompressed JSON file takes about 117M. Make sure you have enough disk space.

Then, let's build the crate:

```
$ cargo build --release
```

Finally, let's run the example:

```
$ setarch x86_64 -R nice -20 taskset -c 3,7 cargo run --release -- results.json
```

Why not using `cargo` directly? Because those tools help improve reproducibility of results:

* `setarch x86_64 -R` disables ASLR.
* `nice -20` sets the highest scheduling priority.
* `tasket -c 3,7` selects a specific physical core to run and should prevent migration to another core. Use appropriate values on your machine. See `/proc/cpuinfo` for more information.

## Results

On my machine, a `Lenovo ThinkPad T480s` with an `Intel(R) Core(TM) i7-8650U CPU @ 1.90GHz` CPU, the results are as follow:

* rust 1.47.0:

```
instructions:u = 1272309573
```

* rust 1.48.0:

```
instructions:u = 1576958223
```

That's a performance regression of about `24%`.

## `rust` bisection

Bisection fails to complete but it *seems* that the regression was introduced in `nightly-2020-09-02`:

```
********************************************************************************
Regression in nightly-2020-09-02
********************************************************************************

fetching https://static.rust-lang.org/dist/2020-09-01/channel-rust-nightly-git-commit-hash.txt
ERROR: Tarball not found at https://static.rust-lang.org/dist/2020-09-01/channel-rust-nightly-git-commit-hash.txt
```

A previous bisection on a closed-source codebase pointed to `nightly-2020-09-04` and to that pull-request: https://github.com/rust-lang/rust/pull/70793
Not sure at this point why bisection found 2 differents nightly.

## `bitvec` bisection

Using only rust 1.48.0, bisection showed that the regression:

* was triggered when using version `0.17.4`
* was *not* triggered when using version `0.18.0`

Further (manual) commit-by-commit bisection shows that the regression was not triggered starting with commit https://github.com/myrrlyn/bitvec/commit/f82112243707923a380e5091deab51c29b4f2948 which is basically a rewrite of the `bitvec` crate.
