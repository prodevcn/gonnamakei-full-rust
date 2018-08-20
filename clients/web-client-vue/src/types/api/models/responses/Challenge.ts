import {ChallengeAPIDocument, ChallengeBlockchainData} from "src/types/database/Challenge";

export interface ChallengeGetResponse {
    challenge?: ChallengeAPIDocument;
    blockchainInfo?: ChallengeBlockchainData;
}

export interface ChallengeBetCreateResponse {
    bet: string,
    message: string,
    fee: number,
}