import { getCurrentWindow } from "@tauri-apps/api/window";
import { openUrl } from "@tauri-apps/plugin-opener";
import { children, createSignal, ErrorBoundary, JSX } from "solid-js";
import { AsyncError } from "./async_error";

const error = new AsyncError();
export function captureError(err: Error) {
    error.capture(err);
}

export function Window(props: { children?: JSX.Element }) {
    const selfChildren = children(() => props.children);
    const mainWindow = getCurrentWindow();
    const [windowToggleMaximizeIcon, setWindowToggleMaximizeIcon] = createSignal("i-mdi:window-maximize w-16px h-16px");
    mainWindow.listen("tauri://resize", async () => setWindowToggleMaximizeIcon(await mainWindow.isMaximized() ?
        "i-mdi:window-restore w-16px h-16px" :
        "i-mdi:window-maximize w-16px h-16px",
    ));
    return (
        <div class="absolute size-full flex flex-col bg-white">
            <div data-tauri-drag-region class="h-32px flex items-center">
                <div data-tauri-drag-region class="w-32px flex justify-center">
                    <div class="rounded hover:cursor-pointer hover:bg-gray-3" onclick={() => openUrl("https://github.com/ZhangXiChang/node-network")}>
                        <div class="i-line-md:github-loop h-16px w-16px" />
                    </div>
                </div>
                <label data-tauri-drag-region class="h-24px font-bold" style={{ "text-shadow": "0px 0px 10px gray" }}>节点网络</label>
                <div data-tauri-drag-region class="flex flex-auto" />
                <div data-tauri-drag-region class="w-32px flex justify-center">
                    <div class="rounded hover:cursor-pointer hover:bg-gray-3" onclick={mainWindow.minimize}>
                        <div class="i-mdi:window-minimize h-16px w-16px" />
                    </div>
                </div>
                <div data-tauri-drag-region class="w-32px flex justify-center">
                    <div class="rounded hover:cursor-pointer hover:bg-gray-3" onclick={mainWindow.toggleMaximize}>
                        <div class={windowToggleMaximizeIcon()} />
                    </div>
                </div>
                <div data-tauri-drag-region class="w-32px flex justify-center">
                    <div class="rounded hover:cursor-pointer hover:bg-gray-3" onclick={mainWindow.close}>
                        <div class="i-mdi:window-close h-16px w-16px" />
                    </div>
                </div>
            </div>
            <ErrorBoundary fallback={(err: Error) => err.message}>
                {error.trigger()}
                {selfChildren()}
            </ErrorBoundary>
        </div>
    );
}
