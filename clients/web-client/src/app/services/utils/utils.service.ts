import {Injectable} from "@angular/core";

@Injectable({
    providedIn: "root",
})
export class UtilsService {

    constructor() {
    }

    scrollToHomeChallenges() {
        const challengesContainer = document.querySelector("[challenges-container]");
        challengesContainer?.scrollIntoView({
            behavior: "smooth",
            inline: "start",
        });
    }
}
