import {ParticipantAPIDocument} from "src/types/database/Participant";
import {BetBlockchainData} from "src/types/database/Bet";
import {ChallengeBlockchainData} from "src/types/database/Challenge";

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