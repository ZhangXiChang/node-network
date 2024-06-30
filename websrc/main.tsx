import { render } from "solid-js/web";
import Window from "./window";
import "./tailwind.css"

window.addEventListener("contextmenu", (e) => e.preventDefault());

render(() => <Window />, document.body);
