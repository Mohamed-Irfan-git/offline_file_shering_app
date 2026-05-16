pub fn page_html(lan_url: &str, qr_svg: &str, host_is_local: bool) -> String {
    let alert_hidden = if host_is_local { "" } else { " hidden" };

    format!(
        r##"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0, viewport-fit=cover">
    <meta name="theme-color" content="#0a0c10">
    <meta name="apple-mobile-web-app-capable" content="yes">
    <title>LAN Share</title>
    <link rel="stylesheet" href="/static/style.css?v=4">
</head>
<body>
    <div class="app-shell">
        <header class="header">
            <div class="header-inner">
                <div class="brand">
                    <span class="brand-mark" aria-hidden="true">
                        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                            <path d="M8 12h8M12 8v8M4 16v2a2 2 0 002 2h12a2 2 0 002-2v-2M7 10l5-5 5 5M12 5v9"/>
                        </svg>
                    </span>
                    <div>
                        <h1>LAN Share</h1>
                        <p class="brand-sub">Fast local file transfer</p>
                    </div>
                </div>
                <div class="header-actions">
                    <span class="status-chip" id="serverStatus">
                        <span class="status-dot" aria-hidden="true"></span>
                        <span class="status-label">Connecting</span>
                    </span>
                    <button type="button" class="btn btn-outline btn-icon-text" id="copyLinkBtn" data-link="{lan_url}">
                        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" aria-hidden="true"><rect x="9" y="9" width="13" height="13" rx="2"/><path d="M5 15H4a2 2 0 01-2-2V4a2 2 0 012-2h9a2 2 0 012 2v1"/></svg>
                        Copy link
                    </button>
                </div>
            </div>
        </header>

        <div id="connectionBanner" class="banner banner-error" hidden role="alert"></div>

        <main class="main">
            <section class="view view-active" id="panel-share" data-panel="share">
                <div class="view-head">
                    <h2>Share &amp; connect</h2>
                    <p>Send anything on your Wi‑Fi — no internet needed.</p>
                </div>

                <ol class="steps" aria-label="How it works">
                    <li><span class="step-num">1</span><span>Run app on one computer</span></li>
                    <li><span class="step-num">2</span><span>Open link on other devices</span></li>
                    <li><span class="step-num">3</span><span>Upload &amp; download files</span></li>
                </ol>

                <div class="layout-split">
                    <article class="card card-glow">
                        <div class="card-label">
                            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" aria-hidden="true"><path d="M10 13a5 5 0 007.54.54l3-3a5 5 0 00-7.07-7.07l-1.72 1.71"/><path d="M14 11a5 5 0 00-7.54-.54l-3 3a5 5 0 007.07 7.07l1.71-1.71"/></svg>
                            Join link
                        </div>
                        <div class="connect-alert"{alert_hidden} id="connectAlert">
                            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" aria-hidden="true"><path d="M12 9v4m0 4h.01M10.29 3.86L1.82 18a2 2 0 001.71 3h16.94a2 2 0 001.71-3L13.71 3.86a2 2 0 00-3.42 0z"/></svg>
                            <span><strong>Phones:</strong> don&apos;t use localhost — use the LAN address below.</span>
                        </div>
                        <div class="url-field">
                            <code id="lanUrl">{lan_url}</code>
                            <button type="button" class="btn btn-ghost btn-sm" id="copyUrlInline" title="Copy address">Copy</button>
                        </div>
                        <ul id="altUrls" class="alt-urls"></ul>
                        <div class="qr-block">
                            <div class="qr-frame" id="qrFrame">{qr_svg}</div>
                            <p class="hint">Scan with your phone camera</p>
                        </div>
                    </article>

                    <article class="card">
                        <div class="card-label">
                            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" aria-hidden="true"><path d="M12 16V4m0 0l-4 4m4-4l4 4M4 20h16"/></svg>
                            Upload files
                        </div>
                        <label id="dropZone" class="dropzone" for="fileInput">
                            <span class="dropzone-icon" aria-hidden="true">
                                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5"><path d="M12 16V4m0 0l-4 4m4-4l4 4M4 20h16"/></svg>
                            </span>
                            <span class="dropzone-title">Tap to choose files</span>
                            <span class="dropzone-sub">Videos, photos, music, documents</span>
                        </label>
                        <input type="file" id="fileInput" class="file-input-native" multiple accept="*/*">
                        <label class="btn btn-outline btn-full pick-files-btn" for="fileInput">
                            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" aria-hidden="true"><path d="M22 19a2 2 0 01-2 2H4a2 2 0 01-2-2V5a2 2 0 012-2h5l2 3h9a2 2 0 012 2z"/></svg>
                            Browse files
                        </label>
                        <div id="uploadQueue" class="queue"></div>
                        <div class="progress-block" id="progressWrap" hidden>
                            <div class="progress-track"><div id="progressBar" class="progress-fill"></div></div>
                            <p id="statusText" class="progress-label"></p>
                        </div>
                        <button type="button" class="btn btn-primary btn-full btn-lg" id="uploadBtn" disabled>
                            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" aria-hidden="true"><path d="M12 19V5m0 0l-4 4m4-4l4 4"/></svg>
                            Start upload
                        </button>
                    </article>
                </div>
            </section>

            <section class="view" id="panel-files" data-panel="files">
                <div class="view-head view-head-row">
                    <div>
                        <h2>Shared files</h2>
                        <p>Available to everyone on this link</p>
                    </div>
                    <button type="button" class="btn btn-ghost btn-icon-only" id="refreshFiles" title="Refresh list" aria-label="Refresh">
                        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M23 4v6h-6M1 20v-6h6"/><path d="M3.51 9a9 9 0 0114.85-3.36L23 10M1 14l4.64 4.36A9 9 0 0020.49 15"/></svg>
                    </button>
                </div>
                <div class="search-bar">
                    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" aria-hidden="true"><circle cx="11" cy="11" r="8"/><path d="M21 21l-4.35-4.35"/></svg>
                    <input type="search" id="fileSearch" class="search-input" placeholder="Search files…" autocomplete="off">
                </div>
                <div id="fileList" class="file-list">
                    <div class="loading-state" id="filesLoading">
                        <span class="spinner" aria-hidden="true"></span>
                        <span>Loading files…</span>
                    </div>
                </div>
            </section>

            <section class="view" id="panel-devices" data-panel="devices">
                <div class="view-head view-head-row">
                    <div>
                        <h2>Devices</h2>
                        <p>Who is connected right now</p>
                    </div>
                    <button type="button" class="btn btn-ghost btn-sm" id="editMyDevice">
                        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" aria-hidden="true"><path d="M11 4H4a2 2 0 00-2 2v14a2 2 0 002 2h14a2 2 0 002-2v-7"/><path d="M18.5 2.5a2.12 2.12 0 013 3L12 15l-4 1 1-4 9.5-9.5z"/></svg>
                        Rename
                    </button>
                </div>
                <div id="myDeviceCard" class="my-device"></div>
                <div id="deviceList" class="device-list">
                    <div class="loading-state" id="devicesLoading">
                        <span class="spinner" aria-hidden="true"></span>
                        <span>Finding devices…</span>
                    </div>
                </div>
            </section>
        </main>

        <nav class="nav-bar" role="tablist" aria-label="Main">
            <button type="button" class="nav-item nav-active" data-tab="share" role="tab" aria-selected="true">
                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" aria-hidden="true"><path d="M4 12v8a2 2 0 002 2h12a2 2 0 002-2v-8M16 6l-4-4-4 4M12 2v13"/></svg>
                <span>Share</span>
            </button>
            <button type="button" class="nav-item" data-tab="files" role="tab">
                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" aria-hidden="true"><path d="M14 2H6a2 2 0 00-2 2v16a2 2 0 002 2h12a2 2 0 002-2V8z"/><path d="M14 2v6h6"/></svg>
                <span>Files</span>
                <em class="nav-badge" id="fileCountBadge">0</em>
            </button>
            <button type="button" class="nav-item" data-tab="devices" role="tab">
                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" aria-hidden="true"><rect x="2" y="3" width="20" height="14" rx="2"/><path d="M8 21h8M12 17v4"/></svg>
                <span>Devices</span>
                <em class="nav-badge" id="deviceCountBadge">0</em>
            </button>
        </nav>
    </div>

    <div id="toast" class="toast" role="status" aria-live="polite"></div>

    <dialog id="renameModal" class="dialog">
        <form id="renameForm" method="dialog" class="dialog-body">
            <h3>Rename this device</h3>
            <p class="dialog-desc">Others will see this name on the network.</p>
            <input type="text" id="renameInput" class="field" maxlength="48" required placeholder="My phone">
            <div class="dialog-actions">
                <button type="button" class="btn btn-ghost" id="renameCancel">Cancel</button>
                <button type="submit" class="btn btn-primary">Save</button>
            </div>
        </form>
    </dialog>

    <dialog id="deleteModal" class="dialog">
        <div class="dialog-body">
            <h3>Delete this file?</h3>
            <p class="dialog-desc" id="deleteFileName"></p>
            <div class="dialog-actions">
                <button type="button" class="btn btn-ghost" id="deleteCancel">Cancel</button>
                <button type="button" class="btn btn-danger" id="deleteConfirm">Delete</button>
            </div>
        </div>
    </dialog>

    <div id="imageModal" class="lightbox" hidden>
        <button type="button" class="lightbox-close" id="closeModal" aria-label="Close">×</button>
        <img id="modalImage" alt="Preview">
    </div>

    <script src="/static/app.js?v=4"></script>
</body>
</html>"##
    )
}
