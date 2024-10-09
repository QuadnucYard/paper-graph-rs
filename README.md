# paper-graph-rs

A rust tool for generating elegant paper graphs using graphviz.

## Usage

```txt
Usage: paper-graph.exe [OPTIONS] --bib <FILE> --graph <FILE> --output <FILE>

Options:
  -b, --bib <FILE>          Path to the .bib file
  -g, --graph <FILE>        Path to the graph structure file
  -o, --output <FILE>       Path to the output file
      --line-width <WIDTH>  The maximum line width in nodes [default: 32]
  -h, --help                Print help
  -V, --version             Print version
```

This tool only create `.dot` file. You would generate other formats by yourself.
