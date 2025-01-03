import { createSignal, ErrorBoundary, For } from "solid-js";
import { WindowBrow, WindowFrame } from "../components/window";
import { ErrorView } from "./error_view";

export function Root() {
    const [messageList, setMessageList] = createSignal([] as string[]);
    return <WindowFrame>
        <ErrorBoundary fallback={(err) => <ErrorView err={err} />}>
            <WindowBrow title="节点网络" show_logo logo_link="https://github.com/ZhangXiChang/node-network" />
            <div class="flex flex-auto flex-col">
                <For each={messageList()}>{(msg) => <div>{msg}</div>}
                </For>
            </div>
            <div class="h-32px flex items-center shadow-black shadow-sm">
                <input type="text" autocomplete="off" id="TextInput" placeholder="输入" class="size-full flex" on:keypress={(e) => {
                    if (e.key == "Enter") {
                        setMessageList([...messageList(), e.currentTarget.value]);
                        e.currentTarget.value = "";
                    }
                }} />
            </div>
        </ErrorBoundary>
    </WindowFrame>;
}
