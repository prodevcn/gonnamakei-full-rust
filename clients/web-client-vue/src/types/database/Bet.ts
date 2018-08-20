import {APIReference} from "./Reference";
import {ChallengeAPIDocument} from "./Challenge";

export interface BetAPIDocument {
    id: string | null,
    userAddress?: string | null,
    betTransaction?: string | null,
    challenge?: APIReference<ChallengeAPIDocument> | null,
    betMoney?: number | null,
}

// This type is SerializableBet in backend.
export interface BetBlockchainData {
    ownerAccount: string,
    receiverAccount?: string | null,
    receiverAuthorityAccount?: string | null,
    state: BetBlockchainState,
    bumpSeed: number,
    amount: number,
    wonAmount: number,
    // Should be a Date.
    appliedAt: number,
    // Should be a Date.
    expiresAt: number,
    fungibleTokenAccount: string,
}

// This type is SerializedBetState in backend.
export enum BetBlockchainState {
    Uninitialized = "uninitialized", Initiated = "initiated", Applied = "applied", Won = "won",
}