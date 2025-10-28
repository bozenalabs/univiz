use unicode_segmentation::UnicodeSegmentation; // for the trait
use unicode_width::UnicodeWidthChar;

fn main() {


    let s = "😀a👩‍💻";
    for (i, c) in s.char_indices() {
        println!("code point '{}' starts at byte index {}", c, i);
    }

    for (i, c) in s.grapheme_indices(true) {
        println!("grapheme '{}' starts at byte index {}", c, i);
    }

    for c in ['😀', 'a', '👩', '‍', '\u{0301}', '💻'] {
        println!(
            "The character {} has a width of {}",
            c,
            c.width().unwrap_or(0)
        );
    }

}
