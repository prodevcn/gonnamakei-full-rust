import {Injectable} from "@angular/core";
import {ActivatedRouteSnapshot, Resolve, RouterStateSnapshot} from "@angular/router";

import {ChallengeService} from "../services/http";
import {ChallengeGetResponse} from "../types/api/models/Challenge";

@Injectable({
    providedIn: "root",
})
export class ChallengeResolver implements Resolve<ChallengeGetResponse> {

    constructor(private challengeService: ChallengeService) {
    }

    resolve(route: ActivatedRouteSnapshot, state: RouterStateSnapshot): Promise<ChallengeGetResponse> {
        let address = route.params.id.substring(3);
        return this.challengeService.getChallenge(address, {
            returnFields: true,
            returnBlockchainData: true,
        });
    }
}
