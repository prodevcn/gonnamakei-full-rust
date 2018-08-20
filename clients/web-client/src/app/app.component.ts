import {Component, OnInit, ViewContainerRef} from "@angular/core";
import {NavigationEnd, Router} from "@angular/router";
import {DataService} from "./services/data";

declare global {
    interface Window {
        gtag: Function;
    }
}

@Component({
    selector: "app-root",
    templateUrl: "./app.component.html",
    styleUrls: ["./app.component.scss"],
})
export class AppComponent implements OnInit {
    title = "gonnaMakeIt";

    constructor(private viewContainerRef: ViewContainerRef, private router: Router, private data: DataService) {
        // To fire google analytics events when page changes.
        this.router.events.subscribe(event => {
            if (event instanceof NavigationEnd) {
                window.gtag("set", "page_path", event.urlAfterRedirects);
                window.gtag("event", "page_view");
            }
        });
    }

    ngOnInit() {
        this.router.events.subscribe(event => {
            if (event instanceof NavigationEnd) {
                this.data.addRoutePath(event.url);
                document.documentElement.scrollTo(0, 0);
                document.body.scrollTo(0, 0);
            }
        });
    }
}
