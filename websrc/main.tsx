import { render } from "solid-js/web";
import Window from "./window";
import "virtual:uno.css"
import "./style.css"

window.oncontextmenu = (e) => e.preventDefault();
window.onkeydown = (e) => e.key != "F12" ? e.preventDefault() : null;

render(() => <Window />, document.body);
