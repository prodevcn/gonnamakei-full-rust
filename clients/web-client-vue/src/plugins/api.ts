import {AxiosInstance, AxiosRequestConfig, AxiosResponse, Method} from "axios";
import {APIError} from "../types/api/APITypes";
import {App} from "Vue";
import {LoadingBar} from "quasar";
import {Logger} from "src/plugins/logger";
import {Store} from "src/stores";
import {CLIENT_NETWORK_ERROR_CODE, CLIENT_NOT_LOGGED_IN_ERROR_CODE} from "src/types/api/APIErrorCodes";

export class APIManager {
    vue: App;
    axios: AxiosInstance;
    private _triggerLoadingBar: boolean;

    // CONSTRUCTORS -----------------------------------------------------------

    constructor(axios: AxiosInstance) {
        this.vue = null as any;
        this.axios = axios;
        this._triggerLoadingBar = true;
    }

    // GETTERS & SETTERS ------------------------------------------------------

    get triggerLoadingBar(): boolean {
        return this._triggerLoadingBar;
    }

    set triggerLoadingBar(value: boolean) {
        this._triggerLoadingBar = value;
    }

    // METHODS ----------------------------------------------------------------

    async get<R>(path: string, options: AxiosRequestOptions = {}): Promise<R | APIError> {
        return this.request("GET", path, undefined, options);
    }

    async post<P, R>(path: string, body: P, options: AxiosRequestOptions = {}): Promise<R | APIError> {
        return this.request("POST", path, body, options);
    }

    async request<P, R>(method: Method, path: string, body: P,
                        options: AxiosRequestOptions = {}): Promise<R | APIError> {
        let ajaxBar: LoadingBar = this.vue.config.globalProperties.$q.loadingBar;
        let triggerLoadingBar = options.triggerLoadingBar !== undefined ? options.triggerLoadingBar :
            this._triggerLoadingBar;
        let requestConfig = options.requestConfig || {};

        if (triggerLoadingBar) {
            ajaxBar.start();
        }

        // Set request information.
        requestConfig.validateStatus = null;
        requestConfig.url = path;
        requestConfig.method = method;
        requestConfig.data = body;

        // Prepare auth header.
        switch (options.authentication) {
            case APIAuthentication.Token:
                if (!Store.auth.isLoggedIn) {
                    return {
                        errorCode: CLIENT_NOT_LOGGED_IN_ERROR_CODE,
                    };
                }

                requestConfig.headers = requestConfig.headers || {};
                requestConfig.headers["authorization"] = "GMI " + Store.auth.token;
                break;
        }

        try {
            let response: AxiosResponse<R | APIError> = await this.axios!.request<R>(requestConfig);

            // Here the response can be an error.
            Logger.debug(`Request[${method}] to ${path}`, body, response);

            if (triggerLoadingBar) {
                ajaxBar.stop();
            }

            return response.data;
        } catch (e: any) {
            if (triggerLoadingBar) {
                ajaxBar.stop();
            }

            // Here there is a problem connecting to the server.
            return {
                errorCode: CLIENT_NETWORK_ERROR_CODE,
                message: e.message,
            };
        }
    }
}

export interface AxiosRequestOptions {
    requestConfig?: AxiosRequestConfig,
    triggerLoadingBar?: boolean,
    authentication?: APIAuthentication
}

export enum APIAuthentication {
    Token
}