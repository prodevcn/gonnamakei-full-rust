import {APIEndpoints} from "src/types/api/APIEndpoints";
import {isAPIError} from "src/types/api/APITypes";
import {
    ClashRoyaleArenaGameDataResponse, ClashRoyaleCardGameDataResponse,
} from "src/types/api/models/responses/data/games/ClashRoyale";

let clashRoyaleCards: ClashRoyaleCardGameDataResponse[] | null = null;
let clashRoyaleArenas: ClashRoyaleArenaGameDataResponse[] | null = null;

export default class GameDataStore {
    // GETTERS ----------------------------------------------------------------

    // METHODS ----------------------------------------------------------------

    async clashRoyaleCards() {
        if (clashRoyaleCards != null) {
            return clashRoyaleCards;
        }

        let response = await APIEndpoints.data.games.clashRoyale.cards({
            triggerLoadingBar: false,
        });

        if (isAPIError(response)) {
            throw response;
        }

        // Freeze objects to not modify them.
        Object.freeze(response);

        for (let i of response) {
            Object.freeze(i);
        }

        clashRoyaleCards = response;
        return response;
    }

    async clashRoyaleArenas() {
        if (clashRoyaleArenas != null) {
            return clashRoyaleArenas;
        }

        let response = await APIEndpoints.data.games.clashRoyale.arenas({
            triggerLoadingBar: false,
        });

        if (isAPIError(response)) {
            throw response;
        }

        // Freeze objects to not modify them.
        Object.freeze(response);

        for (let i of response) {
            Object.freeze(i);
        }

        clashRoyaleArenas = response;
        return response;
    }
}
