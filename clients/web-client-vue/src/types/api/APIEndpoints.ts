import {APIError, PaginatedResponse} from "src/types/api/APITypes";
import {SignatureRequestBody} from "src/types/api/models/requests/Signature";
import {SignatureRequestResponse} from "src/types/api/models/responses/Signature";
import {API} from "boot/axios";
import {
    ParticipantGetRequestBody, ParticipantLoginRequestBody, ParticipantUpdateRequestBody,
} from "src/types/api/models/requests/Participant";
import {ParticipantGetResponse, ParticipantLoginResponse} from "src/types/api/models/responses/Participant";
import {APIAuthentication, AxiosRequestOptions} from "src/plugins/api";
import {ParticipantAPIDocument} from "src/types/database/Participant";
import {
    ChallengeBetCreateRequestBody, ChallengeGetRequestBody, ChallengeListRequestBody,
} from "src/types/api/models/requests/Challenge";
import {ChallengeBetCreateResponse, ChallengeGetResponse} from "src/types/api/models/responses/Challenge";
import {ChallengeAPIDocument} from "src/types/database/Challenge";
import {
    ClashRoyaleArenaGameDataResponse, ClashRoyaleCardGameDataResponse,
} from "src/types/api/models/responses/data/games/ClashRoyale";
import {BetSendRequestBody} from "src/types/api/models/requests/Bet";
import {BetCheckResponse, BetSendResponse} from "src/types/api/models/responses/Bet";
import {Store} from "src/stores";

export const APIEndpoints = {
    signature: {
        async request(body: SignatureRequestBody): Promise<SignatureRequestResponse | APIError> {
            return API.post("signature/request", body);
        },
    },
    participant: {
        async login(body: ParticipantLoginRequestBody): Promise<ParticipantLoginResponse | APIError> {
            return API.post("participant/login", body);
        },
        async logout(): Promise<undefined | APIError> {
            let result = await API.post<undefined, undefined | APIError>("participant/logout", undefined, {
                authentication: APIAuthentication.Token,
            });

            Store.cleanPrivateData();

            return result;
        },
        async get(body: ParticipantGetRequestBody): Promise<ParticipantGetResponse | APIError> {
            return API.post("participant/get", body, {
                authentication: APIAuthentication.Token,
            });
        },
        async update(body: ParticipantUpdateRequestBody): Promise<ParticipantAPIDocument | APIError> {
            return API.post("participant/update", body, {
                authentication: APIAuthentication.Token,
            });
        },
    },
    challenge: {
        async get(address: string, body: ChallengeGetRequestBody): Promise<ChallengeGetResponse | APIError> {
            return API.post(`challenge/${address}`, body);
        },
        async list(body: ChallengeListRequestBody): Promise<PaginatedResponse<ChallengeAPIDocument> | APIError> {
            return API.post(`challenge/list`, body);
        },
        async bet(id: string,
                  body: ChallengeBetCreateRequestBody): Promise<PaginatedResponse<ChallengeBetCreateResponse> | APIError> {
            return API.post(`challenge/${id}/bet`, body, {
                authentication: APIAuthentication.Token,
            });
        },
    },
    bet: {
        async send(id: string, body: BetSendRequestBody): Promise<BetSendResponse | APIError> {
            return API.post(`bet/${id}/send`, body, {
                authentication: APIAuthentication.Token,
            });
        },
        async check(id: string): Promise<PaginatedResponse<BetCheckResponse> | APIError> {
            return API.post(`bet/${id}/check`, undefined);
        },
    },
    data: {
        games: {
            clashRoyale: {
                async cards(options: AxiosRequestOptions = {}): Promise<ClashRoyaleCardGameDataResponse[] | APIError> {
                    return API.post(`data/games/clash_royale/cards`, undefined, options);
                },
                async arenas(options: AxiosRequestOptions = {}): Promise<ClashRoyaleArenaGameDataResponse[] | APIError> {
                    return API.post(`data/games/clash_royale/arenas`, undefined, options);
                },
            },
        },
    },
};
