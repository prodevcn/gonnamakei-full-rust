import {Component, Input, OnInit} from "@angular/core";
import {environment} from "../../../environments/environment";

@Component({
    selector: "app-footer-links",
    templateUrl: "./footer-links.component.html",
    styleUrls: ["./footer-links.component.scss"],
})
export class FooterLinksComponent implements OnInit {

    @Input() isMobile = false;
    socialLinks = environment.socialLinks;

    constructor() {
    }

    ngOnInit(): void {
    }

}
