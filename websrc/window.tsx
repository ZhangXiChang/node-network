import Eyebrow from "./eyebrow";
import Face from "./face/mod";

export default function Window() {
    return (<div class="absolute size-full flex flex-col rounded bg-gray-100">
        <Eyebrow />
        <Face />
    </div>);
};
