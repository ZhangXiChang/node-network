import { render } from "solid-js/web";
import Window from "./window";
import "./styles.scss";

window.addEventListener("contextmenu", (e) => e.preventDefault());
window.addEventListener("selectstart", (e) => e.preventDefault());

render(() => <Window />, document.body);
