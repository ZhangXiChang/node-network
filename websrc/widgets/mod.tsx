import { v4 as uuidv4 } from "uuid";
import { JSX } from "solid-js"

export class Widget {
    protected element: HTMLElement;

    constructor(tag: string = "div") {
        this.element = document.createElement(tag);
        this.element.id = uuidv4();
    }
    html(child: JSX.Element = <></>) {
        this.element.append(child as HTMLElement);
        return this.element as JSX.Element;
    }
}
