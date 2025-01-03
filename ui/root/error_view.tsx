import { WindowBrow } from "../components/window/brow";

export function ErrorView(props: { err: Error }) {
    return <>
        <WindowBrow title="错误视图" />
        <div>{props.err.message}</div>
    </>;
}
