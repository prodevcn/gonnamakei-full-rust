import {Component, Input, OnInit} from "@angular/core";
import {WaitlistPromptComponent} from "../waitlist-prompt/waitlist-prompt.component";

import {GameComingSoonPipe} from "../../pipes/game-coming-soon/game-coming-soon.pipe";
import {ChallengeGamePipe} from "../../pipes/challenge-game/challenge-game.pipe";

@Component({
    selector: "app-challenge2",
    templateUrl: "./challenge2.component.html",
    styleUrls: ["./challenge2.component.scss"],
})
export class Challenge2Component implements OnInit {

    WaitlistPromptComponent = WaitlistPromptComponent;
    @Input() challenge: any;
    isAvailable = true;
    imageCollection = ["/assets/images/nft-collection/Hand5.svg", "/assets/images/nft-collection/Hand4.svg",
        "/assets/images/nft-collection/Hand3.svg", "/assets/images/nft-collection/Hand2.svg",
        "/assets/images/nft-collection/Hand1.svg"];

    constructor(private challengeGamePipe: ChallengeGamePipe, private gameComingSoonPipe: GameComingSoonPipe) {
    }

    ngOnInit(): void {

        this.isAvailable = !this.gameComingSoonPipe.transform(this.challengeGamePipe.transform(this.challenge));
    }

}
