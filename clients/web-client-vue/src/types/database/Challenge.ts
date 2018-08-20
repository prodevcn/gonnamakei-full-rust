import {ValuedNFT} from "./ValuedNFT";
import {APIReference} from "./Reference";
import {GameMilestone} from "./milestones/Game";

export interface ChallengeAPIDocument {
    id: string | null,
    createdAt: number | null,
    name?: string | null,
    description?: string | null,
    nftImageUrl?: string | null,
    blockchainInfo: ChallengeBlockchainData,
    walletAddress?: string | null,
    creatorAddress?: string | null,
    milestones?: ChallengeMilestone[] | null,
    investedNfts?: ValuedNFT[] | null,
    maxBet?: number | null,
    rewardMultiplier?: number | null,
}

export type ChallengeMilestone = ChallengeMilestone_Game | ChallengeMilestone_Other;

export interface ChallengeMilestone_Game {
    T: "gameMilestone",
    V: GameChallengeMilestone
}

export interface ChallengeMilestone_Other {
    T: "otherChallenge",
    V: OtherChallengeMilestone
}

export interface GameChallengeMilestone {
    milestone: GameMilestone;
}

export interface OtherChallengeMilestone {
    challenge: APIReference<ChallengeAPIDocument>;
}

// This type is SerializableChallenge in backend.
export interface ChallengeBlockchainData {
    creatorAccount: string,
    validatorAccount: string,
    state: ChallengeBlockchainState,
    bumpSeed: number,
    url: string,
    authorizedInvestments: boolean,
    authorizedBets: boolean,
    allowRedeemManyNfts: boolean,
    betsExpirationDelay: number,
    minBetAmount: number,
    maxBetAmount: number,
    rewardTimes: number,
    wins: number,
    losses: number,
    expirations: number,
    totalNfts: number,
    maxNfts: number,
    investments: number,
    maxInvestments: number,
    minInvestmentAmount: number,
    maxInvestmentAmount: number,
    totalInvested: number,
    maxFungibleTokens: number,
    parallelBets: number,
    maxParallelBets: number,
    historyBets: number,
    maxHistoryBets: number,
    tokenAccumulatorAccount: string,
    creatorFeeAccount: string,
    betFee: number,
    betFeePercentage: number,
    investmentFee: number,
    investmentFeePercentage: number,
    withdrawInvestmentFee: number,
    withdrawInvestmentFeePercentage: number,
}

// This type is SerializedChallengeState in backend.
export enum ChallengeBlockchainState {
    Uninitialized = "uninitialized", Initiated = "initiated", Active = "active",
}