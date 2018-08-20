<template>
          <header class="fixed-top q-py-sm q-px-lg toolbar sticky-position">
            <q-toolbar class="max-viewport-width transparent">
                <a href="" class="social-icons"><img src="~assets/Brand_Logotype.png" alt="Instagram"  title="Instagram" class="social-icons__img"></a>
                <q-space/>

                <q-btn class="text-white" no-caps color="secondary" flat label="Challenges" @click="goToChallenges"/>
                <q-btn class="text-white" no-caps color="secondary" flat label="The project" @click="goToChallenges"/>
                <q-btn class="text-white" :ripple="false"  no-caps color="secondary" flat label="Play2Mint" @click="goToChallenges"/>
                <template v-if="connectedWallet">
                    <div class="q-mr-xs">{{ t("connectedAtText") }}</div>
                    <q-btn flat no-caps class="q-px-sm" @click="openWalletConnector">
                        <q-img :src="connectedWallet.icon" class="q-mr-sm" fit="contain" height="24px" width="24px"/>
                        {{ connectedWallet.name }}(
                        <span class="text-monospace text-grey-6" style="font-size: 0.8em">{{
                                shortPublicKey
                            }}</span>)
                    </q-btn>
                </template>
                <q-btn  no-caps v-else @click="openWalletConnector">
                <img src="~assets/playbutton.png" />
<!--{{
                        t("connectWalletButtonLabel")
                    }}-->
                </q-btn>
            </q-toolbar>
        </header>
        
        <q-dialog v-model="showWalletConnector">
            <q-card class="bg-gmi-dark-light">
                <q-card-section>
                    <wallet-connector/>
                </q-card-section>
            </q-card>
        </q-dialog>
</template>

<script lang="ts">
import {computed, defineComponent, ref} from "vue";
import {useI18n} from "vue-i18n";
import {Wallets} from "boot/wallets";
import WalletConnector from "components/WalletConnector.vue";
import {GMIRouter} from "src/router";
import Header from "src/components/header.vue";

   export default defineComponent({
    components: {
        WalletConnector,
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
                console.log("perro");
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
.q-toolbar{
    max-width: 100vw;
}
.q-btn:before{
    box-shadow: none;
}
.q-btn:hover{
    box-shadow: none;
}
header button{
    padding-left: 2rem;
    padding-right: 2rem;

}
</style>

<i18n lang="json5" locale="en">
{
    connectedAtText: "Connected at",
    connectWalletButtonLabel: "Connect Wallet",
}
</i18n>