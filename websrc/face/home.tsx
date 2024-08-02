import { invoke } from "@tauri-apps/api/tauri";
import { createSignal } from "solid-js";
import { Button, ButtonNavigation } from "../widgets/button/mod";
import { ImageButton } from "../widgets/button/image-button";

export default function Home() {
    const [sidebarHubNodeLogoButton, setSidebarHubNodeLogoButton] = createSignal(<></>);
    const discoverButton = new Button({
        base: "rounded",
        selected: "bg-blue",
        hovered: "cursor-pointer bg-gray-3"
    });
    discoverButton.setStyleToSelected();
    const homeButton = new Button({
        base: "rounded",
        selected: "bg-blue",
        hovered: "cursor-pointer bg-gray-3"
    });
    homeButton.setStyleToSelected();
    const rootButtonNavigation = new ButtonNavigation([discoverButton]);
    (async () => {
        try {
            const hubnodeTable = await invoke("get_hubnode_table") as {
                base: {
                    name: string,
                    ipv4_address: string,
                    ipv6_address: string,
                    description: string,
                }
                cert_der: string,
                logo: string,
            }[];
            setSidebarHubNodeLogoButton(hubnodeTable.map((hubnodeInfo) => {
                const hubnodeLogoButton = new ImageButton({
                    base: "rounded",
                    selected: "bg-blue",
                    hovered: "cursor-pointer bg-gray-3"
                }, "data:image/png;base64," + hubnodeInfo.logo, 48, 48);
                rootButtonNavigation.addButton(hubnodeLogoButton);
                return hubnodeLogoButton.html();
            }))
        } catch (err) {
            console.log(err);
        }
    })();
    return (<>
        <div class="w-70px flex flex-col">
            <div class="py-4px flex justify-center items-center">
                {discoverButton.html(<div class="i-line-md:compass-loop w-48px h-48px"></div>)}
            </div>
            <div class="flex flex-col items-center">
                {sidebarHubNodeLogoButton()}
            </div>
        </div>
        <div class="w-220px px-10px rounded-lt-8px bg-gray-2 flex flex-col">
            <div class="h-55px pl-15px flex items-center">
                <label class="font-bold text-size-2xl">发现</label>
            </div>
            <div class="flex flex-col items-center">
                {homeButton.html(<label>主页</label>)}
            </div>
        </div>
        <div class="flex-1 px-20px pt-20px bg-white flex flex-col items-center">
            <div class="relative w-full pb-30%">
                <div class="absolute size-full flex bg-blue"></div>
            </div>
        </div>
    </>);
}
