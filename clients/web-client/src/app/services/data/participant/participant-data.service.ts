import {EventEmitter, Injectable} from "@angular/core";
import {ParticipantAPIDocument} from "../../../types/database/Participant";

const API_TOKEN_KEY = "T";

@Injectable({
    providedIn: "root",
})
export class ParticipantDataService {
    private _apiToken: string | null = null;
    private _participant: ParticipantAPIDocument | null = null;

    onParticipantLoaded = new EventEmitter<ParticipantAPIDocument>();
    onParticipantUnloaded = new EventEmitter<ParticipantAPIDocument>();
    onLogout = new EventEmitter<string>();

    constructor() {
        this._apiToken = localStorage.getItem(API_TOKEN_KEY);
    }

    get apiToken(): string | null {
        return this._apiToken;
    }

    set apiToken(value: string | null) {
        if (this._apiToken === value) {
            return;
        }

        let oldValue = this._apiToken;
        this._apiToken = value;

        if (value == null) {
            localStorage.removeItem(API_TOKEN_KEY);
            this.onLogout.emit(oldValue);
        } else {
            localStorage.setItem(API_TOKEN_KEY, value);
        }
    }

    get participant(): ParticipantAPIDocument | null {
        return this._participant;
    }

    set participant(value: ParticipantAPIDocument | null) {
        if (this._participant === value) {
            return;
        }

        if (value == null) {
            this.onParticipantUnloaded.emit();
        } else {
            this.onParticipantLoaded.emit(value);
        }

        this._participant = value;
    }

    get isLoggedIn(): boolean {
        return this._apiToken != null;
    }
}