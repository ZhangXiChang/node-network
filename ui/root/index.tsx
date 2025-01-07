import { Window } from "./window";
import { Button } from "~/components/button";
import { Card } from "~/components/card";
import { Field } from "~/components/field";
import { ErrorCapture } from "./error_capture";
import { getCurrentWindow } from "@tauri-apps/api/window";

export function Root() {
    return (
        <Window>
            <ErrorCapture>
                <Card.Root class="flex-auto shadow-none">
                    <Card.Header>
                        <Card.Title>欢迎使用</Card.Title>
                        <Card.Description>填写个性化信息</Card.Description>
                    </Card.Header>
                    <Card.Body>
                        <Field.Root>
                            <Field.Label>名称</Field.Label>
                            <Field.Input placeholder="取一个喜欢的名称吧" />
                        </Field.Root>
                    </Card.Body>
                    <Card.Footer gap="3">
                        <Button variant="outline" on:click={() => getCurrentWindow().close()}>退出</Button>
                        <Button>登录</Button>
                    </Card.Footer>
                </Card.Root>
            </ErrorCapture>
        </Window>
    );
}
