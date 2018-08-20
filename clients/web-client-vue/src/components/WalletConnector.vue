<template>
    <q-list>
        <template v-if="connectedWallet">
            <h4 class="text-center text-bold q-mb-sm">{{ t("walletConnectedTitle") }}</h4>
            <q-item>
                <q-item-section avatar>
                    <q-img :src="connectedWallet.icon" fit="contain" height="32px" width="32px"/>
                </q-item-section>
                <q-item-section style="min-width: 130px">
                    <q-item-label>{{ connectedWallet.name }}</q-item-label>
                </q-item-section>

                <q-item-section side>
                    <q-btn class="q-px-sm"
                           color="negative"
                           size="sm"
                           no-caps
                           dense
                           @click="disconnectWallet(connectedWallet)">
                        {{ t('disconnectButtonLabel') }}
                    </q-btn>
                </q-item-section>
            </q-item>
            <q-item>
                <q-item-section>
                    <q-item-label>Address</q-item-label>
                </q-item-section>

                <q-item-section side>
                    <q-item-label class="text-monospace text-grey-6" style="font-size: 0.8em">{{
                            publicKey
                        }}
                    </q-item-label>
                </q-item-section>
            </q-item>
            <q-item>
                <q-item-section>
                    <q-item-label>{{ t("preferredWalletOptionLabel") }}</q-item-label>
                    <q-item-label caption>{{ t("preferredWalletOptionExplanation") }}</q-item-label>
                </q-item-section>

                <q-item-section side>
                    <q-toggle dense
                              color="secondary"
                              @update:model-value="togglePreferredWallet"
                              v-model="preferredWallet"/>
                </q-item-section>
            </q-item>
        </template>
        <template v-else>
            <h4 class="text-center text-bold q-mb-sm">{{ t("connectWalletTitle") }}</h4>
            <q-item-label header>{{ t("installedWalletsHeader") }}</q-item-label>
            <q-item v-for="wallet in installedWalletList">
                <q-item-section avatar>
                    <q-img :src="wallet.icon" fit="contain" height="32px" width="32px"/>
                </q-item-section>
                <q-item-section style="min-width: 130px">
                    <q-item-label>{{ wallet.name }}</q-item-label>
                </q-item-section>

                <q-item-section side>
                    <q-btn class="q-px-sm"
                           color="positive"
                           size="sm"
                           no-caps
                           dense
                           @click="connectWallet(wallet)"
                           :loading="connectingWallet === wallet"
                           :disable="!!connectingWallet">
                        {{ t('connectButtonLabel') }}
                    </q-btn>
                </q-item-section>
            </q-item>

            <q-item-label header>{{ t("availableWalletsHeader") }}</q-item-label>
            <q-item v-for="wallet in availableWalletList">
                <q-item-section avatar>
                    <q-img :src="wallet.icon" fit="contain" height="32px" width="32px"/>
                </q-item-section>
                <q-item-section style="min-width: 130px">
                    <q-item-label>{{ wallet.name }}</q-item-label>
                </q-item-section>

                <q-item-section side>
                    <q-btn class="q-ml-xs"
                           color="secondary"
                           size="sm"
                           no-caps
                           dense
                           icon="r_launch"
                           @click="openUrl(wallet.url)"></q-btn>
                </q-item-section>
            </q-item>
        </template>
    </q-list>
</template>

<script lang="ts">
import {computed, defineComponent, ref} from "vue";
import {useI18n} from "vue-i18n";
import {Store} from "src/stores";
import {Wallets} from "boot/wallets";
import {WalletAdapter} from "boot/wallets/types";

export default defineComponent({
    setup() {
        let i18n = useI18n({useScope: "local"});

        // DATA ---------------------------------------------------------------
        let connectingWallet = Wallets.connectingWallet;
        let connectedWallet = Wallets.connectedWallet;
        let preferredWallet = ref(Wallets.loadPreferredWallet() === connectedWallet.value);
        let walletList = Wallets.walletList();
        let installedWalletList = walletList.filter((v) => v.isInstalled);
        let availableWalletList = walletList.filter((v) => !v.isInstalled);

        // COMPUTED -----------------------------------------------------------
        const publicKey = computed<string>(() => {
            let publicKey = Wallets.connectedWallet.value?.publicKey;

            if (!publicKey) {
                return "";
            }

            return publicKey.toBase58();
        });

        return {
            // DATA -----------------------------------------------------------
            connectingWallet,
            connectedWallet,
            preferredWallet,

            // COMPUTED -------------------------------------------------------
            installedWalletList,
            availableWalletList,
            publicKey,

            // METHODS --------------------------------------------------------
            t: i18n.t,

            async connectWallet(wallet: WalletAdapter) {
                try {
                    await wallet.connect();
                    Wallets.storePreferredWallet(wallet);
                    await Store.participant.loadData();
                } catch (e) {
                }
            },

            async disconnectWallet(wallet: WalletAdapter) {
                try {
                    await wallet.disconnect();
                    Wallets.storePreferredWallet(null);
                } catch (e) {
                }
            },

            openUrl(url: string) {
                window.open(url, "_blank");
            },

            togglePreferredWallet() {
                if (preferredWallet.value) {
                    Wallets.storePreferredWallet(null);
                    preferredWallet.value = false;
                } else {
                    Wallets.storePreferredWallet(connectedWallet.value);
                    preferredWallet.value = true;
                }
            },
        };
    },
});
</script>

<i18n lang="json" locale="en">
{
    "walletConnectedTitle": "Wallet Connected",
    "connectWalletTitle": "Connect Wallet",
    "installedWalletsHeader": "Installed Wallets",
    "availableWalletsHeader": "Available Wallets",
    "connectButtonLabel": "Connect",
    "disconnectButtonLabel": "Disconnect",
    "installButtonLabel": "Install",
    "preferredWalletOptionLabel": "Save as preferred wallet?",
    "preferredWalletOptionExplanation": "Allows to auto connect to the wallet"
}
</i18n>