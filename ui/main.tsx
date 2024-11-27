import { render } from "solid-js/web";
import "virtual:uno.css";
import "./style.css";
import Viewport from "./viewport/mod";

window.addEventListener("contextmenu", (e) => e.preventDefault());
window.addEventListener("keydown", (e) => e.key != "F12" ? e.preventDefault() : null);

render(() => <Viewport />, document.body);
