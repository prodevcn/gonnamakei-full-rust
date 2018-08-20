import {Blockchain} from "../../database/Blockchain";
import {ChallengeAPIDocument, ChallengeBlockchainData} from "../../database/Challenge";
import {PaginatedRequest} from "../APITypes";

export interface ChallengeCreateRequestBody {
    blockchain: Blockchain,
    blockchain_address: string,
}

export interface ChallengeGetRequestBody {
    returnFields?: boolean;
    returnBlockchainData?: boolean;
}

export interface ChallengeGetResponse {
    challenge?: ChallengeAPIDocument;
    blockchainInfo?: ChallengeBlockchainData;
}

export interface ChallengeListRequestBody {
    responses?: PaginatedRequest<ChallengeAPIDocument>;
}

export interface ChallengeBetCreateResponse {
    bet: string,
    message: string,
    fee: number,
}

export interface ChallengeBetCreateRequestBody {
    participant: string;
}