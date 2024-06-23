const { invoke } = window.__TAURI__.tauri;

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
