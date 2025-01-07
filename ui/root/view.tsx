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
                        <Field.Input id="login_name" autocomplete="off" placeholder="取一个喜欢的名称吧" />
                    </Field.Root>
                </Card.Body>
                <Card.Footer gap="3">
                    <Button variant="outline" on:click={() => getCurrentWindow().close()}>退出</Button>
                    <Button
                        on:click={() => {
                            const loginName = (document.getElementById("login_name") as HTMLInputElement).value;
                            if (loginName.length) invoke("login", { loginName }).catch((err) => captureError(err));
                        }}
                    >登录</Button>
                </Card.Footer>
            </Card.Root>
        </ErrorCapture>
    );
}
