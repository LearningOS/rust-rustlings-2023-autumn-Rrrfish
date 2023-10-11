// strings3.rs
//
// Execute `rustlings hint strings3` or use the `hint` watch subcommand for a
// hint.

fn trim_me(input: &str) -> String {
    // TODO: Remove whitespace from both ends of a string!

    let mut bytes = input.as_bytes();
    let mut start_idx = 0;
    let mut end_idx = bytes.len();

    while bytes[start_idx] == b' ' {
        start_idx += 1;
    }

    while bytes[end_idx - 1] == b' ' {
        end_idx -= 1;
    }
    input[start_idx..end_idx].to_string()
}

fn compose_me(input: &str) -> String {
    // TODO: Add " world!" to the string! There's multiple ways to do this!
    input.to_string() + " world!"
}

fn replace_me(input: &str) -> String {
    // TODO: Replace "cars" in the string with "balloons"!
    let refer = "cars".as_bytes();
    let client = input.as_bytes();
    let mut start_idx = 0;
    let mut end_idx = 0;
    let mut i = 0;
    let mut j = 0;

    while i < client.len() {
        if client[i] == refer[j] {
            j += 1;
            if j == refer.len() {
                end_idx = i;
                start_idx = end_idx - refer.len();
                break;
            }
        } else {
            j = 0;
        }

        i += 1;
    }

    input[0..=start_idx].to_string() + "balloons" + &input[(end_idx + 1)..]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_a_string() {
        assert_eq!(trim_me("Hello!     "), "Hello!");
        assert_eq!(trim_me("  What's up!"), "What's up!");
        assert_eq!(trim_me("   Hola!  "), "Hola!");
    }

    #[test]
    fn compose_a_string() {
        assert_eq!(compose_me("Hello"), "Hello world!");
        assert_eq!(compose_me("Goodbye"), "Goodbye world!");
    }

    #[test]
    fn replace_a_string() {
        assert_eq!(
            replace_me("I think cars are cool"),
            "I think balloons are cool"
        );
        assert_eq!(
            replace_me("I love to look at cars"),
            "I love to look at balloons"
        );
    }
}
