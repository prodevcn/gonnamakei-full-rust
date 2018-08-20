import {environment as base} from "./environment.base";

export const environment = {
    ...base,
    production: true,
    API_SERVER_URL: "https://api.gonnamakeit.app",
    SOLANA_CLUSTER: "https://api.devnet.solana.com",
};
