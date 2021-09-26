pub fn reply(message: &str) -> &str {
    let letters = message.chars().any(|c| c.is_ascii_alphabetic());
    let caps = message == message.to_uppercase() && letters;
    let ques = message.trim().ends_with("?");
    let ws = message.chars().all(|c| c.is_whitespace());

    match (ws, caps, ques) {
        (true, _, _) => "Fine. Be that way!",
        (false, true, true) => "Calm down, I know what I'm doing!",
        (false, false, true) => "Sure.",
        (false, true, false) => "Whoa, chill out!",
        _ =>  "Whatever."
    }
}
