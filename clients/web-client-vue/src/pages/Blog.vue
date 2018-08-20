<template>
    <main>
        <h6>Blog</h6>
    </main>
</template>

<script lang="ts">
import {defineComponent} from "vue";
import {useI18n} from "vue-i18n";

export default defineComponent({
    setup() {
        let i18n = useI18n({useScope: "local"});

        return {
            // DATA -----------------------------------------------------------

            // COMPUTED -------------------------------------------------------

            // METHODS --------------------------------------------------------
            t: i18n.t,
        };
    },
});
</script>

<style scoped lang="scss"></style>

<i18n lang="json" locale="en">
{
}
</i18n>