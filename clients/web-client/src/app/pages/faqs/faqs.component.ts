import {Component, OnInit} from "@angular/core";
import {Faqs} from "../../constants";

@Component({
    selector: "app-faqs",
    templateUrl: "./faqs.component.html",
    styleUrls: ["./faqs.component.scss"],
})
export class FaqsComponent implements OnInit {

    faqs: typeof Faqs = [];
    opened: any;

    constructor() {
    }

    ngOnInit(): void {
        console.log(Faqs);
        this.faqs = Faqs;
    }

    open(faq: any) {
        if (faq === this.opened) {
            this.opened = null;
        } else {
            this.opened = faq;
        }
    }
}
