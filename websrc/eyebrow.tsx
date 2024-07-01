import { invoke } from "@tauri-apps/api/tauri";
import { appWindow } from "@tauri-apps/api/window";
import { createSignal } from "solid-js";

export default function Eyebrow() {
    const [window_maximize_button_ico_path, set_window_maximize_button_ico_path] = createSignal("./window/eyebrow/window-maximize.svg");
    return (<div class="h-10 flex px-1">
        <div class="flex justify-center items-center" data-tauri-drag-region>
            <div class="flex rounded mx-1 hover:cursor-pointer hover:bg-gray-300" onclick={() => invoke("open", { path: "https://github.com/ZhangXiChang/node-network" })}>
                <img src="./window/eyebrow/logo.svg" />
            </div>
        </div>
        <div class="flex justify-center items-center flex-1" data-tauri-drag-region>
            <label class="title-text-shadow font-sans font-bold" id="title-text">节点网络</label>
        </div>
        <div class="flex justify-center items-center" data-tauri-drag-region>
            <div class="flex rounded mx-1 hover:cursor-pointer hover:bg-gray-300" onclick={() => appWindow.minimize()}>
                <img src="./window/eyebrow/window-minimize.svg" />
            </div>
            <div class="flex rounded mx-1 hover:cursor-pointer hover:bg-gray-300" onclick={async () => {
                await appWindow.toggleMaximize();
                if (await appWindow.isMaximized()) {
                    set_window_maximize_button_ico_path("./window/eyebrow/window-restore.svg");
                } else {
                    set_window_maximize_button_ico_path("./window/eyebrow/window-maximize.svg");
                }
            }}>
                <img src={window_maximize_button_ico_path()} />
            </div>
            <div class="flex rounded mx-1 hover:cursor-pointer hover:bg-gray-300" onclick={() => appWindow.close()}>
                <img src="./window/eyebrow/window-close.svg" />
            </div>
        </div>
    </div>);
};
