import { invoke } from "@tauri-apps/api/tauri";
import { appWindow } from "@tauri-apps/api/window";
import { createSignal } from "solid-js";

export default function Eyebrow() {
    const [window_maximize_button_ico_path, set_window_maximize_button_ico_path] = createSignal("./window/eyebrow/window-maximize.svg");
    return (<div class="Eyebrow">
        <div class="MenuBar" data-tauri-drag-region>
            <div class="GithubLogo" onclick={() => invoke("open", { path: "https://github.com/ZhangXiChang/node-network" })}>
                <img src="./window/eyebrow/github-loop.svg" />
            </div>
            <div class="SettingButton">
                <img src="./window/eyebrow/setting.svg" />
            </div>
        </div>
        <div class="Title" data-tauri-drag-region>
            <label>节点网络</label>
        </div>
        <div class="ControlBar" data-tauri-drag-region>
            <div class="Button" onclick={() => appWindow.minimize()}>
                <img src="./window/eyebrow/window-minimize.svg" />
            </div>
            <div class="Button" onclick={async () => {
                await appWindow.toggleMaximize();
                if (await appWindow.isMaximized()) {
                    set_window_maximize_button_ico_path("./window/eyebrow/window-restore.svg");
                } else {
                    set_window_maximize_button_ico_path("./window/eyebrow/window-maximize.svg");
                }
            }}>
                <img src={window_maximize_button_ico_path()} />
            </div>
            <div class="Button" onclick={() => appWindow.close()}>
                <img src="./window/eyebrow/window-close.svg" />
            </div>
        </div>
    </div>);
};
