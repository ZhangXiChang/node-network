import { invoke } from "@tauri-apps/api/tauri";
import { appWindow } from "@tauri-apps/api/window";
import { createSignal } from "solid-js";

export default function Eyebrow() {
    const [windowMaximizeIcon, setWindowMaximizeIcon] = createSignal(<div class="i-mdi:window-maximize w-16px h-16px"></div>);
    const toggleMaximize = async () => {
        await appWindow.toggleMaximize();
        setWindowMaximizeIcon(await appWindow.isMaximized() ?
            <div class="i-mdi:window-restore w-16px h-16px"></div>
            :
            <div class="i-mdi:window-maximize w-16px h-16px"></div>
        );
    };
    const openGithub = () => invoke("open", { path: "https://github.com/ZhangXiChang/node-network" });
    return (<div class="h-32px pl-2px pr-4px flex">
        <div class="flex justify-center items-center" data-tauri-drag-region>
            <div class="mx-4px rounded hover:cursor-pointer hover:bg-gray-3" onclick={openGithub}>
                <div class="i-line-md:github-loop w-24px h-24px"></div>
            </div>
        </div>
        <div class="flex-1 flex items-center" data-tauri-drag-region>
            <label class="h-24px font-bold" id="window-title-text">节点网络</label>
        </div>
        <div class="flex justify-center items-center" data-tauri-drag-region>
            <div class="mx-4px rounded hover:cursor-pointer hover:bg-gray-3" onclick={() => appWindow.minimize()}>
                <div class="i-mdi:window-minimize w-16px h-16px"></div>
            </div>
            <div class="mx-4px rounded hover:cursor-pointer hover:bg-gray-3" onclick={toggleMaximize}>
                {windowMaximizeIcon()}
            </div>
            <div class="mx-4px rounded hover:cursor-pointer hover:bg-gray-3" onclick={() => appWindow.close()}>
                <div class="i-mdi:window-close w-16px h-16px"></div>
            </div>
        </div>
    </div>);
}
