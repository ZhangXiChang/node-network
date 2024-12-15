import { children, JSX } from "solid-js";

export function WindowFrame(props: { children?: JSX.Element }) {
    const selfChildren = children(() => props.children);
    return <div class="absolute size-full flex flex-col bg-white">
        {selfChildren()}
    </div>;
}
