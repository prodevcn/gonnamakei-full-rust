import {NgModule} from "@angular/core";
import {CommonModule} from "@angular/common";
import {FooterComponent} from "./footer.component";
import {FooterLinksModule} from "../footer-links/footer-links.module";

@NgModule({
    declarations: [FooterComponent],
    imports: [CommonModule, FooterLinksModule],
    exports: [FooterComponent],
})
export class FooterModule {
}
