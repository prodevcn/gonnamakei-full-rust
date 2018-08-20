import {NgModule} from "@angular/core";
import {RouterModule, Routes} from "@angular/router";
import {CommonModule} from "@angular/common";
import {HeaderModule} from "../../components/header/header.module";
import {HomeFooterModule} from "../../components/home-footer/home-footer.module";
import {StoriesComponent} from "./stories.component";

const routes: Routes = [{
    path: "",
    component: StoriesComponent,
}];

@NgModule({
    declarations: [StoriesComponent],
    imports: [CommonModule, HeaderModule, HomeFooterModule, RouterModule.forChild(routes)],
    exports: [StoriesComponent],
})
export class StoriesModule {
}
