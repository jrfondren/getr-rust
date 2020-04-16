# A benchmarking wrapper around getrusage
- known to work on Linux
- probably misreports RSS on macOS
- created as my simple "time for x in {1..100}; ..." benchmarks were a lot less pleasant on OpenBSD.

## notes
- this is a Rust translation of the C version at https://github.com/jrfondren/getr
- for serious benchmarking I'd recommend [hyperfine](https://crates.io/crates/hyperfine), but I like getrusage() for giving peak RSS at the same time as a reasonable average.
- if you like Rosetta Code you might like the eight other implementations of this same program that I've written in other languages.

## build
```
make
```

## usage and examples
```
$ getr 1000 ./fizzbuzz >/dev/null
User time      : 0 s, 434408 us
System time    : 0 s, 239131 us
Time           : 673 ms (0.673 ms/per)
Max RSS        : 2.0 MB
Page reclaims  : 82394
Page faults    : 0
Block inputs   : 0
Block outputs  : 0
vol ctx switches   : 1000
invol ctx switches : 122

$ getr 100 python3 -c ''
User time      : 1 s, 338421 us
System time    : 0 s, 273103 us
Time           : 1611 ms (16.110 ms/per)
Max RSS        : 8.6 MB
Page reclaims  : 103173
Page faults    : 0
Block inputs   : 0
Block outputs  : 0
vol ctx switches   : 99
invol ctx switches : 19
```
