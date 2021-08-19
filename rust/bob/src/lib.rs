// determine if letters are all capital letters
fn all_caps(msg: &str) -> bool {
    msg.chars()
        .filter(|c| c.is_ascii_alphabetic())
        .all(|c| c.is_ascii_uppercase())
}

// determine if string has no alphabet
fn no_alphabet(msg: &str) -> bool {
    msg.chars().all(|c| !c.is_alphabetic())
}

pub fn reply(message: &str) -> &str {
    println!("message=\"{}\"", message);

    // remove trailing whitespace
    let msg = message.trim();

    // All whitespace -> Fine. Be that way!
    if msg.trim().split_whitespace().count() == 0 {
        return "Fine. Be that way!";
    }

    // Questions: ends with "?"
    if let Some(last_char) = msg.chars().last() {
        if last_char == '?' {
            // Question Yell -> Calm down, I know what I'm doing!
            if all_caps(msg) && !no_alphabet(msg) {
                return "Calm down, I know what I'm doing!";
            } else {
                // Asks Question -> Sure.
                return "Sure.";
            }
        }
    }

    // Yell -> Whoa, chill out!
    if all_caps(msg) && !no_alphabet(msg) {
        return "Whoa, chill out!";
    }

    "Whatever."
}
