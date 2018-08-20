export function getNameFromEnum(enumObject: { [key: string]: string }, value: string): string | null {
    for (let key in enumObject) {
        let v = enumObject[key];

        if (v === value) {
            return key;
        }
    }

    return null;
}