import {Blockhash} from "@solana/web3.js";

export interface BetSendRequestBody {
    signature: string,
    recentBlockHash: Blockhash,
}

export interface BetSendResponse {
    startTime: number,
    timeout: number,
}

export enum ChallengeCheckResponseStatus {
    Won = "won", Lost = "lost", NotInitiated = "notInitiated", Initiated = "initiated", Expired = "expired",
}

export interface ChallengeCheckResponse {
    status: ChallengeCheckResponseStatus,
}