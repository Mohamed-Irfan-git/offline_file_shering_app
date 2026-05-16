use axum::response::Html;

use std::fs;

use crate::{
    templates::html::page_html,
    utils::file_size::format_file_size,
};

pub async fn home() -> Html<String> {

    let mut file_list = String::new();

    if let Ok(entries) = fs::read_dir("uploads") {

        for entry in entries.flatten() {

            let path = entry.path();

            if path.is_file() {

                let file_name = entry
                    .file_name()
                    .to_string_lossy()
                    .to_string();

                let size = entry
                    .metadata()
                    .map(|m| format_file_size(m.len()))
                    .unwrap_or("Unknown".to_string());

                file_list.push_str(&format!(
                    r#"
                    <div class="file-card">

                        <div class="file-info">
                            <strong>{}</strong>
                            <p>{}</p>
                        </div>

                        <div class="actions">
                            <a class="download" href="/files/{}">
                                Download
                            </a>

                            <a class="delete" href="/delete/{}">
                                Delete
                            </a>
                        </div>

                    </div>
                    "#,
                    file_name,
                    size,
                    file_name,
                    file_name
                ));
            }
        }
    }

    if file_list.is_empty() {

        file_list = r#"
        <p class="empty">
            No files uploaded yet.
        </p>
        "#
        .to_string();
    }

    Html(page_html(&file_list))
}