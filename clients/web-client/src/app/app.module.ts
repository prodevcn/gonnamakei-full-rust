import {NgModule} from "@angular/core";
import {BrowserModule} from "@angular/platform-browser";
import {HttpClientModule} from "@angular/common/http";
import {AngularSvgIconModule} from "angular-svg-icon";

import {AppRoutingModule} from "./app-routing.module";
import {AppComponent} from "./app.component";
import {FormsModule} from "@angular/forms";

import {DialogModule} from "./components/dialogs/dialogs";

import {IonicModule} from "@ionic/angular";

@NgModule({
    declarations: [AppComponent],
    imports: [BrowserModule, AppRoutingModule, FormsModule, HttpClientModule, DialogModule,
        AngularSvgIconModule.forRoot(), IonicModule.forRoot()],
    bootstrap: [AppComponent],
})
export class AppModule {
}
