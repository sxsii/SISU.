import { invoke } from "@tauri-apps/api/tauri";

async function loadSpecs() {
  try {
    const specs = await invoke("get_computer_specs");

    document.getElementById("os").textContent = specs.os;
    document.getElementById("cpu").textContent = specs.cpu;
    document.getElementById("ram").textContent = specs.ram;
    document.getElementById("storage").textContent = `${specs.storage_used} / ${specs.storage_total}`;
    document.getElementById("directx").textContent = specs.directx;
    document.getElementById("gpu").textContent = specs.gpu.join(", ");
    console.log("Works");
  } catch (e) {
    console.error("Failed to load specs:", e);
  }
}

document.addEventListener("DOMContentLoaded", loadSpecs);

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

// Toggle submenu on click
document.querySelectorAll(".has-submenu").forEach(item => {
  item.addEventListener("click", (e) => {
    e.stopPropagation(); 
    item.classList.toggle("open");
  });
});

// Page switching for leaf items
document.querySelectorAll(".sidebar nav li[data-page]").forEach(item => {
  item.addEventListener("click", (e) => {
    e.stopPropagation();

    document.querySelectorAll(".sidebar nav li").forEach(li => li.classList.remove("active"));
    item.classList.add("active");

    document.querySelectorAll(".page").forEach(p => p.classList.remove("active"));

    const pageId = item.getAttribute("data-page");
    document.getElementById(pageId).classList.add("active");
  });
});

