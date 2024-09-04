import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import Window from "./window";

window.addEventListener("contextmenu", (e) => e.preventDefault());
window.addEventListener("keydown", (e) => e.key != "F12" ? e.preventDefault() : null);

createRoot(document.getElementById("window")!).render(<StrictMode><Window /></StrictMode>);
