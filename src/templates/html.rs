pub fn page_html(file_list: &str) -> String {
    format!(
        r#"
<!DOCTYPE html>
<html>

<head>

    <title>LAN Share</title>

    <meta charset="UTF-8">

    <meta
        name="viewport"
        content="width=device-width, initial-scale=1.0"
    >

    <link
        rel="stylesheet"
        href="/static/style.css"
    >

</head>

<body>

    <div class="container">

        <div class="hero">

            <h1>LAN Share</h1>

            <p>
                Send and receive files offline
                inside the same Wi-Fi or LAN.
            </p>

            <form id="uploadForm">

                <input
                    id="fileInput"
                    type="file"
                    name="file"
                    required
                >

                <button type="submit">
                    Upload File
                </button>

                <div class="progress-box">
                    <div id="progressBar"></div>
                </div>

                <p id="statusText"></p>

            </form>

        </div>

        <div class="files">

            <h2>Uploaded Files</h2>

            {}

        </div>

    </div>

    <script src="/static/app.js"></script>

</body>

</html>
"#,
        file_list
    )
}