import {Component, OnInit} from "@angular/core";
import {ChallengeService} from "../../services/http";
import {ChallengeListRequestBody} from "../../types/api/models/Challenge";
import {ChallengeAPIDocument} from "../../types/database/Challenge";

@Component({
    selector: "app-challenge-list",
    templateUrl: "./challenge-list.component.html",
    styleUrls: ["./challenge-list.component.scss"],
})
export class ChallengeListComponent implements OnInit {

    challenges = [] as Array<ChallengeAPIDocument>;

    constructor(private challengeService: ChallengeService) {
    }

    ngOnInit(): void {
        this.loadChallenges();
    }

    async loadChallenges() {
        //TODO
        const params = {
            responses: {
                page: 0,
                rowsPerPage: 100,
            },
        } as ChallengeListRequestBody;
        const {results: challenges} = await this.challengeService.listChallenges(params);
        this.challenges = challenges;
    }

}
