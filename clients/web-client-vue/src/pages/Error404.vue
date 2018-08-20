<template>
    <div class="fullscreen text-white text-center q-pa-md flex flex-center">
        <div>
            <div style="font-size: 30vh">
                404
            </div>

            <div class="text-h2" style="opacity:.4">
                {{ t("message") }}
            </div>

            <q-btn class="q-mt-xl" color="primary" :label="t('backButtonLabel')" no-caps to="/" unelevated/>
        </div>
    </div>
</template>

<script lang="ts">
import {defineComponent} from "vue";
import {useI18n} from "vue-i18n";

export default defineComponent({
    setup() {
        let i18n = useI18n({useScope: "local"});

        // DATA ---------------------------------------------------------------

        // COMPUTED -----------------------------------------------------------

        return {
            // DATA -----------------------------------------------------------

            // COMPUTED -------------------------------------------------------

            // METHODS --------------------------------------------------------
            t: i18n.t,
        };
    },
});
</script>

<i18n lang="json" locale="en">
{
    "message": "Oops. Nothing here...",
    "backButtonLabel": "Go Home"
}
</i18n>
