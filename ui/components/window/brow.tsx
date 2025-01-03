import { getCurrentWindow } from "@tauri-apps/api/window";
import { open } from "@tauri-apps/plugin-shell";
import { createSignal, Show } from "solid-js";

export function WindowBrow(props: {
    title?: string,
    logo_link?: string,
    show_logo?: boolean
}) {
    const mainWindow = getCurrentWindow();
    const [windowToggleMaximizeIcon, setWindowToggleMaximizeIcon] = createSignal("i-mdi:window-maximize w-16px h-16px");
    mainWindow.listen("tauri://resize", async () => setWindowToggleMaximizeIcon(await mainWindow.isMaximized() ?
        "i-mdi:window-restore w-16px h-16px" :
        "i-mdi:window-maximize w-16px h-16px",
    ));
    return <div data-tauri-drag-region class="h-32px flex items-center">
        <Show when={props.show_logo} fallback={<div class="w-16px" />}>
            <div class="w-32px flex justify-center">
                <div class="rounded hover:cursor-pointer hover:bg-gray-3" onclick={() => props.logo_link && open(props.logo_link)}>
                    <div class="i-line-md:github-loop h-16px w-16px" />
                </div>
            </div>
        </Show>
        <label class="h-24px font-bold" style={{ "text-shadow": "0px 0px 10px gray" }}>{props.title && props.title}</label>
        <div class="flex flex-auto" />
        <div class="w-32px flex justify-center">
            <div class="rounded hover:cursor-pointer hover:bg-gray-3" onclick={mainWindow.minimize}>
                <div class="i-mdi:window-minimize h-16px w-16px" />
            </div>
        </div>
        <div class="w-32px flex justify-center">
            <div class="rounded hover:cursor-pointer hover:bg-gray-3" onclick={mainWindow.toggleMaximize}>
                <div class={windowToggleMaximizeIcon()} />
            </div>
        </div>
        <div class="w-32px flex justify-center">
            <div class="rounded hover:cursor-pointer hover:bg-gray-3" onclick={mainWindow.close}>
                <div class="i-mdi:window-close h-16px w-16px" />
            </div>
        </div>
    </div>;
}
