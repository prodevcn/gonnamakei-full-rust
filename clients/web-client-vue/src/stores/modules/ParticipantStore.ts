import {ParticipantAPIDocument} from "src/types/database/Participant";
import {APIEndpoints} from "src/types/api/APIEndpoints";
import {isAPIError} from "src/types/api/APITypes";
import {Wallets} from "boot/wallets";
import {Store} from "src/stores";
import {ParticipantActiveBetGetResponse} from "src/types/api/models/responses/Participant";

export default class ParticipantStore {
    data: ParticipantAPIDocument | null = null;
    activeBets: ParticipantActiveBetGetResponse[] | null = null;

    // GETTERS ----------------------------------------------------------------

    // METHODS ----------------------------------------------------------------

    async loadData(force: boolean = false): Promise<boolean> {
        let wallet = Wallets.connectedWallet.value;
        let authToken = Store.auth.token;
        if (!wallet || !authToken) {
            return false;
        }

        if (!force && this.data) {
            return true;
        }

        let response = await APIEndpoints.participant.get({
            returnFields: true,
            returnActiveBets: true,
        });

        if (isAPIError(response)) {
            console.error("Cannot get the participant info", response);
            return false;
        }

        this.data = response.participant || null;
        this.activeBets = response.activeBets || null;

        return true;
    }

    cleanAll() {
        this.data = null;
        this.activeBets = null;
    }
}
