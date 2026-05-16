const form = document.getElementById("uploadForm");

const fileInput = document.getElementById("fileInput");

const progressBox =
    document.querySelector(".progress-box");

const progressBar =
    document.getElementById("progressBar");

const statusText =
    document.getElementById("statusText");

form.addEventListener("submit", function (e) {

    e.preventDefault();

    const file = fileInput.files[0];

    if (!file) {

        statusText.textContent =
            "Please select a file.";

        return;
    }

    const formData = new FormData();

    formData.append("file", file);

    const xhr = new XMLHttpRequest();

    xhr.open("POST", "/upload", true);

    progressBox.style.display = "block";

    progressBar.style.width = "0%";

    statusText.textContent = "Uploading...";

    xhr.upload.onprogress = function (event) {

        if (event.lengthComputable) {

            const percent = Math.round(
                (event.loaded / event.total) * 100
            );

            progressBar.style.width =
                percent + "%";

            statusText.textContent =
                "Uploading " + percent + "%";
        }
    };

    xhr.onload = function () {
        if (
            xhr.status === 200 ||
            xhr.status === 303
        ) {

            statusText.textContent =
                "Upload completed.";

            window.location.reload();

        } else {

            statusText.textContent =
                "Upload failed.";
        }
    };

    xhr.onerror = function () {

        statusText.textContent =
            "Network error.";
    };

    xhr.send(formData);
});