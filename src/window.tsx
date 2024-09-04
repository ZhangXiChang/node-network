import TitleBar from "./title-bar";
import Viewport from "./viewport";

export default function Window() {
    return <div class="absolute size-full flex flex-col bg-white">
        <TitleBar />
        <Viewport />
    </div>;
}
