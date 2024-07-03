import { createRoot, createSignal } from "solid-js";
import Landing from "./landing";
import Home from "./home";
import { invoke } from "@tauri-apps/api/tauri";
import Error from "./error";

export default function Face() {
    const [face_content, set_face_content] = createSignal(<Landing />);
    invoke("connect_hubnode").then(() => createRoot(() => { set_face_content(<Home />) })).catch((err) => createRoot(() => set_face_content(<Error {...err} />)));
    return (<div class="flex-1 flex">
        {face_content()}
    </div>);
};
