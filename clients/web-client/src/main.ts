import {enableProdMode} from "@angular/core";
import {platformBrowserDynamic} from "@angular/platform-browser-dynamic";

import {AppModule} from "./app/app.module";
import {environment} from "./environments/environment";

if (environment.production) {
    enableProdMode();
}

platformBrowserDynamic().bootstrapModule(AppModule)
    .catch(err => console.error(err));

const EMAIl_REGEX = /^[\w\.%+-]+@[\w-]+(?:\.[\w-]+)*\.[a-z]+$/;

export function isEmail(email: string): boolean {
    return EMAIl_REGEX.test(email);
}