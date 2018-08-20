import {Injectable} from "@angular/core";
import {HttpClient} from "@angular/common/http";

import {ClashRoyaleArenaGameDataResponse} from "../../../types/api/models/data/Games/ClashRoyale";

import {environment} from "../../../../environments/environment";

const {API_SERVER_URL} = environment;

@Injectable({
    providedIn: "root",
})
export class GamesService {

    constructor(private http: HttpClient) {
    }

    getClashRoyaleCards(): Promise<ClashRoyaleArenaGameDataResponse> {
        return this.http.post<ClashRoyaleArenaGameDataResponse>(`${API_SERVER_URL}/data/games/clash_royale/cards`,
            undefined).toPromise();
    }

    getClashRoyaleArenas(): Promise<ClashRoyaleArenaGameDataResponse> {
        return this.http.post<ClashRoyaleArenaGameDataResponse>(`${API_SERVER_URL}/data/games/clash_royale/arenas`,
            undefined).toPromise();
    }
}