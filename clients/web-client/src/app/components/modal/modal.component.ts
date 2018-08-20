import {Component, EventEmitter, Input, OnInit, Type} from "@angular/core";
import {ModalContainer} from "./modal.interface";

import {ModalController} from "./modal-controller.service";

@Component({
    selector: "app-modal",
    template: "",
})
export class ModalComponent implements OnInit, ModalContainer {

    @Input("component") component: Type<ModalContainer>;
    @Input("componentProps") componentProps: any;
    @Input("modalProps") modalProps: any;

    closeModal = new EventEmitter();

    _close: Function = null;

    constructor(private modalController: ModalController) {

    }

    ngOnInit(): void {
    }

    open() {
        const {
            closeModal,
            close,
        } = this.modalController.open({
            component: this.component,
            componentProps: this.componentProps,
            modalProps: this.modalProps,
        });
        closeModal.subscribe((arg) => this.closeModal.next(arg));
        this._close = close;
    }

    close() {
        if (this._close) {
            this._close()
        }
    }

}