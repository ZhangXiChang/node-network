import { Card } from "~/components/card";
import { captureError, ErrorCapture } from "./error_capture";
import { Field } from "~/components/field";
import { Button } from "~/components/button";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { invoke } from "@tauri-apps/api/core";

export function View() {
    return (
        <ErrorCapture>
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
                        on:click={() => invoke("login", {
                            loginName: (document.getElementById("login_name_input") as HTMLInputElement).value,
                        }).catch((err) => captureError(err))}
                    >登录</Button>
                </Card.Footer>
            </Card.Root>
        </ErrorCapture>
    );
}
