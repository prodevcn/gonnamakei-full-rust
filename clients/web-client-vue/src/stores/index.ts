import {reactive} from "vue";
import GlobalStore from "src/stores/modules/GlobalStore";
import GameDataStore from "src/stores/modules/GameDataStore";
import ChallengeStore from "src/stores/modules/ChallengeStore";
import ParticipantStore from "src/stores/modules/ParticipantStore";
import AuthStore from "src/stores/modules/AuthStore";

// This is the global storage/cache to share data across the app.
export const Store = reactive({
    global: new GlobalStore(),
    auth: new AuthStore(),
    gameData: new GameDataStore(),
    challenge: new ChallengeStore(),
    participant: new ParticipantStore(),

    // METHODS ----------------------------------------------------------------

    cleanPrivateData() {
        Store.auth.cleanAll();
        Store.participant.cleanAll();
    },
});