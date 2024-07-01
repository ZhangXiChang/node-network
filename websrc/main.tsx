import { render } from "solid-js/web";
import Window from "./window";
import "virtual:uno.css"
import "./style.css"

window.addEventListener("contextmenu", (e) => e.preventDefault());
window.addEventListener("keydown", (e) => {
    if (e.key != "F12") {
        e.preventDefault();
    }
});

render(() => <Window />, document.body);
