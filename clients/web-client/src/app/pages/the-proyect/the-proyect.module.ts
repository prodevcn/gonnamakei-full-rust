import {NgModule} from "@angular/core";
import {RouterModule, Routes} from "@angular/router";
import {CommonModule} from "@angular/common";
import {HeaderModule} from "../../components/header/header.module";
import {FooterModule} from "../../components/footer/footer.module";
import {TheProyectComponent} from "./the-proyect.component";

const routes: Routes = [{
    path: "",
    component: TheProyectComponent,
}];

@NgModule({
    declarations: [TheProyectComponent],
    imports: [CommonModule, HeaderModule, FooterModule, RouterModule.forChild(routes)],
    exports: [TheProyectComponent],
})
export class TheProyectModule {
}
