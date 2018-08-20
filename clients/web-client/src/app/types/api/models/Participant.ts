import {ParticipantAPIDocument, ParticipantGamesData} from "../../database/Participant";
import {BetBlockchainData} from "../../database/Bet";
import {ChallengeBlockchainData} from "../../database/Challenge";

// REQUESTS -------------------------------------------------------------------

export interface ParticipantLoginRequestBody {
    id: string,
    signature: string,
}

export interface ParticipantGetRequestBody {
    returnFields: boolean,
    returnActiveBets: boolean,
}

export interface ParticipantUpdateRequestBody {
    gamesData: ParticipantGamesData;
}

// RESPONSES ------------------------------------------------------------------

export interface ParticipantLoginResponse {
    token: string,
}

export interface ParticipantGetResponse {
    participant?: ParticipantAPIDocument | null;
    activeBets?: ParticipantActiveBetGetResponse[] | null;
}

export interface ParticipantActiveBetGetResponse {
    betKey?: string;
    bet?: BetBlockchainData;
    challenge?: ChallengeBlockchainData;
}
