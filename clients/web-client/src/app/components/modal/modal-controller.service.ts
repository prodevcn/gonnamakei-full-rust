import {ApplicationRef, ComponentFactoryResolver, Injectable} from "@angular/core";
import {Observable} from "rxjs";
import {first} from "rxjs/operators";

import {ModalBaseComponent} from "./modal-base/modal-base.component";

@Injectable({
    providedIn: "root",
})
export class ModalController {

    constructor(private applicationRef: ApplicationRef, private componentFactoryResolver: ComponentFactoryResolver) {
    }

    open(_args: { component: any, modalProps?: any, componentProps?: any }): { closeModal: Observable<any>, close: Function } | null {

        const componentFactory = this.componentFactoryResolver.resolveComponentFactory(ModalBaseComponent);
        const viewContainerRef = this.applicationRef.components[0].instance.viewContainerRef;

        const componentRef = viewContainerRef.createComponent(componentFactory);
        const instance = componentRef.instance as ModalBaseComponent;

        const {
            modalProps,
            ...otherArgs
        } = _args;

        Object.assign(instance, {...otherArgs, ...modalProps});
        instance.closeModal.pipe(first()).subscribe(() => {
            const index = viewContainerRef.indexOf(componentRef.hostView);
            if (index >= 0) {
                viewContainerRef.remove(index);
            }
        });
        return {
            closeModal: instance.closeModal,
            close: (arg) => {
                try {
                    instance.close(arg);
                } catch (error) {
                    console.log(error);
                }
            }
        }
    }
}