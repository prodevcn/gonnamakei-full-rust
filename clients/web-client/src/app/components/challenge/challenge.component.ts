import {Component, Input, OnInit} from "@angular/core";
import {WaitlistPromptComponent} from "../waitlist-prompt/waitlist-prompt.component";

import {GameComingSoonPipe} from "../../pipes/game-coming-soon/game-coming-soon.pipe";
import {ChallengeGamePipe} from "../../pipes/challenge-game/challenge-game.pipe";
import {ChallengeAPIDocument} from "../../types/database/Challenge";

@Component({
    selector: "app-challenge",
    templateUrl: "./challenge.component.html",
    styleUrls: ["./challenge.component.scss"],
})
export class ChallengeComponent implements OnInit {

    WaitlistPromptComponent = WaitlistPromptComponent;
    @Input() challenge: ChallengeAPIDocument;
    isAvailable = true;
    @Input() activeBtn = false;

    constructor(private challengeGamePipe: ChallengeGamePipe, private gameComingSoonPipe: GameComingSoonPipe) {
    }

    ngOnInit(): void {
        this.isAvailable = !this.gameComingSoonPipe.transform(this.challengeGamePipe.transform(this.challenge));
    }

}
