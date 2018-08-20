import {Injectable} from "@angular/core";
import {HttpClient} from "@angular/common/http";
import {PaginatedRequest} from "../../../types/api/APITypes";

import {
    ChallengeBetCreateRequestBody,
    ChallengeBetCreateResponse,
    ChallengeGetRequestBody,
    ChallengeGetResponse,
    ChallengeListRequestBody,
} from "../../../types/api/models/Challenge";
import {ChallengeAPIDocument} from "../../../types/database/Challenge";

import {environment} from "../../../../environments/environment";
import {LoadingController} from "@ionic/angular";
import {ParticipantDataService} from "../../data";

const {API_SERVER_URL} = environment;

@Injectable({
    providedIn: "root",
})
export class ChallengeService {
    private challengeBet: ChallengeBetCreateResponse | null = null;

    constructor(private http: HttpClient, private loadCtrl: LoadingController,
                private participantData: ParticipantDataService) {
    }

    async getChallenge(address: string, params: ChallengeGetRequestBody): Promise<ChallengeGetResponse> {
        const loader = await this.startLoader();
        loader.present();

        return this.http.post<ChallengeGetResponse>(`${API_SERVER_URL}/challenge/${address}`,
            params).toPromise().finally(() => loader.dismiss());
    }

    async listChallenges(params: ChallengeListRequestBody): Promise<PaginatedRequest<ChallengeAPIDocument[]>> {
        const loader = await this.startLoader();
        loader.present();

        return this.http.post<PaginatedRequest<ChallengeAPIDocument[]>>(`${API_SERVER_URL}/challenge/list`,
            params).toPromise().finally(() => loader.dismiss());

    }

    async bet(id: string, params: ChallengeBetCreateRequestBody): Promise<ChallengeBetCreateResponse> {
        const loader = await this.startLoader();
        loader.present();

        const challengeId = id.substr(3);
        return this.http.post<ChallengeBetCreateResponse>(`${API_SERVER_URL}/challenge/${challengeId}/bet`, params, {
            headers: {
                "authorization": "GMI " + this.participantData.apiToken,
            },
        }).toPromise()
            .then(data => {
                this.challengeBet = data;
                return data;
            }).finally(() => loader.dismiss());
    }

    getChallengeBet(): Promise<ChallengeBetCreateResponse> {
        return Promise.resolve(this.challengeBet);
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