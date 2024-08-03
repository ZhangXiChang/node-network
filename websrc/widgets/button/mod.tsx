import { Widget } from "../mod";

export interface ButtonStyle {
    base: string;
    hovered: string;
    selected: string;
}

export class Button extends Widget {
    private buttonStyle: ButtonStyle;

    constructor(style: ButtonStyle, tag: string = "div") {
        super(tag);
        this.buttonStyle = style;
        this.element.className = this.baseStyle();
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
    withClickEvent(onClick: (e: MouseEvent) => void) {
        this.addClickEvent(onClick);
        return this;
    }
    addHoverEvent(onHover: (e: MouseEvent, isHovered: boolean) => void) {
        this.element.addEventListener("mouseenter", (e) => onHover(e, true));
        this.element.addEventListener("mouseleave", (e) => onHover(e, false));
    }
    withHoverEvent(onHover: (e: MouseEvent, isHovered: boolean) => void) {
        this.addHoverEvent(onHover);
        return this;
    }
    baseStyle() {
        return "flex justify-center items-center" + " " + this.buttonStyle.base;
    }
    hoveredStyle() {
        return this.baseStyle() + " " + this.buttonStyle.hovered;
    }
    selectedStyle() {
        return this.baseStyle() + " " + this.buttonStyle.selected;
    }
    setStyleToBase() {
        this.element.className = this.baseStyle();
    }
    withStyleToBase() {
        this.setStyleToBase();
        return this;
    }
    setStyleToHovered() {
        this.element.className = this.hoveredStyle();
    }
    withStyleToHovered() {
        this.setStyleToHovered();
        return this;
    }
    setStyleToSelected() {
        this.element.className = this.selectedStyle();
    }
    withStyleToSelected() {
        this.setStyleToSelected();
        return this;
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

export function buttonNavigation(buttons: Button[]) {
    buttons.forEach((selfButton) => selfButton.addClickEvent(() => buttons.forEach((button) => {
        if (button != selfButton) {
            if (button.isSelectedStyle()) {
                button.setStyleToBase();
            }
        }
    })));
}
