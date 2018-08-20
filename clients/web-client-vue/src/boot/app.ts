import {boot} from "quasar/wrappers";
import {App} from "vue";
import {QVueGlobals} from "quasar/dist/types/globals";
import {Logger} from "src/plugins/logger";
import {Solana} from "boot/solana";
import {i18n} from "boot/i18n";
import {Wallets} from "boot/wallets";
import {GMIRouter} from "src/router";

interface GMIAppType {
    app: App,
    quasar: QVueGlobals,
    solana: typeof Solana,
    i18n: typeof i18n
    wallets: typeof Wallets
    router: typeof GMIRouter
}

export const GMIApp: GMIAppType = {
    app: null as any,
    quasar: null as any,
    solana: null as any,
    i18n: null as any,
    wallets: null as any,
    router: null as any,
};

export default boot(({app}) => {
    GMIApp.app = app;
    GMIApp.quasar = app.config.globalProperties.$q;
    GMIApp.solana = Solana;
    GMIApp.i18n = i18n;
    GMIApp.wallets = Wallets;
    GMIApp.router = GMIRouter;
});

Logger.inDevelopment(() => {
    (window as any).GMIApp = GMIApp;
    Logger.debug("[APP] Set at window.GMIApp", GMIApp);
});