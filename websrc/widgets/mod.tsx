import { v4 as uuidv4 } from "uuid";
import { JSX } from "solid-js"

export class Widget {
    protected element: HTMLElement;

    constructor(tag: string = "div", style: string = "") {
        this.element = document.createElement(tag);
        this.element.id = uuidv4();
        this.element.className = style;
    }
    addStyle(style: string) {
        this.element.className += style.length != 0 ? " " + style : "";
    }
    withStyle(style: string) {
        this.addStyle(style);
        return this;
    }
    html(child: JSX.Element = <></>) {
        this.element.append(child as HTMLElement);
        return this.element as JSX.Element;
    }
}
