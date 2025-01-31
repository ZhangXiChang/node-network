import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow, PhysicalSize } from "@tauri-apps/api/window";
import { error } from "@tauri-apps/plugin-log";
import { Button } from "~/components/button";
import { Card } from "~/components/card";
import { Field } from "~/components/field";
import { changeViewUI, ViewUIType } from "./view";

export function LoginUI() {
    invoke("connect_server", { socketaddr: "127.0.0.1:10270" }).catch((err) => error(`${err}`));
    const login = async (loginName: string) => {
        try {
            await invoke("login", { loginName });
            changeViewUI(ViewUIType.ChatUI);
            await getCurrentWindow().setSize(new PhysicalSize(1100, 700));
            getCurrentWindow().center();
        } catch (err) {
            error(`${err}`);
        }
    };
    return <>
        <Card.Root class="flex-1 shadow-none">
            <Card.Header>
                <Card.Title>欢迎使用</Card.Title>
                <Card.Description>填写个性化信息</Card.Description>
            </Card.Header>
            <Card.Body>
                <Field.Root>
                    <Field.Label>名称</Field.Label>
                    <Field.Input id="login_name_input" autocomplete="off" placeholder="取一个喜欢的名称吧"
                        on:input={(e) => {
                            const loginButton = document.getElementById("login_button");
                            if ((e.target as HTMLInputElement).value.length) {
                                if (loginButton?.hasAttribute("disabled")) {
                                    loginButton?.toggleAttribute("disabled");
                                }
                            }
                            else {
                                if (!loginButton?.hasAttribute("disabled")) {
                                    loginButton?.toggleAttribute("disabled");
                                }
                            }
                        }}
                        on:keydown={(e) => {
                            if (e.key == "Enter") {
                                if ((e.target as HTMLInputElement).value.length) {
                                    login((e.target as HTMLInputElement).value);
                                }
                            }
                        }}
                    />
                </Field.Root>
            </Card.Body>
            <Card.Footer class="gap-3">
                <Button variant="outline" on:click={() => getCurrentWindow().close()}>退出</Button>
                <Button id="login_button" disabled
                    on:click={() => login((document.getElementById("login_name_input") as HTMLInputElement).value)}
                >登录</Button>
            </Card.Footer>
        </Card.Root>
    </>;
}
