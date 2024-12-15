import { ErrorBoundary } from "solid-js";
import { WindowBrow, WindowFrame } from "../components/window";
import { ErrorView } from "./error_view";

export function Root() {
    return <WindowFrame>
        <ErrorBoundary fallback={(err) => <ErrorView err={err} />}>
            <WindowBrow title="节点网络" show_logo logo_link="https://github.com/ZhangXiChang/node-network" />
        </ErrorBoundary>
    </WindowFrame>;
}
