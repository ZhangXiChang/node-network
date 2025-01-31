import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { error } from "@tauri-apps/plugin-log";
import { createSignal, For } from "solid-js";
import { Alert } from "~/components/alert";
import { Avatar } from "~/components/avatar";
import { Field } from "~/components/field";

interface ChatMessage {
    nodeName: string,
    value: string
}

export function ChatUI() {
    const [messageList, setMessageList] = createSignal([] as ChatMessage[]);
    getCurrentWindow().listen<ChatMessage>("accept_message", async (e) => setMessageList([...messageList(), e.payload]));
    return <>
        <div class="flex flex-1 flex-col gap-5px">
            <div class="flex flex-1 flex-col gap-5px overflow-auto">
                <For each={messageList()}>{(message) => <>
                    <Alert.Root>
                        <Avatar />
                        <Alert.Content>
                            <Alert.Title>{message.nodeName}</Alert.Title>
                            <Alert.Description innerHTML={message.value.replace(/\n/g, "<br/>")} />
                        </Alert.Content>
                    </Alert.Root>
                </>}</For>
            </div>
            <Field.Root>
                <Field.Textarea id="chat_text_input" class="resize-none" rows={4}
                    on:keydown={async (e) => {
                        if (e.key == "Enter" && e.ctrlKey) {
                            const chat_input_text = (e.target as HTMLTextAreaElement).value;
                            if (chat_input_text.length) {
                                (e.target as HTMLTextAreaElement).value = "";
                                setMessageList([...messageList(), {
                                    nodeName: await invoke("get_node_name").catch((err) => error(`${err}`)) as string,
                                    value: chat_input_text,
                                }]);
                                invoke("send_message", { message: chat_input_text });
                            }
                        }
                    }}
                >
                </Field.Textarea>
            </Field.Root>
        </div>
    </>;
}
