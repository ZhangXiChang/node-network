import { createSignal, For } from "solid-js";

export function ChatFrame() {
    const [messageList, setMessageList] = createSignal([] as string[]);
    return <div class="flex flex-auto flex-col">
        <div class="flex flex-auto flex-col">
            <For each={messageList()}>{(msg) => <div>{msg}</div>}</For>
        </div>
        <div class="h-32px flex items-center shadow-black shadow-sm">
            <input type="text" autocomplete="off" id="TextInput" placeholder="输入" class="size-full flex" on:keypress={(e) => {
                if (e.key == "Enter") {
                    setMessageList([...messageList(), e.currentTarget.value]);
                    e.currentTarget.value = "";
                }
            }} />
        </div>
    </div>;
}
