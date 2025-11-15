# Univiz

A command-line tool for analyzing Unicode strings, providing detailed information about graphemes, code points, and UTF-8 byte sequences.

## Installation

```bash
cargo install univiz
```

## Usage

```bash
univiz "your string here"
```

## Example Output

```
Analyzing string: 'aé😀👩‍💻' width=6 bytes=19
grapheme='a' width=1 bytes=1 byte_idx=0
  code point='a' (U+61) width=1 byte_idx=0 byte_idx_global=0
    utf8 byte: 61 0b1100001
grapheme='é' width=1 bytes=3 byte_idx=1
  code point='e' (U+65) width=1 byte_idx=0 byte_idx_global=1
    utf8 byte: 65 0b1100101
  code point='́' (U+301) width=0 byte_idx=1 byte_idx_global=2
    utf8 byte: CC 0b11001100
    utf8 byte: 81 0b10000001
grapheme='😀' width=2 bytes=4 byte_idx=4
  code point='😀' (U+1F600) width=2 byte_idx=0 byte_idx_global=4
    utf8 byte: F0 0b11110000
    utf8 byte: 9F 0b10011111
    utf8 byte: 98 0b10011000
    utf8 byte: 80 0b10000000
grapheme='👩‍💻' width=2 bytes=11 byte_idx=8
  code point='👩' (U+1F469) width=2 byte_idx=0 byte_idx_global=8
    utf8 byte: F0 0b11110000
    utf8 byte: 9F 0b10011111
    utf8 byte: 91 0b10010001
    utf8 byte: A9 0b10101001
  code point='‍' (U+200D) width=0 byte_idx=4 byte_idx_global=12
    utf8 byte: E2 0b11100010
    utf8 byte: 80 0b10000000
    utf8 byte: 8D 0b10001101
  code point='💻' (U+1F4BB) width=2 byte_idx=7 byte_idx_global=15
    utf8 byte: F0 0b11110000
    utf8 byte: 9F 0b10011111
    utf8 byte: 92 0b10010010
    utf8 byte: BB 0b10111011
```

## Features

- Grapheme cluster analysis
- Code point information with Unicode values
- UTF-8 byte sequence breakdown
- Display width calculation
- Detailed byte indexing (local and global)
