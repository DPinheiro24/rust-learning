const { invoke } = window.__TAURI__.core;
const { listen } = window.__TAURI__.event;

function updateUI(data) {
  const MB = 1024 * 1024;
  const totalMB = Math.round(data.total_memory / MB);
  const usedMB = Math.round(data.used_memory / MB);
  const usedPct = ((usedMB / totalMB) * 100).toFixed(1);

  document.getElementById("host-name").textContent = data.host_name;
  document.getElementById("system-name").textContent = data.system_name;
  document.getElementById("os-version").textContent = `Version ${data.os_version}`;
  document.getElementById("total-memory").textContent = `${totalMB} MB`;
  document.getElementById("used-memory").textContent = `${usedMB} MB (${usedPct}%)`;
  document.getElementById("memory-bar").style.width = `${usedPct}%`;

  const cpuGrid = document.getElementById("cpu-grid");
  cpuGrid.innerHTML = Object.entries(data.cpu_usage)
    .sort(([a], [b]) => a.localeCompare(b, undefined, { numeric: true }))
    .map(([name, usage]) => {
      const pct = parseFloat(usage).toFixed(1);
      return `
        <div class="cpu-row">
          <span class="cpu-name">${name}</span>
          <div class="memory-bar-track cpu-bar-track">
            <div class="memory-bar-fill" style="width:${pct}%"></div>
          </div>
          <span class="cpu-pct">${pct}%</span>
        </div>`;
    })
    .join("");
}

let notificationTimer = null;

function showNotification(text) {
  const el = document.getElementById("notification");
  el.textContent = text;
  el.classList.add("visible");

  clearTimeout(notificationTimer);
  notificationTimer = setTimeout(() => el.classList.remove("visible"), 3000);
}

window.addEventListener("DOMContentLoaded", async () => {
  await listen("sys-info", (event) => {
    updateUI(event.payload);
  });

  document.getElementById("pause-btn").addEventListener("click", async () => {
    const message = await invoke("switch_pause");
    showNotification(message);
  });
});
