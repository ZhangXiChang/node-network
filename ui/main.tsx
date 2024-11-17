import { render } from "solid-js/web";
import Window from "./window/mod";
import "virtual:uno.css";
import "./style.css";

window.addEventListener("contextmenu", (e) => e.preventDefault());
window.addEventListener("keydown", (e) => e.key != "F12" ? e.preventDefault() : null);

render(() => <Window />, document.body);
