import { createSignal, Match, Switch } from "solid-js";
import { LoginUI } from "./login_ui";
import { ChatUI } from "./chat_ui";

export enum ViewUIType {
    LoginUI,
    ChatUI
}

const [viewUIType, changeViewUI] = createSignal(ViewUIType.LoginUI);
export { changeViewUI };

export function View() {
    return <>
        <Switch>
            <Match when={viewUIType() == ViewUIType.LoginUI}>
                <LoginUI />
            </Match>
            <Match when={viewUIType() == ViewUIType.ChatUI}>
                <ChatUI />
            </Match>
        </Switch>
    </>;
}
