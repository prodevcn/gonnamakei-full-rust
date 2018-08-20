import {PaginatedRequest} from "src/types/api/APITypes";
import {ChallengeAPIDocument} from "src/types/database/Challenge";

export interface ChallengeGetRequestBody {
    returnFields?: boolean;
    returnBlockchainData?: boolean;
}

export interface ChallengeListRequestBody {
    responses?: PaginatedRequest<ChallengeAPIDocument>;
}

export interface ChallengeBetCreateRequestBody {
    participant: string;
}