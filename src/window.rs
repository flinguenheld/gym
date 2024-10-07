/// Print and fill the frame
/// Leave the cursor at the end of 'txt'
pub fn print_window(txt: String, title: &str, success: u16, fails: u16, icon: &str) {
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

    if !icon.is_empty() {
        print!("{}{}", termion::cursor::Goto(10, 6), icon);
    }

    // Txt
    for (i, line) in txt.lines().enumerate() {
        print!("{}{}", termion::cursor::Goto(18, i as u16 + 6), line);
    }
}

/// Cut the txt in n lines of chars_per_line length.
/// on_space = true to wait the next space.
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

pub enum Icon {
    Warning,
    Loop,
    Answer,
    Bulb,
    None,
}

pub struct Window {
    pub title: String,
    pub success: u16,
    pub fails: u16,
    pub icon: Icon,
}

impl Window {
    pub fn new(title: String) -> Window {
        Window {
            title,
            success: 0,
            fails: 0,
            icon: Icon::None,
        }
    }

    pub fn print(&self, txt: String) {
        print!("{}{}", termion::clear::All, termion::cursor::Goto(1, 2));
        println!("    ┌──────────────────────────────────────────────────┬─────────┬─────────┐\r");
        println!("    │                                                  │         │         │\r");
        println!("    ├──────────────────────────────────────────────────┴─────────┴─────────┤\r");
        for _ in 0..(txt.lines().count() + 2) {
            println!(
                "    │                                                                      │\r"
            );
        }
        println!("    └──────────────────────────────────────────────────────────────────────┘\r");

        print!("{}{}", termion::cursor::Goto(7, 3), self.title);
        print!("{} ✅ {}", termion::cursor::Goto(58, 3), self.success);
        print!("{} ❌ {}", termion::cursor::Goto(68, 3), self.fails);

        match self.icon {
            Icon::Warning => {
                print!("{}{}", termion::cursor::Goto(10, 6), '❌');
            }
            Icon::Loop => {
                print!("{}{}", termion::cursor::Goto(10, 6), '🔄');
            }
            Icon::Answer => {
                print!("{}{}", termion::cursor::Goto(10, 6), '🎁');
            }
            _ => {}
        }

        // Txt
        for (i, line) in txt.lines().enumerate() {
            print!("{}{}", termion::cursor::Goto(18, i as u16 + 6), line);
        }
    }
}
