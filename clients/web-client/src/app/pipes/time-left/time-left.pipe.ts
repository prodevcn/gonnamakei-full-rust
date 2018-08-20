import {Injectable, Pipe, PipeTransform} from "@angular/core";

@Injectable() @Pipe({
    name: "timeLeft",
    pure: false,
})
export class TimeLeftPipe implements PipeTransform {

    transform(startDateTime: string, totalTimeMs: number): number {
        const start = new Date(startDateTime);
        const end = new Date(start.getTime() + totalTimeMs);
        const now = new Date();

        if (now <= end) {
            const remainingMilliseconds = end.getTime() - now.getTime();
            return remainingMilliseconds;
        }
        return 0;
    }

}
