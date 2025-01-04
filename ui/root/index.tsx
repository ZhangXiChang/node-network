import { ErrorBoundary } from "solid-js";
import { WindowBrow, WindowFrame } from "../components/window";
import { ErrorView } from "./error_view";
import { invoke } from "@tauri-apps/api/core";
import { AsyncError } from "../components";

export function Root() {
    const asyncError = new AsyncError();
    return <WindowFrame>
        <ErrorBoundary fallback={(err) => <ErrorView err={err} />}>
            {asyncError.trigger()}
            <WindowBrow title="节点网络" show_logo logo_link="https://github.com/ZhangXiChang/node-network" />
            <div class="flex flex-auto flex-col">
                <input type="text" id="name" class="flex" />
                <input type="text" id="ipv4_addr" class="flex" />
                <div on:click={() => invoke("connect").catch((err) => asyncError.capture(err))}><label>登录</label></div>
            </div>
        </ErrorBoundary>
    </WindowFrame>;
}
