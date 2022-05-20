# simed

Simulate Degenerate Text

```
cargo install --path .
```


## Usage

Print the help text
```
simed -h
```

Generate Degenerate Text of width 100
```
simed 100 > x.eds
```

Options

```
    -d, --percent-degenerate <d>
            percentage of degenerate loci [default: 10]

    -h, --help
            Print help information

    -l, --max-length <max_length>
            max length of a degenerate segment [default: 1]

    -s, --max-variants <max_variants>
            Maximum number of variants in a degenerate position [default: 2]

    -V, --version
            Print version information
```
