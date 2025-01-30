import { invoke } from "@tauri-apps/api/core";
import { error } from "@tauri-apps/plugin-log";
import { createSignal, For } from "solid-js";
import { Alert } from "~/components/alert";
import { Avatar } from "~/components/avatar";
import { Field } from "~/components/field";

interface Message {
    nodeName: string,
    value: string
}

export function ChatUI() {
    const [messageList, setMessageList] = createSignal([] as Message[]);
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
                <Field.Textarea class="resize-none" rows={4}
                    on:keydown={async (e) => {
                        if (e.key == "Enter" && e.ctrlKey) {
                            if ((e.target as HTMLInputElement).value.length) {
                                setMessageList([...messageList(), {
                                    nodeName: await invoke("get_node_name").catch((err) => error(`${err}`)) as string,
                                    value: (e.target as HTMLInputElement).value,
                                }]);
                                (e.target as HTMLInputElement).value = "";
                            }
                        }
                    }}
                >
                </Field.Textarea>
            </Field.Root>
        </div>
    </>;
}
