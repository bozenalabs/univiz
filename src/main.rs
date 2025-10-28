use unicode_segmentation::UnicodeSegmentation; // for the trait
use unicode_width::UnicodeWidthChar;
use unicode_width::UnicodeWidthStr;

fn main() {

    let s = std::env::args().nth(1).unwrap_or_else(|| "aé😀👩‍💻".to_string());

    print!("Analyzing string: '{}' width={} bytes={}\n", s, s.width(), s.len());
    let mut total_byte_idx = 0;
    for (gi, g) in s.grapheme_indices(true) {
        print !("grapheme='{}' width={} bytes={} byte_idx={}\n", g, g.width(), g.len(), gi);
        for (ci, c) in g.char_indices() {
            print !("  code point='{}' (U+{:X}) width={} byte_idx={} byte_idx_global={}\n", c, c as u32, c.width().unwrap_or(0), ci, total_byte_idx + ci);
            
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
                    }
                    _ => 4
                };

                // ANSI escape codes
                let red = "\x1b[31m";   // Red foreground
                let reset = "\x1b[0m";  // Reset to default

                // Split string at char boundaries
                let colored: String = binary
                    .chars()
                    .enumerate()
                    .map(|(i, c)| {
                        if i < prefix_to_color {
                            format!("{}{}{}", red, c, reset)
                        } else {
                            c.to_string()
                        }
                    })
                    .collect();

                
                print!("    utf8 byte: {:X} {}\n", b, colored);
                assert_eq!(s.as_bytes()[total_byte_idx +  ci + bi], *b);
            }
        }
        total_byte_idx += g.len();
    }
}
