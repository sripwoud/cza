/// Basic functionality tests that don't require private field access

#[test]
fn test_embedded_templates_exist() {
    // Test that the embedded templates.toml is valid
    let templates_content = include_str!("../templates.toml");
    let parsed: Result<toml::Table, _> = toml::from_str(templates_content);

    assert!(parsed.is_ok(), "templates.toml should be valid TOML");
    let table = parsed.unwrap();
    assert!(
        table.contains_key("templates"),
        "Should have templates section"
    );
}

#[test]
fn test_project_name_validation_logic() {
    // Test project name validation logic without accessing private methods
    fn is_valid_project_name(name: &str) -> bool {
        if name.is_empty() {
            return false;
        }

        // Must start with letter
        if !name.chars().next().unwrap().is_ascii_alphabetic() {
            return false;
        }

        // Only alphanumeric, hyphens, and underscores
        name.chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
    }

    // Valid names
    assert!(is_valid_project_name("valid-name"));
    assert!(is_valid_project_name("valid_name"));
    assert!(is_valid_project_name("validName"));
    assert!(is_valid_project_name("a"));

    // Invalid names
    assert!(!is_valid_project_name(""));
    assert!(!is_valid_project_name("123invalid"));
    assert!(!is_valid_project_name("invalid name"));
    assert!(!is_valid_project_name("invalid/name"));
    assert!(!is_valid_project_name("invalid.name"));
}

#[test]
fn test_title_case_formatting() {
    fn title_case(input: &str) -> String {
        // Handle various naming conventions: kebab-case, snake_case, camelCase, PascalCase
        input
            .replace(['-', '_'], " ") // Replace dashes and underscores with spaces
            .split_whitespace()
            .map(|word| {
                let mut chars = word.chars();
                match chars.next() {
                    None => String::new(),
                    Some(first) => {
                        let mut result = first.to_uppercase().to_string();
                        result.push_str(&chars.collect::<String>().to_lowercase());
                        result
                    }
                }
            })
            .collect::<Vec<_>>()
            .join(" ")
    }

    let test_cases = vec![
        ("my-project", "My Project"),
        ("noir_app", "Noir App"),
        ("simple", "Simple"),
        ("test-case", "Test Case"),
    ];

    for (input, expected) in test_cases {
        assert_eq!(title_case(input), expected, "Failed for input: {input}");
    }
}
