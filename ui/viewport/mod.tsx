import CenterPanel from "./center-panel";
import SidePanel from "./side-panel";

export default function Viewport() {
    return <div class="absolute size-full flex bg-white">
        <SidePanel />
        <CenterPanel />
    </div>;
}
