export function convertToUint8Array(string: string): Uint8Array {
    return Uint8Array.from(Array.from(string).map((letter) => letter.charCodeAt(0)));
}