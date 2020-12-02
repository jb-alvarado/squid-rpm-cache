
use text_io::read; // For easy stdin input
use regex::Regex;

fn main() {
    // TODO: update urls to handle the repodata metadata
    let url_regex = Regex::new(r"[^/]+(\.rpm|\.deb|\.iso)").unwrap();

    loop {
        let line: String = read!("{}\n");
        let parts: Vec<&str> = line.split(" ").collect();

        if parts.len() > 0 {
            let url = parts[0];
            let mut id = url;

            // If it matches, only use the base rpm name instead of the full url
            if let Some(captures) = url_regex.captures(url) {
                id = captures.get(0).unwrap().as_str();
            }

            println!("OK store-id={}", id);
        } else {
            eprintln!("Unknown input: {}", line);
        }
    }
}
