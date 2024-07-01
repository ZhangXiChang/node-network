import Eyebrow from "./eyebrow";
import Face from "./face/mod";

export default function Window() {
    return (<div class="flex-1 flex flex-col bg-gray-1">
        <Eyebrow />
        <Face />
    </div>);
};
