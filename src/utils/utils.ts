import { MouseEvent } from "react";

export const prevent_default = (event: MouseEvent) => event.preventDefault();

export const bytes_to_image = (bytes: Uint8Array, type: string) => {
    const binary = Array.prototype.map.call(bytes, (byte: number) => String.fromCharCode(byte)).join("");
    const base64 = btoa(binary);

    return `data:image/${type};base64,${base64}`;
}