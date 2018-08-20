const API_TOKEN_KEY = "T";

export default class AuthStore {
    private _token: string | null = null;

    // CONSTRUCTORS -----------------------------------------------------------

    constructor() {
        this._token = localStorage.getItem(API_TOKEN_KEY) || null;
    }

    // GETTERS ----------------------------------------------------------------

    get token(): string | null {
        return this._token;
    }

    get isLoggedIn() {
        return this.token !== null;
    }

    // SETTERS ----------------------------------------------------------------

    set token(value: string | null) {
        if (this._token === value) {
            return;
        }

        this._token = value;

        if (value == null) {
            localStorage.removeItem(API_TOKEN_KEY);
        } else {
            localStorage.setItem(API_TOKEN_KEY, value);
        }
    }

    cleanAll() {
        this.token = null;
    }
}