import {NgModule} from "@angular/core";
import {RouterModule, Routes} from "@angular/router";
import {CommonModule} from "@angular/common";
import {AboutUsComponent} from "./about-us.component";
import {AngularSvgIconModule} from "angular-svg-icon";

import {FooterModule} from "../../components/footer/footer.module";
import {HeaderModule} from "../../components/header/header.module";

const routes: Routes = [{
    path: "",
    component: AboutUsComponent,
}];

@NgModule({
    declarations: [AboutUsComponent],
    imports: [CommonModule, FooterModule, HeaderModule, AngularSvgIconModule, RouterModule.forChild(routes)],
})
export class AboutUsModule {
}
