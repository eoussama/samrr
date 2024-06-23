const { invoke } = window.__TAURI__.tauri;
const { listen } = window.__TAURI__.event;

let testInputEl;
let testMsgEl;

async function test() {
  testMsgEl.textContent = await invoke("test", { value: testInputEl.value });
}

window.addEventListener("DOMContentLoaded", () => {
  testInputEl = document.querySelector("#test-input");
  testMsgEl = document.querySelector("#test-msg");

  document.querySelector("#test-form").addEventListener("submit", (e) => {
    e.preventDefault();
    test();
  });
});

function setupListener() {
  console.log('listened')

  listen('success', (event) => {
    console.log(event.payload); // Should print: Hello from Rust!
  });

  listen('error', (event) => {
    console.error(event.payload); // Should print: Hello from Rust!
  });
}

setupListener();