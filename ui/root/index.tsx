import { Match, Switch } from "solid-js";
import { View } from "./view";
import { Window } from "./window";
import { type as osType } from "@tauri-apps/plugin-os";

export function Root() {
    return (
        <Switch>
            <Match when={osType() == "windows"}>
                <Window>
                    <View />
                </Window>
            </Match>
            <Match when={osType() == "android"}>
                <View />
            </Match>
        </Switch>
    );
}
