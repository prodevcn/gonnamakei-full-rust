export interface BetSendResponse {
    startTime: number,
    timeout: number,
}

export enum BetCheckResponseStatus {
    Won = "won", Lost = "lost", NotInitiated = "notInitiated", Initiated = "initiated", Expired = "expired",
}

export interface BetCheckResponse {
    status: BetCheckResponseStatus,
}