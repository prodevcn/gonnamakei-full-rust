import {Injectable} from "@angular/core";

@Injectable({
    providedIn: "root",
})
export class DataService {
    routePaths = [] as Array<string>;

    constructor() {
    }

    addRoutePath(path: string) {
        this.routePaths.push(path);
    }
}
