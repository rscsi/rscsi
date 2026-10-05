pub fn whoami() -> &'static str {
    env!("CARGO_PKG_NAME")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_is_me() {
        assert_eq!(whoami(), env!("CARGO_PKG_NAME"));
    }
}
