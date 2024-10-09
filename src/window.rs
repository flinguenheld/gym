const BLUE: &str = "\x1b[34m";
const BOLD_ON: &str = "\x1b[1m";
const BOLD_OFF: &str = "\x1b[21m";
const OFF: &str = "\x1b[0m";
const RED: &str = "\x1b[31m";

pub enum Icon {
    Cross,
    Loop,
    Gift,
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

    /// Print and fill the frame
    /// Leave the cursor at the end of 'txt'
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
            Icon::Cross => {
                print!("{}❌", termion::cursor::Goto(10, 6));
            }
            Icon::Loop => {
                print!("{}🔄", termion::cursor::Goto(10, 6));
            }
            Icon::Gift => {
                print!("{}🎁", termion::cursor::Goto(10, 6));
            }
            _ => {}
        }

        // Txt
        for (i, line) in txt.lines().enumerate() {
            print!("{}{}", termion::cursor::Goto(18, i as u16 + 6), line);
        }
    }

    /// Clean the terminal, print a summary an exit.
    pub fn exit(&self) {
        print!(
            "{}{}{}{}{}:{}{} {} success {} fails ",
            termion::clear::All,
            termion::cursor::Goto(1, 1),
            BLUE,
            BOLD_ON,
            self.title,
            BOLD_OFF,
            OFF,
            self.success,
            self.fails
        );
        std::process::exit(0);
    }
}

/// Clean the terminal, print an error an exit.
pub fn exit_with_error(text: &str) {
    print!(
        "{}{}{}{}Error:{}{} {}{}See gym -h ",
        termion::clear::All,
        termion::cursor::Goto(1, 1),
        RED,
        BOLD_ON,
        BOLD_OFF,
        OFF,
        text,
        termion::cursor::Goto(8, text.lines().count() as u16 + 1)
    );
    std::process::exit(1);
}

/// Cut txt in n lines of chars_per_line length.
/// on_space = true to wait the next space.
pub fn format(txt: &str, chars_per_line: usize, on_space: bool) -> String {
    let mut lines = String::new();
    let mut l = String::new();
    for c in txt.chars() {
        l.push(c);

        if (!on_space || c.is_whitespace()) && l.len() > chars_per_line {
            lines = format!("{}{}\n", lines, l.trim_end());
            l.clear();
        }
    }

    format!("{}{}", lines, l)
}
