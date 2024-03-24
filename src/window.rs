pub fn print_window(title: &str, success: u16, fails: u16, warning: bool, nb_row: u8) {
    print!("{}{}", termion::clear::All, termion::cursor::Goto(1, 2));
    println!("    ┌──────────────────────────────────────────────────┬────────┬────────┐\r");
    println!("    │                                                  │        │        │\r");
    println!("    ├──────────────────────────────────────────────────┴────────┴────────┤\r");
    for _ in 0..nb_row {
        println!("    │                                                                    │\r");
    }
    println!("    └────────────────────────────────────────────────────────────────────┘\r");

    print!("{}{}", termion::cursor::Goto(7, 3), title);

    print!("{} ✅ {}", termion::cursor::Goto(58, 3), success);
    print!("{} ❌ {}", termion::cursor::Goto(67, 3), fails);

    if warning {
        print!("{}❌", termion::cursor::Goto(10, 6));
    }
}
