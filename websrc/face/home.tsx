import { invoke } from "@tauri-apps/api/tauri";
import { createSignal } from "solid-js";

export default function Home() {
    const [sidebar_hubnode_logo_button, set_sidebar_hubnode_logo_button] = createSignal(<></>);
    invoke("get_user_star_hubnode_logo").then(() => { });
    return (<>
        <div class="w-70px flex flex-col items-center">
            <div class="h-55px flex justify-center items-center">
                <div class="rounded hover:cursor-pointer hover:bg-gray-3">
                    <div class="i-line-md:compass-loop w-48px h-48px"></div>
                </div>
            </div>
            {sidebar_hubnode_logo_button()}
        </div>
        <div class="w-220px px-10px rounded-lt-8px bg-gray-2 flex flex-col">
            <div class="h-55px pl-15px flex items-center">
                <label class="font-bold text-size-2xl">发现</label>
            </div>
            <div class="flex-1 flex flex-col items-center">
                <div class="w-95% h-40px pl-5% rounded flex items-center hover:cursor-pointer hover:bg-gray-3" onclick={(e) => {
                    e.currentTarget.className = "w-95% h-40px pl-5% rounded flex items-center bg-blue";
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
