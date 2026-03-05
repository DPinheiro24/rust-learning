const { invoke } = window.__TAURI__.core;

// DOM Elements
let noteTitleEl;
let noteContentEl;
let saveMsgEl;
let notesListEl;
let noteViewerEl;
let currentNoteTitleEl;
let currentNoteContentEl;

// Save a new note
async function saveNote(e) {
  e.preventDefault();

  const title = noteTitleEl.value.trim();
  const content = noteContentEl.value.trim();

  if (!title || !content) {
    saveMsgEl.textContent = "Please fill in both title and content";
    return;
  }

  try {
    const result = await invoke("save_note", { title, content });
    saveMsgEl.textContent = result;
    saveMsgEl.style.color = "green";

    // Clear form
    noteTitleEl.value = "";
    noteContentEl.value = "";

    // Refresh notes list
    await loadNotes();
  } catch (error) {
    saveMsgEl.textContent = `Error: ${error}`;
    saveMsgEl.style.color = "red";
  }
}

// Load all notes
async function loadNotes() {
  try {
    const notes = await invoke("get_notes");

    if (notes.length === 0) {
      notesListEl.innerHTML = "<p>No notes yet. Create your first note above!</p>";
      return;
    }

    // Create list of notes
    notesListEl.innerHTML = notes
      .map(title => `
        <div class="note-item" data-title="${title}">
          <span>${title}</span>
          <button class="read-btn" data-title="${title}">Read</button>
        </div>
      `)
      .join("");

    // Add click handlers to read buttons
    document.querySelectorAll(".read-btn").forEach(btn => {
      btn.addEventListener("click", async (e) => {
        const title = e.target.dataset.title;
        await readNote(title);
      });
    });
  } catch (error) {
    notesListEl.innerHTML = `<p style="color: red;">Error loading notes: ${error}</p>`;
  }
}

// Read a specific note
async function readNote(title) {
  try {
    const content = await invoke("read_note", { title });

    currentNoteTitleEl.textContent = title;
    currentNoteContentEl.textContent = content;
    noteViewerEl.style.display = "block";
  } catch (error) {
    alert(`Error reading note: ${error}`);
  }
}

// Initialize app
window.addEventListener("DOMContentLoaded", () => {
  // Get DOM elements
  noteTitleEl = document.querySelector("#note-title");
  noteContentEl = document.querySelector("#note-content");
  saveMsgEl = document.querySelector("#save-msg");
  notesListEl = document.querySelector("#notes-list");
  noteViewerEl = document.querySelector("#note-viewer");
  currentNoteTitleEl = document.querySelector("#current-note-title");
  currentNoteContentEl = document.querySelector("#current-note-content");

  // Event listeners
  document.querySelector("#note-form").addEventListener("submit", saveNote);
  document.querySelector("#refresh-btn").addEventListener("click", loadNotes);
  document.querySelector("#close-note-btn").addEventListener("click", () => {
    noteViewerEl.style.display = "none";
  });

  // Load notes on startup
  loadNotes();
});
