# labelize

> **Turn ZPL/EPL into pixels — label rendering, simplified.**

A fast, embeddable Rust engine that parses ZPL (Zebra Programming Language) and EPL (Eltron Programming Language) label data and renders it to PNG or PDF. Use it as a CLI tool, an HTTP microservice, or a library in your own Rust project.

## Features

- **ZPL Parser** — Full ZPL command support including text, barcodes, graphics, stored formats, and graphic fields
- **EPL Parser** — EPL command support for text, barcodes, line draw, and reference points
- **Barcode Support** — Code 128, Code 39, EAN-13, Interleaved 2-of-5, PDF417, Aztec, DataMatrix, QR Code, MaxiCode
- **PNG & PDF Output** — Monochrome 1-bit PNG or embedded PDF output
- **CLI Tool** — Convert label files from the command line with auto-detection, multi-label support, and customizable dimensions
- **HTTP Server** — RESTful API for label conversion with format detection via Content-Type header
- **Embedded Fonts** — No runtime font dependencies; includes Helvetica Bold Condensed, DejaVu Sans Mono, and ZPL GS fonts
- **Library API** — Use as a Rust library in your own applications

## Quick Start

### Install

```bash
# From source
cargo install --path .

# Or via Homebrew
brew tap GOODBOY008/homebrew-labelize && brew install labelize
```

### Convert a label

```bash
# ZPL → PNG (auto-detected)
labelize convert label.zpl

# EPL → PNG
labelize convert label.epl

# ZPL → PDF
labelize convert label.zpl -t pdf

# Custom dimensions (100×62mm at 12 dpmm)
labelize convert label.zpl --width 100 --height 62 --dpmm 12
```

### Run as HTTP server

```bash
labelize serve --port 8080

# Convert via API
curl -X POST http://localhost:8080/convert \
  -H "Content-Type: application/zpl" \
  -d '^XA^FO50,50^A0N,40,40^FDHello World^FS^XZ' \
  -o label.png
```

## CLI Reference

```
Usage: labelize <COMMAND>

Commands:
  convert  Convert a ZPL/EPL file to PNG or PDF
  serve    Start HTTP server for label conversion

Convert Options:
  <INPUT>               Input file path (.zpl or .epl)
  -o, --output <PATH>   Output file path (default: input stem + .png/.pdf)
  -f, --format <FMT>    Input format override: zpl | epl
  -t, --type <TYPE>     Output type: png | pdf [default: png]
  --width <MM>          Label width in mm [default: 102]
  --height <MM>         Label height in mm [default: 152]
  --dpmm <N>            Dots per mm [default: 8]

Serve Options:
  --host <HOST>         Bind address [default: 0.0.0.0]
  -p, --port <PORT>     Listen port [default: 8080]
```

## HTTP API

| Endpoint       | Method | Description                                   |
|---------------|--------|-----------------------------------------------|
| `/health`     | GET    | Health check → `{"status":"ok"}`             |
| `/convert`    | POST   | Convert label data → PNG or PDF              |

**POST /convert** query parameters:

| Parameter | Default | Description            |
|-----------|---------|------------------------|
| `width`   | 102     | Label width in mm      |
| `height`  | 152     | Label height in mm     |
| `dpmm`    | 8       | Dots per mm            |
| `output`  | png     | Output format: png/pdf |

Set `Content-Type: application/zpl` or `Content-Type: application/epl` to select the parser.

## Library Usage

```rust
use std::io::Cursor;
use labelize::{ZplParser, Renderer, DrawerOptions};

let zpl = b"^XA^FO50,50^A0N,40,40^FDHello^FS^XZ";
let mut parser = ZplParser::new();
let labels = parser.parse(zpl).unwrap();

let renderer = Renderer::new();
let mut buf = Cursor::new(Vec::new());
renderer.draw_label_as_png(&labels[0], &mut buf, DrawerOptions::default()).unwrap();

std::fs::write("output.png", buf.into_inner()).unwrap();
```

## Supported Commands

### ZPL

Text: `^FO` `^FT` `^FD` `^FS` `^A` `^CF` `^FB` `^FR` `^FH` `^FN` `^FW` `^FV`
Barcodes: `^BC` `^BE` `^B2` `^B3` `^B7` `^BO` `^BX` `^BQ` `^BD` `^BY`
Graphics: `^GB` `^GC` `^GD` `^GF` `^GS` `~DG` `^IL` `^XG`
Label: `^XA` `^XZ` `^PW` `^PO` `^LH` `^LR` `^CI`
Formats: `^DF` `^XF`

### EPL

`N` (new label) · `A` (text) · `B` (barcode) · `LO` (line draw) · `R` (reference point) · `P` (print)

## Architecture

```
  ZPL/EPL input
       │
       ▼
  ┌─────────┐     ┌──────────┐     ┌─────────┐
  │  Parser  │ ──▶ │ Renderer │ ──▶ │ Encoder │
  └─────────┘     └──────────┘     └─────────┘
       │                │                │
   LabelInfo        RgbaImage       PNG / PDF
```

- **Parser** — Tokenizes input, maintains VirtualPrinter state, produces `Vec<LabelElement>`
- **Renderer** — Creates canvas, iterates elements, dispatches drawing (text, graphics, barcodes), handles reverse print and label inversion
- **Encoder** — Converts RGBA image to monochrome PNG or embeds into single-page PDF

## Testing

```bash
# Run E2E golden-file tests (compares output against Go reference images)
cargo test

# Run with verbose output
cargo test -- --nocapture
```

57 golden-file E2E tests compare rendered output against reference PNGs from the original Go implementation.

## Building

```bash
cargo build --release
```

The binary is at `target/release/labelize`.

## License

See [LICENSE](../LICENSE) in the repository root.
