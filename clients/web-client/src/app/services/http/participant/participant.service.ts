import {Injectable} from "@angular/core";
import {HttpClient} from "@angular/common/http";

import {
    ParticipantGetRequestBody,
    ParticipantGetResponse,
    ParticipantLoginRequestBody,
    ParticipantLoginResponse,
    ParticipantUpdateRequestBody,
} from "../../../types/api/models/Participant";
import {ParticipantAPIDocument} from "../../../types/database/Participant";

import {environment} from "../../../../environments/environment";
import {ParticipantDataService} from "../../data";
import {APIError} from "../../../types/api/APITypes";

const {API_SERVER_URL} = environment;

@Injectable({
    providedIn: "root",
})
export class ParticipantService {

    constructor(private http: HttpClient, private participantData: ParticipantDataService) {
        participantData.onLogout.subscribe((oldApiToken) => {
            try {
                this.logoutParticipant(oldApiToken);
            } catch (e) {
            }
        });
    }

    async loginParticipant(params: ParticipantLoginRequestBody): Promise<void> {
        let response = await this.http.post<ParticipantLoginResponse>(`${API_SERVER_URL}/participant/login`,
            params).toPromise();

        this.participantData.apiToken = response.token;
    }

    async logoutParticipant(apiToken: string): Promise<void> {
        try {
            await this.http.post<void>(`${API_SERVER_URL}/participant/logout`, null, {
                headers: {
                    "authorization": "GMI " + apiToken,
                },
            }).toPromise();
        } catch (e) {
        } finally {
            this.participantData.apiToken = null;
        }
    }

    getParticipant(params: ParticipantGetRequestBody): Promise<ParticipantGetResponse> {
        if (!this.participantData.isLoggedIn) {
            throw {
                errorCode: "not_logged_in",
            } as APIError;
        }

        return this.http.post<ParticipantGetResponse>(`${API_SERVER_URL}/participant/get`, params, {
            headers: {
                "authorization": "GMI " + this.participantData.apiToken,
            },
        }).toPromise();
    }

    updateParticipant(params: ParticipantUpdateRequestBody): Promise<ParticipantAPIDocument> {
        if (!this.participantData.isLoggedIn) {
            throw {
                errorCode: "not_logged_in",
            } as APIError;
        }

        return this.http.post<ParticipantAPIDocument>(`${API_SERVER_URL}/participant/update`, params, {
            headers: {
                "authorization": "GMI " + this.participantData.apiToken,
            },
        }).toPromise();
    }
}