import { open } from "@tauri-apps/api/shell";
import { appWindow } from "@tauri-apps/api/window";
import { createSignal } from "solid-js";

export default function TitleBar() {
    const [windowToggleMaximizeIcon, setwindowToggleMaximizeIcon] = createSignal("i-mdi:window-maximize w-16px h-16px");
    appWindow.listen("tauri://resize", async () =>
        setwindowToggleMaximizeIcon(await appWindow.isMaximized() ?
            "i-mdi:window-restore w-16px h-16px" :
            "i-mdi:window-maximize w-16px h-16px",
        ),
    );
    return <div data-tauri-drag-region class="h-32px flex items-center">
        <div class="w-32px flex justify-center">
            <div class="rounded hover:cursor-pointer hover:bg-gray-3" onclick={() => open("https://github.com/ZhangXiChang/node-network")}>
                <div class="i-line-md:github-loop h-16px w-16px" />
            </div>
        </div>
        <label class="h-24px font-bold" style={{ "text-shadow": "0px 0px 10px gray" }}>节点网络</label>
        <div class="flex-1" />
        <div class="w-32px flex justify-center">
            <div class="rounded hover:cursor-pointer hover:bg-gray-3" onclick={appWindow.minimize}>
                <div class="i-mdi:window-minimize h-16px w-16px" />
            </div>
        </div>
        <div class="w-32px flex justify-center">
            <div class="rounded hover:cursor-pointer hover:bg-gray-3" onclick={appWindow.toggleMaximize}>
                <div class={windowToggleMaximizeIcon()} />
            </div>
        </div>
        <div class="w-32px flex justify-center">
            <div class="rounded hover:cursor-pointer hover:bg-gray-3" onclick={appWindow.close}>
                <div class="i-mdi:window-close h-16px w-16px" />
            </div>
        </div>
    </div>;
}
