import {Injectable} from "@angular/core";
import {ActivatedRouteSnapshot, Resolve, RouterStateSnapshot} from "@angular/router";

import {ChallengeService} from "../services/http";
import {ChallengeBetCreateResponse} from "../types/api/models/Challenge";

@Injectable({
    providedIn: "root",
})
export class ChallengeBetResolver implements Resolve<ChallengeBetCreateResponse> {

    constructor(private challengeService: ChallengeService) {
    }

    resolve(route: ActivatedRouteSnapshot, state: RouterStateSnapshot): Promise<ChallengeBetCreateResponse> {
        return this.challengeService.getChallengeBet();
    }
}
