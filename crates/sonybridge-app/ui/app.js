// SonyBridge UI Logic

const invoke = window.__TAURI__ ? window.__TAURI__.core.invoke : async (cmd, args) => {
  console.log(`[Mock Tauri Invoke] ${cmd}`, args);
  return { status: "ok" };
};

// Preset frequency maps
const PRESETS = {
  Off: { bands: [0, 0, 0, 0, 0], bass: 0 },
  Excited: { bands: [-1, 2, 4, 3, 1], bass: 3 },
  Bright: { bands: [-2, 0, 2, 4, 5], bass: 0 },
  Mellow: { bands: [2, 1, -1, -2, -3], bass: 1 },
  Relaxed: { bands: [1, 2, 0, -1, 0], bass: 2 },
  Vocal: { bands: [-2, 3, 5, 2, -1], bass: -1 },
  TrebleBoost: { bands: [-3, -1, 1, 4, 6], bass: 0 },
  BassBoost: { bands: [4, 2, 0, 0, 0], bass: 6 },
  Speech: { bands: [-4, 4, 6, 1, -2], bass: -4 },
  Custom1: { bands: [0, 0, 0, 0, 0], bass: 0 },
  Custom2: { bands: [0, 0, 0, 0, 0], bass: 0 }
};

let currentAncMode = "NoiseCanceling";
let debounceTimer = null;

function debounce(func, delay = 75) {
  return (...args) => {
    clearTimeout(debounceTimer);
    debounceTimer = setTimeout(() => func(...args), delay);
  };
}

document.addEventListener("DOMContentLoaded", () => {
  // ANC Mode Buttons
  const modeButtons = document.querySelectorAll(".seg-btn");
  const ambientGroup = document.getElementById("ambient-slider-group");
  const ambientSlider = document.getElementById("ambient-slider");
  const ambientVal = document.getElementById("ambient-level-val");
  const voiceFocusCheck = document.getElementById("voice-focus-check");

  modeButtons.forEach(btn => {
    btn.addEventListener("click", () => {
      modeButtons.forEach(b => b.classList.remove("active"));
      btn.classList.add("active");
      currentAncMode = btn.dataset.mode;

      if (currentAncMode === "AmbientSound") {
        ambientGroup.style.opacity = "1";
        ambientGroup.style.pointerEvents = "auto";
      } else {
        ambientGroup.style.opacity = "0.4";
        ambientGroup.style.pointerEvents = "none";
      }

      sendAncUpdate();
    });
  });

  // Ambient Sound Slider
  ambientSlider.addEventListener("input", (e) => {
    const val = e.target.value;
    ambientVal.textContent = `Level ${val}`;
    debounceSendAncUpdate();
  });

  voiceFocusCheck.addEventListener("change", () => {
    debounceSendAncUpdate();
  });

  const debounceSendAncUpdate = debounce(sendAncUpdate, 75);

  function sendAncUpdate() {
    const payload = {
      mode: currentAncMode,
      level: parseInt(ambientSlider.value, 10),
      focusOnVoice: voiceFocusCheck.checked
    };
    invoke("set_anc_mode", payload).catch(console.error);
  }

  // Equalizer Controls
  const presetSelect = document.getElementById("eq-preset-select");
  const eqBands = [
    { slider: document.getElementById("eq-400"), label: document.getElementById("val-400") },
    { slider: document.getElementById("eq-1k"), label: document.getElementById("val-1k") },
    { slider: document.getElementById("eq-2k5"), label: document.getElementById("val-2k5") },
    { slider: document.getElementById("eq-6k3"), label: document.getElementById("val-6k3") },
    { slider: document.getElementById("eq-16k"), label: document.getElementById("val-16k") }
  ];
  const clearBassSlider = document.getElementById("clear-bass-slider");
  const clearBassVal = document.getElementById("clear-bass-val");

  function updateEqLabels() {
    eqBands.forEach(b => {
      const v = parseInt(b.slider.value, 10);
      b.label.textContent = v > 0 ? `+${v}` : `${v}`;
    });
    const bass = parseInt(clearBassSlider.value, 10);
    clearBassVal.textContent = bass > 0 ? `+${bass}` : `${bass}`;
  }

  presetSelect.addEventListener("change", (e) => {
    const preset = PRESETS[e.target.value];
    if (preset) {
      preset.bands.forEach((val, idx) => {
        eqBands[idx].slider.value = val;
      });
      clearBassSlider.value = preset.bass;
      updateEqLabels();
      sendEqUpdate();
    }
  });

  eqBands.forEach(b => {
    b.slider.addEventListener("input", () => {
      updateEqLabels();
      debounceSendEqUpdate();
    });
  });

  clearBassSlider.addEventListener("input", () => {
    updateEqLabels();
    debounceSendEqUpdate();
  });

  const debounceSendEqUpdate = debounce(sendEqUpdate, 75);

  function sendEqUpdate() {
    const payload = {
      preset: presetSelect.value,
      bands: eqBands.map(b => parseInt(b.slider.value, 10)),
      clearBass: parseInt(clearBassSlider.value, 10)
    };
    invoke("set_equalizer", payload).catch(console.error);
  }

  // Reconnect & Audio Enforce Buttons
  document.getElementById("btn-reconnect").addEventListener("click", () => {
    const pillText = document.getElementById("connection-text");
    pillText.textContent = "Reconnecting...";
    invoke("trigger_reconnect").then(() => {
      pillText.textContent = "Connected";
    }).catch(err => {
      console.error(err);
      pillText.textContent = "Disconnected";
    });
  });

  document.getElementById("btn-fix-audio").addEventListener("click", () => {
    const btn = document.getElementById("btn-fix-audio");
    btn.textContent = "Routing...";
    invoke("enforce_a2dp").then(() => {
      btn.textContent = "Enforced A2DP";
      setTimeout(() => { btn.textContent = "Enforce A2DP"; }, 2000);
    }).catch(console.error);
  });

  // Settings Toggles
  document.getElementById("touch-sensor-toggle").addEventListener("change", (e) => {
    invoke("set_touch_sensor", { enabled: e.target.checked }).catch(console.error);
  });

  document.getElementById("auto-off-select").addEventListener("change", (e) => {
    invoke("set_auto_off", { minutes: parseInt(e.target.value, 10) }).catch(console.error);
  });

  // Initialize labels
  updateEqLabels();
});
