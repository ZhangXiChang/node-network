import { invoke } from "@tauri-apps/api/tauri";
import { createRoot, createSignal } from "solid-js";
import Landing from "./landing";
import Home from "./home";
import Error from "./error";

export default function Face() {
    const [faceContent, setFaceContent] = createSignal(<Landing />);
    (async () => {
        try {
            await invoke("connect_server");
            setFaceContent(createRoot(() => <Home />));
        } catch (err) {
            setFaceContent(createRoot(() => <Error err={err} />))
        }
    })();
    return <div class="flex-1 flex">
        {faceContent()}
    </div>;
};
