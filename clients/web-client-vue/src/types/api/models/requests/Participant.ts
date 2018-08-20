import {ParticipantGamesData} from "src/types/database/Participant";

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
