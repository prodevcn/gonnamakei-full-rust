import {Component, OnInit} from "@angular/core";
import {WaitlistPromptComponent} from "../waitlist-prompt/waitlist-prompt.component";

@Component({
    selector: "app-home-footer",
    templateUrl: "./home-footer.component.html",
    styleUrls: ["./home-footer.component.scss"],
})
export class HomeFooterComponent implements OnInit {

    WaitlistPromptComponent = WaitlistPromptComponent;

    ngOnInit(): void {
    }

}
