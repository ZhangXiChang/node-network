import { Widget } from "../mod";

export interface ButtonStyle {
    base: string;
    hovered: string;
    selected: string;
}

export class Button extends Widget {
    private style: ButtonStyle;

    constructor(style: ButtonStyle, tag: string = "div") {
        super(tag)
        this.style = style;
        this.element.className = this.style.base;
        this.addClickEvent(() => {
            if (!this.isSelectedStyle()) {
                this.setStyleToSelected();
            }
        });
        this.addHoverEvent((_, isHovered) => {
            if (this.isBaseStyle() || this.isHoveredStyle()) {
                isHovered ? this.setStyleToHovered() : this.setStyleToBase();
            }
        });
    }
    addClickEvent(onClick: (e: MouseEvent) => void) {
        this.element.addEventListener("click", onClick);
    }
    addHoverEvent(onHover: (e: MouseEvent, isHovered: boolean) => void) {
        this.element.addEventListener("mouseenter", (e) => onHover(e, true));
        this.element.addEventListener("mouseleave", (e) => onHover(e, false));
    }
    baseStyle() {
        return this.style.base;
    }
    hoveredStyle() {
        return this.baseStyle() + " " + this.style.hovered;
    }
    selectedStyle() {
        return this.baseStyle() + " " + this.style.selected;
    }
    setStyleToBase() {
        this.element.className = this.baseStyle();
    }
    setStyleToHovered() {
        this.element.className = this.hoveredStyle();
    }
    setStyleToSelected() {
        this.element.className = this.selectedStyle();
    }
    isBaseStyle() {
        return this.element.className == this.baseStyle();
    }
    isHoveredStyle() {
        return this.element.className == this.hoveredStyle();
    }
    isSelectedStyle() {
        return this.element.className == this.selectedStyle();
    }
}

export class ButtonNavigation {
    private buttons: Button[];

    constructor(buttons: Button[] = []) {
        this.buttons = buttons;
        this.buttons.forEach((button) => {
            button.addClickEvent(() => this.setOtherButtonStyleToBase(button));
        });
    }
    setOtherButtonStyleToBase(button: Button) {
        this.buttons.forEach((allbutton) => {
            if (allbutton != button) {
                if (allbutton.isSelectedStyle()) {
                    allbutton.setStyleToBase();
                }
            }
        });
    }
    addButton(button: Button) {
        this.buttons.push(button);
        button.addClickEvent(() => this.setOtherButtonStyleToBase(button))
    }
}
