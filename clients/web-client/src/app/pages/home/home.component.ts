import {Component, ElementRef, OnInit, ViewChild} from "@angular/core";
import {WaitlistPromptComponent} from "../../components/waitlist-prompt/waitlist-prompt.component";

import {ChallengeService} from "../../services/http";

import {ChallengeListRequestBody} from "../../types/api/models/Challenge";
import {ChallengeAPIDocument} from "../../types/database/Challenge";

@Component({
    selector: "app-home",
    templateUrl: "./home.component.html",
    styleUrls: ["./home.component.scss"],
})
export class HomeComponent implements OnInit {

    WaitlistPromptComponent = WaitlistPromptComponent;
    challenges: ChallengeAPIDocument[] = [];
    availableChallenges: ChallengeAPIDocument[] = [];

    @ViewChild("challengeScroller", {static: true}) challengeScroller: ElementRef<HTMLElement>;

    constructor(private challengeService: ChallengeService) {

    }

    ngOnInit(): void {
        this.loadChallenges();
    }

    async loadChallenges() {
        const params = {
            responses: {
                page: 0,
                rowsPerPage: 100,
            },
        } as ChallengeListRequestBody;
        const {results: challenges} = await this.challengeService.listChallenges(params);
        this.challenges = [...challenges];
        this.availableChallenges = challenges.map(challenge => ({
            ...challenge,
            available: true,
        }));
    }

    scrollChallenges(direction: "right" | "left") {
        const scroller = this.challengeScroller.nativeElement;
        const scrollWidth = Math.max(scroller.firstElementChild?.getBoundingClientRect()?.width, 100);
        const left = (direction === "right" ? 1 : -1) * scrollWidth;
        scroller.scrollBy({
            left,
            behavior: "smooth",
        });
    }
}
