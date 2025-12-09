pub fn transform(text: &mut String) -> &String {
    let first = text.chars().next().unwrap_or_default();

    match first {
        'a' | 'A' | 'e' | 'E' | 'i' | 'I' | 'o' | 'O' | 'u' | 'U' => text.push_str("-hay"),
        _ => *text = format!("{}-{first}ay", text.chars().skip(1).collect::<String>()),
    }
    text
}
