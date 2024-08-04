import { invoke } from "@tauri-apps/api/tauri";
import { createSignal } from "solid-js";
import { Button, buttonNavigation } from "../widgets/button/mod";
import { ImageButton } from "../widgets/button/image-button";

export default function Home() {
    const [sidebarHubNodeLogoButton, setSidebarHubNodeLogoButton] = createSignal(<></>);
    const discoverButton = new Button({
        base: "rounded",
        selected: "bg-blue",
        hovered: "cursor-pointer bg-gray-3"
    });
    const homeButton = new Button({
        base: "rounded",
        selected: "bg-blue",
        hovered: "cursor-pointer bg-gray-3"
    });
    const rootButtonNavigation = [discoverButton];
    (async () => {
        try {
            const hubnodeTable = await invoke("get_hubnode_table") as { logo: string }[];
            setSidebarHubNodeLogoButton(hubnodeTable.map((hubnodeInfo) => {
                const hubnodeLogoButton = new ImageButton({
                    base: "rounded",
                    selected: "",
                    hovered: "cursor-pointer"
                }, "data:image/png;base64," + hubnodeInfo.logo, 48, 48);
                rootButtonNavigation.push(hubnodeLogoButton);
                return <div class="py-4px flex">
                    {hubnodeLogoButton.html()}
                </div>;
            }));
        } catch (err) {
            console.log(err);
        }
        buttonNavigation(rootButtonNavigation);
    })();
    return (<>
        <div class="w-70px flex flex-col items-center">
            <div class="py-4px flex">
                {discoverButton.withStyleToSelected().html(<div class="i-line-md:compass-loop w-48px h-48px"></div>)}
            </div>
            {sidebarHubNodeLogoButton()}
        </div>
        <div class="w-220px px-10px rounded-lt-8px bg-gray-2 flex flex-col items-center">
            <div class="h-55px pr-120px flex items-center">
                <label class="font-bold text-size-2xl">发现</label>
            </div>
            {homeButton.withStyleToSelected().withStyle("w-100px h-55px").html(<label>主页</label>)}
        </div>
        <div class="flex-1 px-20px pt-20px bg-white flex flex-col items-center">
            <div class="relative w-full pb-30%">
                <div class="absolute size-full flex bg-blue"></div>
            </div>
        </div>
    </>);
}
