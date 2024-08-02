export default function Error({ err }: any) {
    return <div class="flex-1 flex justify-center items-center">
        <label class="font-bold text-size-xl">{"致命错误，原因：" + err}</label>
    </div>;
}
