<template>
    <div class="full-width full-height scroll-y">
        <Header/>
        <div class="max-viewport-width">
            <router-view/>
        </div>
        <Footer></Footer>
        <q-dialog v-model="showWalletConnector">
            <q-card class="bg-gmi-dark-light">
                <q-card-section>
                    <wallet-connector/>
                </q-card-section>
            </q-card>
        </q-dialog>
    </div>
</template>

<script lang="ts">
import {computed, defineComponent, ref} from "vue";
import {useI18n} from "vue-i18n";
import {Wallets} from "boot/wallets";
import WalletConnector from "components/WalletConnector.vue";
import {GMIRouter} from "src/router";
import Footer from "src/components/footer.vue";
import Header from "src/components/header.vue";

export default defineComponent({
    components: {
        WalletConnector,
        Footer,
        Header
    },
    setup() {
        let i18n = useI18n({useScope: "local"});

        // DATA ---------------------------------------------------------------
        const showWalletConnector = ref(false);

        // COMPUTED -----------------------------------------------------------
        const shortPublicKey = computed<string>(() => {
            let publicKey = Wallets.connectedWallet.value?.publicKey;

            if (!publicKey) {
                return "";
            }

            let publicKeyBase58 = publicKey.toBase58();
            return publicKeyBase58.substr(0, 4) + "..." + publicKeyBase58.substr(publicKeyBase58.length - 4, 4);
        });

        // WATCHES ------------------------------------------------------------

        // METHODS ------------------------------------------------------------

        return {
            // DATA -----------------------------------------------------------
            showWalletConnector,

            // COMPUTED -------------------------------------------------------
            connectedWallet: Wallets.connectedWallet,
            shortPublicKey,

            // METHODS --------------------------------------------------------
            t: i18n.t,

            openWalletConnector() {
                showWalletConnector.value = true;
            },

            goToChallenges() {
                if (GMIRouter.currentRoute.value.name === "Challenges") {
                    return;
                }

                GMIRouter.push({
                    name: "Challenges",
                });
            },
        };
    },
});
</script>

<style scoped lang="scss">
.toolbar {
  z-index: 500;
  background-color: transparentize($gmi-dark, 0.1);
}
</style>

<i18n lang="json5" locale="en">
{
    connectedAtText: "Connected at",
    connectWalletButtonLabel: "Connect Wallet",
}
</i18n>
