export interface ParticipantAPIDocument {
    id: string | null,
    gamesData?: ParticipantGamesData | null,
}

export interface ParticipantGamesData {
    clashRoyale?: ParticipantClashRoyaleGameData | null,
}

export interface ParticipantClashRoyaleGameData {
    tag?: string | null,
}