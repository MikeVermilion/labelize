# Rendering Diff Thresholds

Labelize renders ZPL/EPL labels to pixel images. This document tracks the
expected pixel-difference percentage for every test label compared against
reference images from the [Labelary ZPL viewer](https://labelary.com/viewer.html).

## Reference Setup

| Parameter | Value |
|-----------|-------|
| DPI | 8 dpmm (≈ 203 dpi) |
| Label size | 4.005 × 8.01 inches (101.625 × 203.25 mm) |
| Pixel dims | 813 × 1626 |
| Source | Labelary API `http://api.labelary.com/v1/printers/8dpmm/labels/4.005x8.01/0/` |

The EPL label `dpduk.epl` uses a reference rendered by the Go-based labelize
predecessor because Labelary does not support EPL.

## Diff Categories

| Category | Range | Meaning |
|----------|-------|---------|
| PERFECT | 0 % | Pixel-identical |
| GOOD | < 1 % | Sub-pixel / anti-alias noise |
| MINOR | 1 – 5 % | Small font or position deltas |
| MODERATE | 5 – 15 % | Font engine, embedded graphics, or 2D barcode differences |
| HIGH | ≥ 15 % | Missing encoder or large structural mismatch |

## Per-Label Thresholds

Each label has a CI tolerance set to **current diff + 2 % headroom**.
If a future change raises the diff beyond this ceiling the golden test fails.

| Label | Ext | Diff % | Tolerance | Primary diff source |
|-------|-----|--------|-----------|---------------------|
| amazon | zpl | 5.78 | 8.0 | Font metrics |
| aztec_ec | zpl | 8.07 | 10.0 | Aztec barcode encoding |
| barcode128_default_width | zpl | 0.72 | 3.0 | Sub-pixel barcode bars |
| barcode128_line | zpl | 0.23 | 3.0 | Sub-pixel |
| barcode128_line_above | zpl | 0.26 | 3.0 | Sub-pixel |
| barcode128_mode_a | zpl | 0.72 | 3.0 | Sub-pixel |
| barcode128_mode_d | zpl | 2.90 | 5.0 | Font + barcode bars |
| barcode128_mode_n | zpl | 0.72 | 3.0 | Sub-pixel |
| barcode128_mode_n_cba_sets | zpl | 1.40 | 4.0 | Barcode set switching |
| barcode128_mode_u | zpl | 3.32 | 5.5 | Font metrics |
| barcode128_rotated | zpl | 0.23 | 3.0 | Sub-pixel |
| bstc | zpl | 0.00 | 1.0 | Perfect |
| dbs | zpl | 9.69 | 12.0 | GFA graphics + font |
| dhlecommercetr | zpl | 5.47 | 8.0 | Font metrics |
| dhlpaket | zpl | 8.53 | 11.0 | GFA graphics + font |
| dhlparceluk | zpl | 5.98 | 8.0 | Font metrics |
| dpdpl | zpl | 7.60 | 10.0 | Font metrics |
| dpduk | epl | 5.83 | 8.0 | EPL reference from Go renderer |
| ean13 | zpl | 0.88 | 3.0 | Sub-pixel |
| encodings_013 | zpl | 2.18 | 5.0 | Character encoding |
| fedex | zpl | 18.26 | 20.0 | PDF417 encoding + font |
| gb_0_height | zpl | 0.00 | 1.0 | Perfect |
| gb_0_width | zpl | 0.00 | 1.0 | Perfect |
| gb_normal | zpl | 0.00 | 1.0 | Perfect |
| gb_rounded | zpl | 0.07 | 1.0 | Rounding artefacts |
| glscz | zpl | 4.25 | 7.0 | Font metrics |
| glsdk_return | zpl | 7.11 | 10.0 | Font metrics |
| gs | zpl | 1.01 | 3.0 | Graphic symbol font |
| icapaket | zpl | 5.14 | 8.0 | Font metrics |
| jcpenney | zpl | 7.18 | 10.0 | Font metrics |
| kmart | zpl | 8.75 | 11.0 | Font metrics |
| labelary | zpl | 9.84 | 12.0 | Font metrics + Code128 |
| pnldpd | zpl | 13.58 | 16.0 | GFA graphics + Aztec + Code128 |
| pocztex | zpl | 7.94 | 10.0 | Font metrics |
| porterbuddy | zpl | 13.79 | 16.0 | GFA graphics + QR code |
| posten | zpl | 4.74 | 7.0 | Font metrics |
| qr_code_ft_manual | zpl | 3.00 | 5.0 | QR barcode + position |
| qr_code_offset | zpl | 1.69 | 4.0 | QR position offset |
| return_qrcode | zpl | 6.08 | 8.5 | QR + font |
| reverse | zpl | 0.79 | 3.0 | Sub-pixel |
| reverse_qr | zpl | 2.98 | 5.0 | QR barcode |
| swisspost | zpl | 1.94 | 4.0 | Font metrics |
| templating | zpl | 4.89 | 7.0 | Font metrics |
| text_fallback_default | zpl | 4.66 | 7.0 | Font metrics |
| text_fo_b | zpl | 0.11 | 2.0 | Sub-pixel |
| text_fo_i | zpl | 0.11 | 2.0 | Sub-pixel |
| text_fo_n | zpl | 0.10 | 2.0 | Sub-pixel |
| text_fo_r | zpl | 0.10 | 2.0 | Sub-pixel |
| text_ft_auto_pos | zpl | 1.55 | 4.0 | Auto-position cursor |
| text_ft_b | zpl | 0.04 | 2.0 | Sub-pixel |
| text_ft_i | zpl | 0.04 | 2.0 | Sub-pixel |
| text_ft_n | zpl | 0.14 | 2.0 | Sub-pixel |
| text_ft_r | zpl | 0.15 | 2.0 | Sub-pixel |
| text_multiline | zpl | 0.68 | 3.0 | Word-wrap boundaries |
| ups | zpl | 25.19 | 28.0 | MaxiCode (no encoder) |
| ups_surepost | zpl | 35.61 | 38.0 | MaxiCode (no encoder) |
| usps | zpl | 5.63 | 8.0 | Font metrics |

## Known Limitations

### MaxiCode (ups, ups_surepost)
MaxiCode is a proprietary 2D symbology used by UPS. No Rust encoding library
exists. The current encoder draws the correct bullseye and hexagonal grid
structure but the data encoding is not standards-compliant. These labels will
remain HIGH until a proper encoder is implemented.

### PDF417 (fedex)
The `pdf417` crate produces **valid, scannable** barcodes, but the specific
codeword arrangement differs from Labelary's encoder. Both are correct per the
ISO 15438 specification; different encoders may choose different text/byte/numeric
compaction modes resulting in visually different (but equivalent) barcodes.

### Aztec (aztec_ec, pnldpd)
The `rxing` crate's Aztec writer produces proper Aztec codes. Minor differences
stem from error correction level defaults and symbol sizing when the ZPL
parameters leave the size open.

### Font Rendering
Labelize uses `ab_glyph` with Helvetica Bold Condensed for font 0 and
DejaVu Sans Mono variants for bitmap fonts A–H. Labelary uses its own
proprietary font set. Character advance widths and hinting differ between
engines, causing 1–10 % diffs on text-heavy labels.

### GFA Graphics
Embedded `^GFA` hex graphics are decoded and rasterised accurately. Remaining
differences (< 2 %) are primarily from anti-aliasing at logo edges and slight
coordinate rounding.

## Updating References

To regenerate all Labelary reference images:

```sh
# ZPL labels (Labelary API)
for f in testdata/*.zpl; do
  name=$(basename "$f" .zpl)
  curl -s -X POST http://api.labelary.com/v1/printers/8dpmm/labels/4.005x8.01/0/ \
    -F "file=@$f" -o "testdata/${name}.png"
done

# EPL labels — Labelary does not support EPL.
# Use the Go renderer or keep existing references.
```

## Running the Diff Report

```sh
# Full report (no failure on HIGH)
cargo test --test e2e diff_report -- --nocapture

# Golden tests with per-label tolerances (fails on regression)
cargo test --test e2e e2e_golden -- --test-threads=4
```
