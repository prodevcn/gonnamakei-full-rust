export const Logger = {
    inDevelopment(action: Function) {
        if (process.env.NODE_ENV === "development") {
            action();
        }
    },
    trace(...data: any[]) {
        if (process.env.NODE_ENV === "development") {
            console.trace(...data);
        }
    },

    debug(...data: any[]) {
        if (process.env.NODE_ENV === "development") {
            console.debug(...data);
        }
    },

    infoInDevelopment(...data: any[]) {
        if (process.env.NODE_ENV === "development") {
            console.info(...data);
        }
    },

    info(...data: any[]) {
        console.info(...data);
    },

    warnInDevelopment(...data: any[]) {
        if (process.env.NODE_ENV === "development") {
            console.warn(...data);
        }
    },

    warn(...data: any[]) {
        console.warn(...data);
    },

    errorInDevelopment(...data: any[]) {
        if (process.env.NODE_ENV === "development") {
            console.error(...data);
        }
    },

    error(...data: any[]) {
        console.error(...data);
    },
};