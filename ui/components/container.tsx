import { children, JSX } from "solid-js";

export function Container(props: { children?: JSX.Element }) {
    const selfChildren = children(() => props.children);
    return <div class="">
        {selfChildren()}
    </div>;
}
