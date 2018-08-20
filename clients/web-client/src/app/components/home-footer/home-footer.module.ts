import {NgModule} from "@angular/core";
import {CommonModule} from "@angular/common";
import {HomeFooterComponent} from "./home-footer.component";
import {FooterLinksModule} from "../footer-links/footer-links.module";
import {ModalModule} from "../modal/modal.module";
import {WaitlistPromptModule} from "../waitlist-prompt/waitlist-prompt.module";

@NgModule({
    declarations: [HomeFooterComponent],
    imports: [CommonModule, ModalModule, WaitlistPromptModule, FooterLinksModule],
    exports: [HomeFooterComponent],
})
export class HomeFooterModule {
}
