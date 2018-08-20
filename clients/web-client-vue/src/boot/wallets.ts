import {boot} from "quasar/wrappers";
import {computed} from "vue";
import {PREFERRED_WALLET_STORAGE_KEY} from "src/Constants";
import {PhantomWallet} from "boot/wallets/phantom";
import {WalletAdapter} from "boot/wallets/types";

export const Wallets = {
    // WALLETS ----------------------------------------------------------------
    wallets: {
        phantom: new PhantomWallet(),
    },

    // COMPUTED ---------------------------------------------------------------
    connectingWallet: computed<WalletAdapter | null>(() => {
        for (let key in Wallets.wallets) {
            let wallet: WalletAdapter = (Wallets.wallets as any)[key];

            if (wallet.isConnecting.value) {
                return wallet;
            }
        }

        return null;
    }),
    connectedWallet: computed<WalletAdapter | null>(() => {
        for (let key in Wallets.wallets) {
            let wallet: WalletAdapter = (Wallets.wallets as any)[key];

            if (wallet.isConnected.value) {
                return wallet;
            }
        }

        return null;
    }),

    // METHODS ----------------------------------------------------------------

    walletList(): WalletAdapter[] {
        let list = [];

        for (let key in Wallets.wallets) {
            let wallet: WalletAdapter = (Wallets.wallets as any)[key];
            list.push(wallet);
        }

        return list;
    },

    findWalletByKey(key: string): WalletAdapter | null {
        return (Wallets.wallets as any)[key] || null;
    },

    loadPreferredWallet(): WalletAdapter | null {
        let key = localStorage.getItem(PREFERRED_WALLET_STORAGE_KEY);

        if (key) {
            return Wallets.findWalletByKey(key);
        }

        return null;
    },

    storePreferredWallet(wallet: WalletAdapter | null) {
        if (wallet != null) {
            localStorage.setItem(PREFERRED_WALLET_STORAGE_KEY, wallet.key);
        } else {
            localStorage.removeItem(PREFERRED_WALLET_STORAGE_KEY);
        }
    },
};

// Try to login into the preferred wallet.
(async () => {
    const preferredWallet = Wallets.loadPreferredWallet();

    if (preferredWallet && preferredWallet.isInstalled) {
        try {
            await preferredWallet.connect();
        } catch (e) {
        }
    }
})();

export default boot(() => {
});