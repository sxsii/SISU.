const { invoke } = window.__TAURI__.core;

let greetInputEl;
let greetMsgEl;

/*
async function greet() {
  // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  greetMsgEl.textContent = await invoke("greet", { name: greetInputEl.value });
}
*/

document.querySelectorAll(".sidebar nav li").forEach(item => {
  item.addEventListener("click", () => {
    // remove "active" from all nav items
    document.querySelectorAll(".sidebar nav li").forEach(li => li.classList.remove("active"));
    item.classList.add("active");

    // hide all pages
    document.querySelectorAll(".page").forEach(p => p.classList.remove("active"));

    // show selected page
    const pageId = item.getAttribute("data-page");
    document.getElementById(pageId).classList.add("active");
  });
});
