import {Injectable} from "@angular/core";
import {ActivatedRouteSnapshot, CanActivate, Router, RouterStateSnapshot} from "@angular/router";
import {ChallengeService} from "../services/http";
import {DataService} from "../services/data";
import {BetService} from "../services/http/bet/bet.service";

@Injectable({
    providedIn: "root",
})
export class GamePlayGuard implements CanActivate {

    constructor(private challengeService: ChallengeService, private betService: BetService, private data: DataService,
                private router: Router) {
    }

    async canActivate(route: ActivatedRouteSnapshot, state: RouterStateSnapshot): Promise<boolean> {
        const betId = route.params.betId;
        const betSendResponse = await this.betService.getBetSend();
        const challengeBet = await this.challengeService.getChallengeBet();
        const canProceed = !!(betSendResponse && challengeBet);

        if (!canProceed && this.data.routePaths.length === 0) {
            this.router.navigate(["/"]);
        }
        return canProceed;

    }

}
