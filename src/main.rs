use clap::Parser;
use colored::*;
use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthChar;
use unicode_width::UnicodeWidthStr;
use unicode_names2::name;

/// A command-line tool for analyzing Unicode strings, providing detailed information 
/// about graphemes, code points, and UTF-8 byte sequences.
#[derive(Parser)]
#[command(name = "univiz")]
#[command(about = "Unicode string analyzer - visualize graphemes, code points, and UTF-8 encoding")]
#[command(long_about = "Univiz analyzes Unicode strings and provides detailed information about:
- Grapheme cluster analysis with display width
- Code point information with Unicode values and names  
- UTF-8 byte sequence breakdown with binary representation
- Detailed byte indexing (local and global)")]
#[command(version)]
struct Cli {
    /// The string to analyze (default: 'aé😀👩‍💻')
    text: Option<String>,
}

fn main() {
    let cli = Cli::parse();
    let s = cli.text.unwrap_or_else(|| "aé😀👩‍💻".to_string());

    println!(
        "Analyzing string: '{}' {} {}",
        s,
        format!("width={}", s.width()).dimmed(),
        format!("bytes={}", s.len()).dimmed()
    );
    let mut total_byte_idx = 0;
    for (gi, g) in s.grapheme_indices(true) {
        println!(
            "grapheme='{}' {} {} {}",
            g,
            format!("width={}", g.width()).dimmed(),
            format!("bytes={}", g.len()).dimmed(),
            format!("byte_idx={}", gi).dimmed()
        );
        for (ci, c) in g.char_indices() {

            let char_name = name(c).map(|n| n.to_string()).unwrap_or("<unknown>".to_string());
            println!(
                "  code point='{}' (U+{:X}) ({}) {} {} {}",
                c,
                c as u32,
                char_name,
                format!("width={}", c.width().unwrap_or(0)).dimmed(),
                format!("byte_idx={}", ci).dimmed(),
                format!("byte_idx_global={}", total_byte_idx + ci).dimmed()
            );

            let mut buffer = [0; 4];
            let bytes = c.encode_utf8(&mut buffer).as_bytes();
            for (bi, b) in bytes.iter().enumerate() {
                let binary = format!("{:#b}", b);
                let prefix_to_color = match bi {
                    0 => match bytes.len() {
                        1 => 3,
                        2 => 5,
                        3 => 6,
                        4 => 7,
                        _ => 0,
                    },
                    _ => 4,
                };

                // Split string at char boundaries
                let colored: String = binary
                    .chars()
                    .enumerate()
                    .map(|(i, c)| {
                        if i < prefix_to_color {
                            c.to_string().red().to_string()
                        } else {
                            c.to_string()
                        }
                    })
                    .collect();

                let idx_string = format!("({})", bi).dimmed();
                println!("    utf8 byte {}: {:X} {}", idx_string, *b, colored);
                assert_eq!(s.as_bytes()[total_byte_idx + ci + bi], *b);
            }
        }
        total_byte_idx += g.len();
    }
}
