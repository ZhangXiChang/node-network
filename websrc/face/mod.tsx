import { createSignal } from "solid-js";
import Landing from "./landing";
import HubNodeBrowser from "./hub-node-browser";

export default function Face() {
    const [face_content, set_face_content] = createSignal(<Landing />);
    set_face_content(<HubNodeBrowser />);
    return (<div class="Face">
        {face_content()}
    </div>);
};
