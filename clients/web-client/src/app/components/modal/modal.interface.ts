import {EventEmitter} from "@angular/core";

export interface ModalContainer {
    closeModal?: EventEmitter<any>;
}