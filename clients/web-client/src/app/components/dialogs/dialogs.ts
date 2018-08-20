import {NgModule} from "@angular/core";
import {CommonModule} from "@angular/common";
import {MessageDialogComponent} from "./message-dialog/message-dialog.component";

@NgModule({
    declarations: [MessageDialogComponent],
    imports: [CommonModule],
    exports: [MessageDialogComponent],
})
export class DialogModule {
}
