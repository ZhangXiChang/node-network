import { render } from "solid-js/web";
import Window from "./window/mod";
import "virtual:uno.css";
import "./style.css";
import { getCurrentWindow } from "@tauri-apps/api/window";

export const mainWindow = getCurrentWindow();

window.addEventListener("contextmenu", (e) => e.preventDefault());
window.addEventListener("keydown", (e) => e.key != "F12" ? e.preventDefault() : null);

render(() => <Window />, document.body);
