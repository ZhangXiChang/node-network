import { invoke } from "@tauri-apps/api/tauri";
import { appWindow } from "@tauri-apps/api/window";
import { createSignal } from "solid-js";

export default function Eyebrow() {
    const [window_toggleMaximize_button_ico, set_window_toggleMaximize_button_ico] = createSignal(<div class="i-mdi:window-maximize w-16px h-16px"></div>);
    return (<div class="h-10 bg-gray-1 flex px-1">
        <div class="flex justify-center items-center" data-tauri-drag-region>
            <div class="flex mx-1 hover:cursor-pointer hover:bg-gray-3" onclick={() => invoke("open", { path: "https://github.com/ZhangXiChang/node-network" })}>
                <div class="i-line-md:github-loop w-24px h-24px"></div>
            </div>
        </div>
        <div class="flex-1 flex justify-center items-center" data-tauri-drag-region>
            <label class="font-bold" id="title-text">节点网络</label>
        </div>
        <div class="flex justify-center items-center mr-1" data-tauri-drag-region>
            <div class="flex mx-1 hover:cursor-pointer hover:bg-gray-3" onclick={() => appWindow.minimize()}>
                <div class="i-mdi:window-minimize w-16px h-16px"></div>
            </div>
            <div class="flex mx-1 hover:cursor-pointer hover:bg-gray-3" onclick={async () => {
                await appWindow.toggleMaximize();
                if (await appWindow.isMaximized()) {
                    set_window_toggleMaximize_button_ico(<div class="i-mdi:window-restore w-16px h-16px"></div>);
                } else {
                    set_window_toggleMaximize_button_ico(<div class="i-mdi:window-maximize w-16px h-16px"></div>);
                }
            }}>
                {window_toggleMaximize_button_ico()}
            </div>
            <div class="flex mx-1 hover:cursor-pointer hover:bg-gray-3" onclick={() => appWindow.close()}>
                <div class="i-mdi:window-close w-16px h-16px"></div>
            </div>
        </div>
    </div>);
};
