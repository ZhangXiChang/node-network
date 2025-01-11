import { createSignal, Match, Switch } from "solid-js";
import { LoginUI } from "./login_ui";

enum ViewUIType {
    LoginUI
}

const [viewUIType, changeViewUI] = createSignal(ViewUIType.LoginUI);
export { changeViewUI };

export function View() {
    return (
        <Switch>
            <Match when={viewUIType() == ViewUIType.LoginUI}>
                <LoginUI />
            </Match>
        </Switch>
    );
}
