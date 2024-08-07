fn main() {
    let mut s = String::from("Some text here");

    line_changer(&mut s);

    let len = line_size(&s);

    println!("{s} - {len} chars");

    println!("--------------------------");

    let mut some_text = String::from("text");

    let l1 = &some_text;
    let l2 = &some_text;
    

    println!("l1 - {l1}, l2 - {l2}");

    println!("second print:");
    println!("l1 - {l1}, l2 - {l2}");
    let l3 = &mut some_text;
    println!("{l3}");
}

fn line_size(s: &String) -> usize {
    s.len()
}

fn line_changer(s: &mut String) {
    s.push_str(". New text added!");
}