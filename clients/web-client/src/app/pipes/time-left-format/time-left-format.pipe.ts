import {Pipe, PipeTransform} from "@angular/core";

@Pipe({
    name: "timeLeftFormat",
    pure: false,
})
export class TimeLeftFormatPipe implements PipeTransform {

    transform(milliseconds: number, full?: boolean): string {
        const fullSeconds = milliseconds / 1000;

        const minutes = Math.floor(fullSeconds / 60);
        const seconds = Math.round(fullSeconds - (minutes * 60));

        if (!full) {
            return `${this.padZero(minutes)}:${this.padZero(seconds)}`;
        } else if (full) {

            if (seconds) {
                return `${this.padZero(minutes)} minutes ${this.padZero(seconds)} seconds`;
            } else {
                return `${this.padZero(minutes)} minutes`;
            }

        }
    }

    padZero(a: number): string {
        if (a < 10) {
            return `0${a}`
        }
        return `${a}`;
    }

}
