import { invoke } from "@tauri-apps/api/tauri";
import { createRoot, createSignal } from "solid-js";

interface SelectStyle {
    ed: string;
    un: string;
}
interface ButtonStyle {
    base: string;
    select: SelectStyle;
}

class Button {
    group: string;
    style: ButtonStyle;

    constructor(group: string, style: ButtonStyle) {
        this.group = group;
        this.style = style;
    }
    id(): string {
        return this.group;
    }
    selectedStyle(): string {
        return this.style.base + " " + this.style.select.ed;
    }
    unselectedStyle(): string {
        return this.style.base + " " + this.style.select.un;
    }
    select(target: Element) {
        if (target.className != this.selectedStyle()) {
            let buttonGroup = document.querySelectorAll("#" + this.id());
            for (let i = 0; i < buttonGroup.length; i++) {
                buttonGroup[i].className = this.unselectedStyle();
            }
            target.className = this.selectedStyle();
        }
    }
}
class ButtonGroup {
    buttons: Button[];

    constructor(buttons: Button[]) {
        this.buttons = buttons;
    }
}

const RootOptionsButton = new Button("RootOptionsButton", {
    base: "rounded",
    select: {
        ed: "bg-blue",
        un: "hover:cursor-pointer hover:bg-gray-3"
    }
});
const HubNodeOptionsButton = new Button("RootOptionsButton", {
    base: "",
    select: {
        ed: "rounded bg-blue",
        un: "rounded-full hover:cursor-pointer hover:bg-gray-3"
    }
});
const DiscoverOptionsButton = new Button("DiscoverOptionsButton", {
    base: "w-95% h-40px pl-5% rounded flex items-center",
    select: {
        ed: "bg-blue",
        un: "hover:cursor-pointer hover:bg-gray-3"
    }
});

export default function Home() {
    const [sidebarHubNodeLogoButton, setSidebarHubNodeLogoButton] = createSignal(<></>);
    (async () => {
        try {
            let userStarHubNodeLogo = ["", ""];//await invoke("get_user_star_hubnode_logo") as string[];
            createRoot(() => {
                setSidebarHubNodeLogoButton(<>
                    {userStarHubNodeLogo.map((_) => (
                        <div class="py-4px flex justify-center items-center">
                            <div class={HubNodeOptionsButton.unselectedStyle()} id={HubNodeOptionsButton.id()} onclick={(e) => {
                                HubNodeOptionsButton.select(e.currentTarget);
                            }}>
                                <div class="i-line-md:compass-loop w-48px h-48px"></div>
                            </div>
                        </div>
                    ))}
                </>)
            });
        } catch (err: any) {
            createRoot(() => { setSidebarHubNodeLogoButton(<>{err}</>) });
        }
    })();
    return (<>
        <div class="w-70px flex flex-col">
            <div class="py-4px flex justify-center items-center">
                <div class={RootOptionsButton.selectedStyle()} id={RootOptionsButton.id()} onclick={(e) => {
                    RootOptionsButton.select(e.currentTarget);
                }}>
                    <div class="i-line-md:compass-loop w-48px h-48px"></div>
                </div>
            </div>
            <div class="flex flex-col">
                {sidebarHubNodeLogoButton()}
            </div>
        </div>
        <div class="w-220px px-10px rounded-lt-8px bg-gray-2 flex flex-col">
            <div class="h-55px pl-15px flex items-center">
                <label class="font-bold text-size-2xl">发现</label>
            </div>
            <div class="flex flex-col items-center">
                <div class={DiscoverOptionsButton.selectedStyle()} id={DiscoverOptionsButton.id()} onclick={(e) => {
                    DiscoverOptionsButton.select(e.currentTarget);
                }}>
                    <label>主页</label>
                </div>
            </div>
        </div>
        <div class="flex-1 px-20px pt-20px bg-white flex flex-col items-center">
            <div class="relative w-full pb-30%">
                <div class="absolute size-full flex bg-blue"></div>
            </div>
        </div>
    </>);
}
