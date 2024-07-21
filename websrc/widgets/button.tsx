import { v4 as uuidv4 } from "uuid";

interface OptionStyle {
    ed: string;
    un: string;
}

interface ButtonStyle {
    base: string;
    select: OptionStyle;
    hover: OptionStyle;
}

export class Button {
    private uuid: string;
    private style: ButtonStyle;

    constructor(style: ButtonStyle) {
        this.uuid = uuidv4();
        this.style = style;
    }
    id(): string {
        return this.uuid;
    }
    selectedStyle(): string {
        return this.style.base + " " + this.style.select.ed;
    }
    unselectedStyle(): string {
        return this.style.base + " " + this.style.select.un;
    }
}

export class ButtonGroup {
    private buttons: Button[];

    constructor(buttons: Button[]) {
        this.buttons = buttons;
    }
    add(button: Button) {
        this.buttons.push(button);
    }
    select(button: Button) {
        const element = document.getElementById(button.id())!;
        if (element.className != button.selectedStyle()) {
            this.buttons.forEach((button) => document.getElementById(button.id())!.className = button.unselectedStyle());
            element.className = button.selectedStyle();
        }
    }
}
