use uuid::Uuid;

pub fn generate_safe_file_name(original_name: &str) -> String {
    let original_name = original_name.replace(' ', "-");

    let clean_name: String = original_name
        .chars()
        .filter(|c| {
            c.is_ascii_alphanumeric()
                || *c == '.'
                || *c == '-'
                || *c == '_'
        })
        .collect();

    let parts: Vec<&str> = clean_name.rsplitn(2, '.').collect();

    if parts.len() == 2 {
        let extension = parts[0];
        let name = parts[1];

        format!("{}-{}.{}", name, Uuid::new_v4(), extension)
    } else {
        format!("{}-{}", clean_name, Uuid::new_v4())
    }
}

pub fn display_file_name(stored_name: &str) -> String {
    let parts: Vec<&str> = stored_name.rsplitn(2, '.').collect();

    if parts.len() == 2 {
        let extension = parts[0];
        let base = parts[1];

        if let Some((name, _uuid)) = base.rsplit_once('-') {
            if !name.is_empty() {
                return format!("{}.{}", name, extension);
            }
        }
    }

    stored_name.to_string()
}
