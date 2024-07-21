import { invoke } from "@tauri-apps/api/tauri";
import { createRoot, createSignal } from "solid-js";
import { Button, ButtonGroup } from "../widgets/button";

export default function Home() {
    const [sidebarHubNodeLogoButton, setSidebarHubNodeLogoButton] = createSignal(<></>);
    const discoverButton = new Button({
        base: "rounded",
        select: {
            ed: "bg-blue",
            un: "hover:cursor-pointer hover:bg-gray-3"
        },
        hover: {
            ed: "bg-blue",
            un: "hover:cursor-pointer hover:bg-gray-3"
        }
    });
    const homeButton = new Button({
        base: "w-95% h-40px pl-5% rounded flex items-center",
        select: {
            ed: "bg-blue",
            un: "hover:cursor-pointer hover:bg-gray-3"
        },
        hover: {
            ed: "bg-blue",
            un: "hover:cursor-pointer hover:bg-gray-3"
        }
    });
    const rootMenuButton = new ButtonGroup([discoverButton]);
    const discoverMenuButton = new ButtonGroup([homeButton]);
    (async () => {
        try {
            const hubNodeInfoList = await invoke("get_hubnode_table") as {
                base: {
                    name: string,
                    ipv4_address: string,
                    ipv6_address: string,
                    description: string,
                }
                cert_der: string,
                logo: string,
            }[];
            createRoot(() => {
                setSidebarHubNodeLogoButton(<>
                    {hubNodeInfoList.map((hubNodeInfo) => {
                        const button = new Button({
                            base: "",
                            select: {
                                ed: "rounded bg-blue",
                                un: "rounded-full hover:cursor-pointer hover:bg-gray-3"
                            },
                            hover: {
                                ed: "bg-blue",
                                un: "hover:cursor-pointer hover:bg-gray-3"
                            }
                        });
                        rootMenuButton.add(button);
                        return (<div class="py-4px flex justify-center items-center">
                            <div class={button.unselectedStyle()} id={button.id()} onclick={() => {
                                rootMenuButton.select(button);
                            }}>
                                <img width="48" height="48" src={"data:image/png;base64," + hubNodeInfo.logo} />
                            </div>
                        </div>)
                    })}
                </>)
            });
        } catch (err: any) {
            createRoot(() => { setSidebarHubNodeLogoButton(<>{err}</>) });
        }
    })();
    return (<>
        <div class="w-70px flex flex-col">
            <div class="py-4px flex justify-center items-center">
                <div class={discoverButton.selectedStyle()} id={discoverButton.id()} onclick={() => {
                    rootMenuButton.select(discoverButton);
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
                <div class={homeButton.selectedStyle()} id={homeButton.id()} onclick={() => {
                    discoverMenuButton.select(homeButton);
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
