pub fn print_window(title: &str, success: u16, fails: u16, warning: bool) {
    print!("{}{}", termion::clear::All, termion::cursor::Goto(1, 2));
    println!("    ┌──────────────────────────────────────────┬────────┬────────┐");
    println!("    │                                          │        │        │");
    println!("    ├──────────────────────────────────────────┴────────┴────────┤");
    println!("    │                                                            │");
    println!("    │                                                            │");
    println!("    │                                                            │");
    println!("    └────────────────────────────────────────────────────────────┘");

    print!("{}{}", termion::cursor::Goto(7, 3), title);

    print!("{} ✅ {}", termion::cursor::Goto(50, 3), success);
    print!("{} ❌ {}", termion::cursor::Goto(59, 3), fails);

    if warning {
        print!("{}❌", termion::cursor::Goto(10, 6));
    }
}
