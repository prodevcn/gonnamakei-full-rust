import {Injectable} from "@angular/core";
import {HttpClient} from "@angular/common/http";

import {BetSendRequestBody, BetSendResponse, ChallengeCheckResponse} from "../../../types/api/models/Bet";

import {environment} from "../../../../environments/environment";
import {LoadingController} from "@ionic/angular";
import {ParticipantDataService} from "../../data";

const {API_SERVER_URL} = environment;

@Injectable({
    providedIn: "root",
})
export class BetService {
    private betId: string | null = null;
    private betSend: BetSendResponse | null = null;

    constructor(private http: HttpClient, private loadCtrl: LoadingController,
                private participantData: ParticipantDataService) {
    }

    async checkSentBet(): Promise<ChallengeCheckResponse> {
        return this.http.post<ChallengeCheckResponse>(`${API_SERVER_URL}/bet/${this.betId}/check`,
            undefined).toPromise();
    }

    async sendBet(id: string, params: BetSendRequestBody): Promise<BetSendResponse> {
        const loader = await this.startLoader();
        loader.present();

        return this.http.post<BetSendResponse>(`${API_SERVER_URL}/bet/${id}/send`, params, {
            headers: {
                "authorization": "GMI " + this.participantData.apiToken,
            },
        }).toPromise()
            .then(data => {
                this.betId = id;
                this.betSend = data;
                return data;
            }).finally(() => loader.dismiss());
    }

    getBetSend(): Promise<BetSendResponse> {
        return Promise.resolve(this.betSend);
    }

    async startLoader(...args) {
        return await this.loadCtrl.create({
            backdropDismiss: false,
            showBackdrop: true,
            translucent: true,
            mode: "md",
            cssClass: "loader-custom", ...args,
        });
    }
}