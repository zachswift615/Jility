/// Generate URL-friendly slug from a name
pub fn generate_slug(name: &str) -> String {
    name.to_lowercase()
        .replace(|c: char| !c.is_alphanumeric() && c != ' ' && c != '-', "")
        .replace(' ', "-")
        .trim_matches('-')
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_slug_basic() {
        assert_eq!(generate_slug("My Workspace"), "my-workspace");
    }

    #[test]
    fn test_generate_slug_special_chars() {
        assert_eq!(generate_slug("John's Team!"), "johns-team");
    }

    #[test]
    fn test_generate_slug_multiple_spaces() {
        assert_eq!(generate_slug("  Many   Spaces  "), "many-spaces");
    }

    #[test]
    fn test_generate_slug_already_slug() {
        assert_eq!(generate_slug("already-a-slug"), "already-a-slug");
    }

    #[test]
    fn test_generate_slug_unicode() {
        assert_eq!(generate_slug("Café Résumé"), "caf-rsum");
    }
}
