import { invoke } from "@tauri-apps/api/tauri";
import { createRoot, createSignal } from "solid-js";

enum RootOptions {
    Discover,
    HubNode
}
enum DiscoverOptions {
    Home,
}

interface SelectStyle {
    ed: string;
    un: string;
}
interface ButtonStyle {
    base: string;
    select: SelectStyle;
}

class Button {
    name: string;
    style: ButtonStyle;

    constructor(name: string, style: ButtonStyle) {
        this.name = name;
        this.style = style;
    }
    selectedStyle(): string {
        return this.name + " " + this.style.base + " " + this.style.select.ed;
    }
    unselectedStyle(): string {
        return this.name + " " + this.style.base + " " + this.style.select.un;
    }
    select(target: Element) {
        for (let i = 0; i < document.getElementsByClassName(this.selectedStyle()).length; i++) {
            document.getElementsByClassName(this.selectedStyle())[i].className = this.unselectedStyle();
        }
        target.className = this.selectedStyle();
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
    base: "rounded",
    select: {
        ed: "bg-blue",
        un: "hover:cursor-pointer hover:bg-gray-3"
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
    const [rootOptions, setRootOptions] = createSignal(RootOptions.Discover);
    const [discoverOptions, setDiscoverOptions] = createSignal(DiscoverOptions.Home);
    (async () => {
        try {
            let userStarHubNodeLogo = await invoke("get_user_star_hubnode_logo") as string[];
            createRoot(() => {
                setSidebarHubNodeLogoButton(<>
                    {userStarHubNodeLogo.map((logo) => (
                        <div class={RootOptionsButton.unselectedStyle()} onclick={(e) => {
                            if (rootOptions() != RootOptions.HubNode) {
                                setRootOptions(RootOptions.HubNode);
                                RootOptionsButton.select(e.currentTarget);
                            }
                        }}>
                            <img class="w-48px h-48px" src={"data:image/png;base64," + logo} />
                        </div>
                    ))}
                </>)
            });
        } catch (err: any) {
            createRoot(() => { setSidebarHubNodeLogoButton(<>{err}</>) });
        }
    })();
    return (<>
        <div class="w-70px flex flex-col items-center">
            <div class="h-55px flex justify-center items-center">
                <div class={RootOptionsButton.selectedStyle()} onclick={(e) => {
                    if (rootOptions() != RootOptions.Discover) {
                        setRootOptions(RootOptions.Discover);
                        RootOptionsButton.select(e.currentTarget);
                    }
                }}>
                    <div class="i-line-md:compass-loop w-48px h-48px"></div>
                </div>
            </div>
            <div class="flex-1">
                {sidebarHubNodeLogoButton()}
            </div>
        </div>
        <div class="w-220px px-10px rounded-lt-8px bg-gray-2 flex flex-col">
            <div class="h-55px pl-15px flex items-center">
                <label class="font-bold text-size-2xl">发现</label>
            </div>
            <div class="flex-1 flex flex-col items-center">
                <div class={DiscoverOptionsButton.selectedStyle()} onclick={(e) => {
                    if (discoverOptions() != DiscoverOptions.Home) {
                        setDiscoverOptions(DiscoverOptions.Home);
                        DiscoverOptionsButton.select(e.currentTarget);
                    }
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
