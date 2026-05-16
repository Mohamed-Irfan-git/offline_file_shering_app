pub fn get_file_icon(file_name: &str) -> &'static str {
    let lower = file_name.to_lowercase();

    if lower.ends_with(".png")
        || lower.ends_with(".jpg")
        || lower.ends_with(".jpeg")
        || lower.ends_with(".gif")
        || lower.ends_with(".webp")
        || lower.ends_with(".bmp")
        || lower.ends_with(".svg")
    {
        "🖼️"
    } else if lower.ends_with(".mp4")
        || lower.ends_with(".mkv")
        || lower.ends_with(".avi")
        || lower.ends_with(".mov")
        || lower.ends_with(".wmv")
        || lower.ends_with(".webm")
        || lower.ends_with(".m4v")
        || lower.ends_with(".flv")
    {
        "🎬"
    } else if lower.ends_with(".mp3")
        || lower.ends_with(".wav")
        || lower.ends_with(".m4a")
        || lower.ends_with(".flac")
        || lower.ends_with(".aac")
        || lower.ends_with(".ogg")
        || lower.ends_with(".wma")
    {
        "🎵"
    } else if lower.ends_with(".pdf") {
        "📕"
    } else if lower.ends_with(".zip")
        || lower.ends_with(".rar")
        || lower.ends_with(".7z")
        || lower.ends_with(".tar")
        || lower.ends_with(".gz")
    {
        "🗜️"
    } else if lower.ends_with(".doc")
        || lower.ends_with(".docx")
        || lower.ends_with(".txt")
        || lower.ends_with(".rtf")
    {
        "📝"
    } else if lower.ends_with(".xls")
        || lower.ends_with(".xlsx")
        || lower.ends_with(".csv")
    {
        "📊"
    } else if lower.ends_with(".apk")
        || lower.ends_with(".exe")
        || lower.ends_with(".dmg")
        || lower.ends_with(".deb")
    {
        "⚙️"
    } else {
        "📄"
    }
}
