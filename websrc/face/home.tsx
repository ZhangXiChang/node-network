export default function Home() {
    return (<>
        <div class="w-70px flex flex-col items-center">
            <div class="h-55px flex justify-center items-center">
                <div class="rounded hover:cursor-pointer hover:bg-gray-3">
                    <div class="i-line-md:compass-loop w-48px h-48px"></div>
                </div>
            </div>
        </div>
        <div class="w-220px bg-gray-2 flex flex-col rounded-lt-8px px-20px">
            <div class="h-55px flex items-center pl-15px">
                <label class="font-bold text-size-2xl">发现</label>
            </div>
            <div class="w-full flex-1 flex flex-col items-center">
                <div class="w-full h-40px flex items-center pl-10px rounded hover:cursor-pointer hover:bg-gray-3" onclick={(e) => {
                    e.currentTarget.className = "w-full h-40px flex items-center pl-10px rounded bg-blue";
                }}>
                    <label>主页</label>
                </div>
            </div>
        </div>
        <div class="flex-1 bg-white flex flex-col items-center px-20px pt-20px">
            <div class="relative w-full pb-30%">
                <div class="absolute size-full flex bg-blue"></div>
            </div>
        </div>
    </>);
}
