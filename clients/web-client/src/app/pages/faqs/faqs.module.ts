import {NgModule} from "@angular/core";
import {RouterModule, Routes} from "@angular/router";
import {CommonModule} from "@angular/common";
import {AngularSvgIconModule} from "angular-svg-icon";

import {FooterModule} from "../../components/footer/footer.module";
import {HeaderModule} from "../../components/header/header.module";
import {FaqsComponent} from "./faqs.component";

const routes: Routes = [{
    path: "",
    component: FaqsComponent,
}];

@NgModule({
    declarations: [FaqsComponent],
    imports: [CommonModule, FooterModule, HeaderModule, AngularSvgIconModule, RouterModule.forChild(routes)],
})
export class FaqsModule {
}
