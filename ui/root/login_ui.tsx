import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow, PhysicalSize } from "@tauri-apps/api/window";
import { error } from "@tauri-apps/plugin-log";
import { Button } from "~/components/button";
import { Card } from "~/components/card";
import { Field } from "~/components/field";
import { changeViewUI, ViewUIType } from "./view";

export function LoginUI() {
    return (
        <Card.Root class="flex-auto shadow-none">
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
                            if (e.currentTarget.value.length) {
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
                    />
                </Field.Root>
            </Card.Body>
            <Card.Footer gap="3">
                <Button variant="outline" on:click={() => getCurrentWindow().close()}>退出</Button>
                <Button
                    id="login_button" disabled
                    on:click={async () => {
                        try {
                            await invoke("login", {
                                loginName: (document.getElementById("login_name_input") as HTMLInputElement).value,
                            });
                            changeViewUI(ViewUIType.ChatUI);
                            await getCurrentWindow().setSize(new PhysicalSize(1100, 700));
                            getCurrentWindow().center();
                        } catch (err) {
                            error(`${err}`);
                        }
                    }}
                >登录</Button>
            </Card.Footer>
        </Card.Root>
    );
}
