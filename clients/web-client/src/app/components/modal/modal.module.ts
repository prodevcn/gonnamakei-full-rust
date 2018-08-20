import {NgModule} from "@angular/core";
import {CommonModule} from "@angular/common";
import {ModalComponent} from "./modal.component";
import {ModalBaseComponent, ModalBodyContainer} from "./modal-base/modal-base.component";

@NgModule({
    declarations: [ModalComponent, ModalBodyContainer, ModalBaseComponent],
    imports: [CommonModule],
    exports: [ModalComponent],
})
export class ModalModule {
}
