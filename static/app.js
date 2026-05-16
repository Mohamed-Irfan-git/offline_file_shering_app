(function () {
    "use strict";

    const $ = (sel) => document.querySelector(sel);
    const $$ = (sel) => document.querySelectorAll(sel);

    const dropZone = $("#dropZone");
    const fileInput = $("#fileInput");
    const uploadBtn = $("#uploadBtn");
    const uploadQueue = $("#uploadQueue");
    const progressWrap = $("#progressWrap");
    const progressBar = $("#progressBar");
    const statusText = $("#statusText");
    const fileList = $("#fileList");
    const fileSearch = $("#fileSearch");
    const deviceList = $("#deviceList");
    const myDeviceCard = $("#myDeviceCard");
    const fileCountBadge = $("#fileCountBadge");
    const deviceCountBadge = $("#deviceCountBadge");
    const copyLinkBtn = $("#copyLinkBtn");
    const copyUrlInline = $("#copyUrlInline");
    const lanUrlEl = $("#lanUrl");
    const renameModal = $("#renameModal");
    const renameForm = $("#renameForm");
    const renameInput = $("#renameInput");
    const renameCancel = $("#renameCancel");
    const editMyDevice = $("#editMyDevice");
    const deleteModal = $("#deleteModal");
    const deleteFileName = $("#deleteFileName");
    const deleteCancel = $("#deleteCancel");
    const deleteConfirm = $("#deleteConfirm");
    const imageModal = $("#imageModal");
    const modalImage = $("#modalImage");
    const closeModal = $("#closeModal");
    const toast = $("#toast");
    const refreshFiles = $("#refreshFiles");
    const connectAlert = $("#connectAlert");
    const altUrls = $("#altUrls");
    const qrFrame = $("#qrFrame");
    const connectionBanner = $("#connectionBanner");
    const serverStatus = $("#serverStatus");
    const statusLabel = serverStatus
        ? serverStatus.querySelector(".status-label")
        : null;
    const filesLoading = $("#filesLoading");
    const devicesLoading = $("#devicesLoading");

    let shareUrl = "";
    let pendingFiles = [];
    let allFiles = [];
    let deleteTarget = null;
    let toastTimer = null;
    let connected = false;

    let deviceId = storageGet("deviceId");
    let deviceName = storageGet("deviceName");
    let deviceMeta = null;

    const isMobile =
        /Android|iPhone|iPad|iPod|Mobile/i.test(navigator.userAgent);

    function storageGet(key) {
        try {
            return localStorage.getItem(key);
        } catch {
            return null;
        }
    }

    function storageSet(key, value) {
        try {
            localStorage.setItem(key, value);
        } catch {
            /* private mode */
        }
    }

    /** Works on http:// LAN (crypto.randomUUID needs HTTPS except localhost). */
    function createId() {
        if (
            typeof crypto !== "undefined" &&
            typeof crypto.randomUUID === "function"
        ) {
            try {
                return crypto.randomUUID();
            } catch {
                /* not a secure context */
            }
        }
        return (
            "d-" +
            Date.now().toString(36) +
            "-" +
            Math.random().toString(36).slice(2, 10)
        );
    }

    function apiUrl(path) {
        return new URL(path, window.location.origin).href;
    }

    function showToast(message, type) {
        toast.textContent = message;
        toast.classList.remove("toast-success", "toast-error");
        if (type === "success") toast.classList.add("toast-success");
        if (type === "error") toast.classList.add("toast-error");
        toast.classList.add("show");
        clearTimeout(toastTimer);
        toastTimer = setTimeout(() => {
            toast.classList.remove("show");
        }, 3000);
    }

    function emptyCard(icon, title, hint) {
        return `
            <div class="empty-card">
                <span class="empty-icon">${icon}</span>
                <p>${escapeHtml(title)}</p>
                <span class="empty-hint">${hint}</span>
            </div>
        `;
    }

    async function copyText(text) {
        try {
            await navigator.clipboard.writeText(text);
            showToast("Link copied", "success");
            return true;
        } catch {
            showToast("Could not copy link", "error");
            return false;
        }
    }

    function detectDevice() {
        const ua = navigator.userAgent;
        let deviceType = "desktop";
        let platform = "Unknown";
        let browser = "Browser";

        if (/Mobi|Android/i.test(ua) && !/Tablet|iPad/i.test(ua)) {
            deviceType = "mobile";
        } else if (/Tablet|iPad/i.test(ua)) {
            deviceType = "tablet";
        }

        if (/Windows NT/i.test(ua)) platform = "Windows";
        else if (/Mac OS X|Macintosh/i.test(ua)) platform = "macOS";
        else if (/Android/i.test(ua)) platform = "Android";
        else if (/iPhone|iPad|iPod/i.test(ua)) platform = "iOS";
        else if (/Linux/i.test(ua)) platform = "Linux";
        else if (/CrOS/i.test(ua)) platform = "Chrome OS";

        if (/Edg\//i.test(ua)) browser = "Edge";
        else if (/OPR\/|Opera/i.test(ua)) browser = "Opera";
        else if (/Firefox\//i.test(ua)) browser = "Firefox";
        else if (/Chrome\//i.test(ua) && !/Edg/i.test(ua)) browser = "Chrome";
        else if (/Safari/i.test(ua) && !/Chrome/i.test(ua)) browser = "Safari";

        const defaultName = `${platform} · ${browser}`;

        return { deviceType, platform, browser, defaultName };
    }

    function deviceIcon(type) {
        if (type === "mobile") return "📱";
        if (type === "tablet") return "📲";
        return "💻";
    }

    function formatBytes(bytes) {
        if (!isFinite(bytes) || bytes <= 0) return "0 B";
        if (bytes < 1024) return bytes.toFixed(0) + " B";
        if (bytes < 1024 * 1024) return (bytes / 1024).toFixed(1) + " KB";
        if (bytes < 1024 * 1024 * 1024) return (bytes / (1024 * 1024)).toFixed(1) + " MB";
        return (bytes / (1024 * 1024 * 1024)).toFixed(1) + " GB";
    }

    function formatTime(ts) {
        if (!ts) return "";
        const d = new Date(ts * 1000);
        const now = new Date();
        const diff = (now - d) / 1000;
        if (diff < 60) return "Just now";
        if (diff < 3600) return Math.floor(diff / 60) + "m ago";
        if (diff < 86400) return Math.floor(diff / 3600) + "h ago";
        return d.toLocaleDateString();
    }

    function initDevice() {
        deviceMeta = detectDevice();

        if (!deviceId) {
            deviceId = createId();
            storageSet("deviceId", deviceId);
        }

        if (!deviceName) {
            deviceName = deviceMeta.defaultName;
            storageSet("deviceName", deviceName);
        }
    }

    function setConnectionState(ok, message) {
        connected = ok;

        if (serverStatus) {
            serverStatus.classList.toggle("error", !ok);
            if (statusLabel) {
                statusLabel.textContent = ok ? "Connected" : "Offline";
            }
        }

        if (connectionBanner) {
            if (!ok && message) {
                connectionBanner.hidden = false;
                connectionBanner.classList.remove("ok");
                connectionBanner.textContent = message;
            } else {
                connectionBanner.hidden = true;
            }
        }
    }

    async function checkConnection() {
        if (isLocalHost() && isMobile) {
            setConnectionState(
                false,
                "Wrong address on phone. Open http://192.168.x.x:5000 from your laptop screen, not localhost."
            );
            return false;
        }

        try {
            const res = await fetch(apiUrl("/api/ping"), {
                cache: "no-store",
            });
            if (!res.ok) throw new Error("ping failed");
            setConnectionState(true, "");
            if (isMobile) {
                showToast("Connected — tap Browse files to upload", "success");
            }
            return true;
        } catch {
            setConnectionState(
                false,
                "Cannot reach server. Same Wi-Fi? On phone use the laptop IP (e.g. http://192.168.1.5:5000), not localhost."
            );
            return false;
        }
    }

    async function registerDevice() {
        if (!connected) return;

        try {
            await fetch(apiUrl("/api/devices/register"), {
                method: "POST",
                headers: { "Content-Type": "application/json" },
                body: JSON.stringify({
                    id: deviceId,
                    name: deviceName,
                    device_type: deviceMeta.deviceType,
                    platform: deviceMeta.platform,
                    browser: deviceMeta.browser,
                }),
            });
        } catch {
            /* retry on next interval */
        }
    }

    async function renameDevice(name) {
        const res = await fetch("/api/devices/rename", {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify({ id: deviceId, name }),
        });

        if (res.ok) {
            deviceName = name.trim();
            storageSet("deviceName", deviceName);
            showToast("Device renamed");
            registerDevice();
            loadDevices();
            return true;
        }

        showToast("Could not rename device");
        return false;
    }

    function renderMyDevice() {
        myDeviceCard.innerHTML = `
            <div class="device-row is-self">
                <div class="device-avatar">${deviceIcon(deviceMeta.deviceType)}</div>
                <div class="device-info">
                    <strong>${escapeHtml(deviceName)}</strong>
                    <span>${escapeHtml(deviceMeta.platform)} · ${escapeHtml(deviceMeta.browser)} · This device</span>
                </div>
                <span class="device-badge">You</span>
                <span class="device-online" title="Online"></span>
            </div>
        `;
    }

    function escapeHtml(str) {
        const d = document.createElement("div");
        d.textContent = str;
        return d.innerHTML;
    }

    async function loadDevices() {
        if (!connected) {
            deviceList.innerHTML = emptyCard(
                "📡",
                "Not connected",
                "Open the LAN link from your computer on this phone."
            );
            return;
        }

        deviceList.innerHTML =
            '<div class="loading-state"><span class="spinner" aria-hidden="true"></span><span>Finding devices…</span></div>';

        try {
            const res = await fetch(
                apiUrl(
                    "/api/devices?exclude=" + encodeURIComponent(deviceId)
                ),
                { cache: "no-store" }
            );
            if (!res.ok) throw new Error("devices failed");
            const devices = await res.json();
            const others = devices.filter((d) => d.id !== deviceId);

            deviceCountBadge.textContent = String(others.length + 1);

            renderMyDevice();

            if (others.length === 0) {
                deviceList.innerHTML = emptyCard(
                    "👋",
                    "Just you for now",
                    "Open the same link on another phone or laptop on this Wi‑Fi."
                );
                return;
            }

            deviceList.innerHTML = others
                .map(
                    (d) => `
                <div class="device-row">
                    <div class="device-avatar">${deviceIcon(d.device_type)}</div>
                    <div class="device-info">
                        <strong>${escapeHtml(d.name)}</strong>
                        <span>${escapeHtml(d.platform)} · ${escapeHtml(d.browser)}</span>
                    </div>
                    <span class="device-online" title="Online"></span>
                </div>
            `
                )
                .join("");
        } catch {
            deviceList.innerHTML = emptyCard(
                "⚠️",
                "Could not load devices",
                "Check your connection and pull to refresh."
            );
        }
    }

    async function loadFiles() {
        if (!connected) {
            fileList.innerHTML = emptyCard(
                "📁",
                "Not connected",
                "Use the LAN address from your computer to see files here."
            );
            return;
        }

        fileList.innerHTML =
            '<div class="loading-state"><span class="spinner" aria-hidden="true"></span><span>Loading files…</span></div>';

        try {
            const res = await fetch(apiUrl("/api/files"), {
                cache: "no-store",
            });
            if (!res.ok) throw new Error("files failed");
            allFiles = await res.json();
            fileCountBadge.textContent = String(allFiles.length);
            renderFiles();
        } catch {
            fileList.innerHTML = emptyCard(
                "⚠️",
                "Could not load files",
                "Same Wi‑Fi? Tap the refresh button to try again."
            );
        }
    }

    function renderFiles() {
        const query = (fileSearch.value || "").trim().toLowerCase();
        const filtered = query
            ? allFiles.filter(
                  (f) =>
                      f.display_name.toLowerCase().includes(query) ||
                      f.name.toLowerCase().includes(query)
              )
            : allFiles;

        if (filtered.length === 0) {
            fileList.innerHTML = query
                ? emptyCard("🔍", "No matches", "Try a different search term.")
                : emptyCard(
                      "📂",
                      "No files yet",
                      "Go to Share, pick files, then tap Start upload."
                  );
            return;
        }

        fileList.innerHTML = filtered
            .map((f) => {
                const thumb = f.is_image
                    ? `<img src="/files/${encodeURIComponent(f.name)}" alt="" data-full="/files/${encodeURIComponent(f.name)}" class="preview-img">`
                    : f.icon;

                return `
                <div class="file-row" data-name="${escapeHtml(f.name)}">
                    <div class="file-thumb">${thumb}</div>
                    <div class="file-meta">
                        <strong>${escapeHtml(f.display_name)}</strong>
                        <span>${escapeHtml(f.size)} · ${formatTime(f.modified)}</span>
                    </div>
                    <div class="file-actions">
                        <a class="dl" href="/download/${encodeURIComponent(f.name)}" download>Download</a>
                        <button type="button" class="rm" data-delete="${escapeHtml(f.name)}">Delete</button>
                    </div>
                </div>
            `;
            })
            .join("");

        fileList.querySelectorAll(".preview-img").forEach((img) => {
            img.addEventListener("click", (e) => {
                e.preventDefault();
                e.stopPropagation();
                modalImage.src = img.dataset.full;
                imageModal.hidden = false;
            });
        });

        fileList.querySelectorAll("[data-delete]").forEach((btn) => {
            btn.addEventListener("click", () => {
                deleteTarget = btn.dataset.delete;
                const file = allFiles.find((f) => f.name === deleteTarget);
                deleteFileName.textContent = file
                    ? file.display_name
                    : deleteTarget;
                deleteModal.showModal();
            });
        });
    }

    function renderQueue() {
        uploadQueue.innerHTML = "";
        pendingFiles.forEach((file, index) => {
            const row = document.createElement("div");
            row.className = "queue-item";
            row.innerHTML = `
                <span>${escapeHtml(file.name)}</span>
                <small>${formatBytes(file.size)}</small>
                <button type="button" class="queue-remove" data-index="${index}" aria-label="Remove">×</button>
            `;
            uploadQueue.appendChild(row);
        });

        uploadBtn.disabled = pendingFiles.length === 0;

        uploadQueue.querySelectorAll(".queue-remove").forEach((btn) => {
            btn.addEventListener("click", (e) => {
                e.stopPropagation();
                const i = parseInt(btn.dataset.index, 10);
                pendingFiles.splice(i, 1);
                renderQueue();
            });
        });
    }

    function addFiles(fileListObj) {
        const added = Array.from(fileListObj);
        if (added.length === 0) return;
        pendingFiles.push(...added);
        renderQueue();
        showToast(added.length + " file(s) ready — tap Start upload", "success");
    }

    function isLocalHost() {
        const h = window.location.hostname;
        return h === "localhost" || h === "127.0.0.1" || h === "::1";
    }

    function uploadSingleFile(file, index, total) {
        return new Promise((resolve, reject) => {
            const formData = new FormData();
            formData.append("file", file);

            const xhr = new XMLHttpRequest();
            xhr.open("POST", apiUrl("/upload"), true);
            xhr.timeout = 0;

            const startTime = Date.now();

            xhr.upload.onprogress = (event) => {
                if (event.lengthComputable) {
                    const fileRatio = event.loaded / event.total;
                    const overall = ((index - 1) + fileRatio) / total;
                    const percent = Math.round(overall * 100);
                    const elapsed = (Date.now() - startTime) / 1000;
                    const speed = event.loaded / Math.max(elapsed, 0.1);

                    progressBar.style.width = percent + "%";
                    statusText.textContent =
                        `File ${index}/${total}: ${file.name} — ${percent}% · ${formatBytes(speed)}/s`;
                }
            };

            xhr.onload = () => {
                if (xhr.status >= 200 && xhr.status < 300) {
                    resolve();
                } else {
                    reject(new Error("upload failed"));
                }
            };

            xhr.onerror = () => reject(new Error("network"));
            xhr.send(formData);
        });
    }

    async function uploadFiles() {
        if (pendingFiles.length === 0) return;

        const files = pendingFiles.slice();
        const total = files.length;

        progressWrap.hidden = false;
        progressBar.style.width = "0%";
        uploadBtn.disabled = true;
        statusText.textContent = "Uploading…";

        let uploaded = 0;

        try {
            for (let i = 0; i < files.length; i++) {
                await uploadSingleFile(files[i], i + 1, total);
                uploaded++;
            }

            pendingFiles = [];
            fileInput.value = "";
            renderQueue();
                showToast(`${uploaded} file(s) uploaded`, "success");
            statusText.textContent = "";
            await loadFiles();
            switchTab("files");
        } catch {
            statusText.textContent = "Upload failed.";
            showToast("Upload failed — use the LAN link, same Wi-Fi", "error");
        } finally {
            progressWrap.hidden = true;
            uploadBtn.disabled = false;
            progressBar.style.width = "0%";
        }
    }

    function applyNetworkInfo(net) {
        if (!isLocalHost()) {
            shareUrl = window.location.origin;
        } else {
            shareUrl = net.share_url;
        }

        lanUrlEl.textContent = shareUrl;
        copyLinkBtn.dataset.link = shareUrl;

        if (qrFrame && net.qr_svg) {
            qrFrame.innerHTML = net.qr_svg;
        }

        if (connectAlert) {
            connectAlert.hidden = !(net.host_is_local || isLocalHost());
        }

        if (altUrls) {
            const others = (net.urls || []).filter((u) => u !== shareUrl);
            altUrls.innerHTML =
                others.length === 0
                    ? ""
                    : others
                          .map(
                              (u) =>
                                  `<li><a href="${escapeHtml(u)}">${escapeHtml(u)}</a></li>`
                          )
                          .join("");
        }
    }

    async function initNetwork() {
        try {
            const res = await fetch(apiUrl("/api/network"), {
                cache: "no-store",
            });
            if (!res.ok) throw new Error();
            const net = await res.json();
            applyNetworkInfo(net);
        } catch {
            if (!isLocalHost()) {
                shareUrl = window.location.origin;
                lanUrlEl.textContent = shareUrl;
                copyLinkBtn.dataset.link = shareUrl;
            }
        }
    }

    async function deleteFile() {
        if (!deleteTarget) return;

        const res = await fetch(
            `/api/files/${encodeURIComponent(deleteTarget)}`,
            { method: "DELETE" }
        );

        deleteModal.close();
        deleteTarget = null;

        if (res.ok) {
            showToast("File deleted");
            await loadFiles();
        } else {
            showToast("Could not delete file");
        }
    }

    function switchTab(tabId) {
        $$(".nav-item, .tab").forEach((t) => {
            const active = t.dataset.tab === tabId;
            t.classList.toggle("nav-active", active);
            t.classList.toggle("active", active);
            t.setAttribute("aria-selected", active ? "true" : "false");
        });

        $$(".view, .panel").forEach((p) => {
            const active = p.dataset.panel === tabId;
            p.classList.toggle("view-active", active);
            p.classList.toggle("active", active);
        });
    }

    function setupTabs() {
        $$(".nav-item, .tab").forEach((tab) => {
            tab.addEventListener("click", () => {
                const id = tab.dataset.tab;
                switchTab(id);
                if (id === "files") loadFiles();
                if (id === "devices") {
                    registerDevice();
                    loadDevices();
                }
            });
        });
    }

    function setupUpload() {
        if (!fileInput) return;

        if (!dropZone) return;

        dropZone.addEventListener("dragover", (e) => {
            e.preventDefault();
            dropZone.classList.add("active");
        });

        dropZone.addEventListener("dragleave", () => {
            dropZone.classList.remove("active");
        });

        dropZone.addEventListener("drop", (e) => {
            e.preventDefault();
            dropZone.classList.remove("active");
            if (e.dataTransfer.files.length) {
                addFiles(e.dataTransfer.files);
            }
        });

        fileInput.addEventListener("change", () => {
            if (fileInput.files && fileInput.files.length) {
                addFiles(fileInput.files);
            }
            fileInput.value = "";
        });

        uploadBtn.addEventListener("click", uploadFiles);
    }

    async function refreshAll() {
        await checkConnection();
        await initNetwork();
        await registerDevice();
        await loadDevices();
        await loadFiles();
    }

    function getShareUrl() {
        return (
            shareUrl ||
            lanUrlEl?.textContent?.trim() ||
            copyLinkBtn?.dataset?.link ||
            window.location.origin
        );
    }

    function setupCopy() {
        copyLinkBtn.addEventListener("click", async () => {
            const ok = await copyText(getShareUrl());
            if (ok) {
                copyLinkBtn.classList.add("copied");
                setTimeout(() => copyLinkBtn.classList.remove("copied"), 1500);
            }
        });

        copyUrlInline.addEventListener("click", () => copyText(getShareUrl()));
    }

    function setupRename() {
        editMyDevice.addEventListener("click", () => {
            renameInput.value = deviceName;
            renameModal.showModal();
            renameInput.focus();
            renameInput.select();
        });

        renameCancel.addEventListener("click", () => renameModal.close());

        renameForm.addEventListener("submit", async (e) => {
            e.preventDefault();
            const name = renameInput.value.trim();
            if (!name) return;
            const ok = await renameDevice(name);
            if (ok) renameModal.close();
        });
    }

    function setupDelete() {
        deleteCancel.addEventListener("click", () => {
            deleteTarget = null;
            deleteModal.close();
        });

        deleteConfirm.addEventListener("click", deleteFile);
    }

    function closeLightbox() {
        imageModal.hidden = true;
        modalImage.removeAttribute("src");
    }

    function setupLightbox() {
        imageModal.hidden = true;

        closeModal.addEventListener("click", closeLightbox);

        imageModal.addEventListener("click", (e) => {
            if (e.target === imageModal) {
                closeLightbox();
            }
        });

        document.addEventListener("keydown", (e) => {
            if (e.key === "Escape" && !imageModal.hidden) {
                closeLightbox();
            }
        });
    }

    fileSearch.addEventListener("input", renderFiles);
    refreshFiles.addEventListener("click", loadFiles);

    initDevice();
    setupTabs();
    setupUpload();
    setupCopy();
    setupRename();
    setupDelete();
    setupLightbox();

    refreshAll();

    document.addEventListener("visibilitychange", () => {
        if (!document.hidden) refreshAll();
    });

    setInterval(() => {
        if (!document.hidden) {
            registerDevice();
            loadDevices();
        }
    }, 4000);

    setInterval(() => {
        if (!document.hidden) loadFiles();
    }, 6000);
})();
