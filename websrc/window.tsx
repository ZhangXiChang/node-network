import Eyebrow from "./eyebrow";
import Face from "./face/mod";

export default function Window() {
    return (<div class="flex-1 flex flex-col">
        <Eyebrow />
        <Face />
    </div>);
};
