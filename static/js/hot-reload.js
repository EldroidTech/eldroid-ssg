// Hot Reload Client Script
function initHotReload(port) {
    // WebSocket connection for hot reload
    const ws = new WebSocket(`ws://localhost:${port}/ws`);
    
    // Create error overlay container
    const errorOverlay = document.createElement('div');
    errorOverlay.style.cssText = `
        position: fixed;
        top: 0;
        left: 0;
        right: 0;
        background: rgba(200, 0, 0, 0.85);
        color: white;
        padding: 20px;
        font-family: monospace;
        font-size: 14px;
        z-index: 9999;
        display: none;
        white-space: pre-wrap;
        max-height: 50vh;
        overflow-y: auto;
    `;
    document.body.appendChild(errorOverlay);

    // Track loaded CSS files
    const loadedCssFiles = new Set();
    document.querySelectorAll('link[rel="stylesheet"]').forEach(link => {
        loadedCssFiles.add(link.href);
    });

    ws.onmessage = (event) => {
        try {
            const data = JSON.parse(event.data);
            
            if (data.type === 'css') {
                // Handle CSS hot reload
                const links = document.querySelectorAll('link[rel="stylesheet"]');
                links.forEach(link => {
                    if (link.href.includes(data.path)) {
                        // Add timestamp to force reload
                        const newHref = link.href.split('?')[0] + '?t=' + Date.now();
                        link.href = newHref;
                    }
                });
            } else if (data.type === 'error') {
                // Show error overlay
                errorOverlay.textContent = data.message;
                errorOverlay.style.display = 'block';
                // Auto-hide after 5 seconds
                setTimeout(() => {
                    errorOverlay.style.display = 'none';
                }, 5000);
            } else if (event.data === 'reload') {
                // Full page reload for other changes
                window.location.reload();
            }
        } catch (e) {
            // If not JSON, do a full reload
            if (event.data === 'reload') {
                window.location.reload();
            }
        }
    };

    ws.onclose = () => {
        // Try to reconnect every 1s
        setTimeout(() => initHotReload(port), 1000);
    };
}
