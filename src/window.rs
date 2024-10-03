/// Print and fill the frame
/// Leave the cursor at the end of 'txt'
pub fn print_window(txt: String, title: &str, success: u16, fails: u16, warning: bool) {
    print!("{}{}", termion::clear::All, termion::cursor::Goto(1, 2));
    println!("    ┌──────────────────────────────────────────────────┬─────────┬─────────┐\r");
    println!("    │                                                  │         │         │\r");
    println!("    ├──────────────────────────────────────────────────┴─────────┴─────────┤\r");
    for _ in 0..(txt.lines().count() + 2) {
        println!("    │                                                                      │\r");
    }
    println!("    └──────────────────────────────────────────────────────────────────────┘\r");

    print!("{}{}", termion::cursor::Goto(7, 3), title);
    print!("{} ✅ {}", termion::cursor::Goto(58, 3), success);
    print!("{} ❌ {}", termion::cursor::Goto(68, 3), fails);

    if warning {
        print!("{}❌", termion::cursor::Goto(10, 6));
    }

    // Txt
    for (i, line) in txt.lines().enumerate() {
        print!("{}{}", termion::cursor::Goto(18, i as u16 + 6), line);
    }
}

/// Cut the line in n lines of chars_per_line length
pub fn format(txt: String, chars_per_line: usize, on_space: bool) -> String {
    let mut lines = String::new();
    let mut l = String::new();
    for c in txt.chars() {
        l.push(c);

        if (!on_space || c.is_whitespace()) && l.len() > chars_per_line {
            lines = format!("{}{}\n", lines, l.trim_end());
            l.clear();
        }
    }

    format!("{}{}\n", lines, l)
}
