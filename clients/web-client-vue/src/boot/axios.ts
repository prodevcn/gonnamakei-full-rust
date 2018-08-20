import {boot} from "quasar/wrappers";
import axios, {AxiosInstance} from "axios";
import {APIManager} from "src/plugins/api";

const DEFAULT_API_SERVER_URL = process.env.API_SERVER_URL!;

declare module "@vue/runtime-core" {
    interface ComponentCustomProperties {
        $axios: AxiosInstance;
        $api: APIManager;
    }
}

export const API = new APIManager(axios.create({
    baseURL: DEFAULT_API_SERVER_URL,
    headers: {
        "Content-type": "application/json",
    },
}));

export default boot(({app}) => {
    // Configure api.
    API.vue = app;

    // for use inside Vue files (Options API) through this.$axios and this.$api

    app.config.globalProperties.$axios = axios;
    // ^ ^ ^ this will allow you to use this.$axios (for Vue Options API form)
    //       so you won't necessarily have to import axios in each vue file

    app.config.globalProperties.$api = API;
    // ^ ^ ^ this will allow you to use this.$api (for Vue Options API form)
    //       so you can easily perform requests against your app's API
});