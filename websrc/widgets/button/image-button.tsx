import { Button, ButtonStyle } from "./mod";

export class ImageButton extends Button {
    constructor(style: ButtonStyle, imgUrl: string, width: number, height: number) {
        super(style, "img");
        const element = this.element as HTMLImageElement;
        element.src = imgUrl;
        element.width = width;
        element.height = height;
    }
}
