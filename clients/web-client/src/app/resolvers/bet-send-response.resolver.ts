import {Injectable} from "@angular/core";
import {ActivatedRouteSnapshot, Resolve, RouterStateSnapshot} from "@angular/router";
import {BetSendResponse} from "../types/api/models/Bet";
import {BetService} from "../services/http/bet/bet.service";

@Injectable({
    providedIn: "root",
})
export class BetSendResponseResolver implements Resolve<BetSendResponse> {

    constructor(private betService: BetService) {
    }

    resolve(route: ActivatedRouteSnapshot, state: RouterStateSnapshot): Promise<BetSendResponse> {
        return this.betService.getBetSend();
    }
}
