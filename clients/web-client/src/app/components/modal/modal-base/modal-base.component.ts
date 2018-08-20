import {
    Component, ComponentFactoryResolver, Directive, EventEmitter, Input, OnInit, Type, ViewChild, ViewContainerRef,
} from "@angular/core";
import {ModalContainer} from "../modal.interface";

@Directive({
    selector: "[modalBodyContainer]",
})
export class ModalBodyContainer {
    constructor(public viewContainerRef: ViewContainerRef) {
    }
}

@Component({
    selector: "app-modal-base",
    templateUrl: "./modal-base.component.html",
    styleUrls: ["./modal-base.component.scss"],
})
export class ModalBaseComponent implements OnInit, ModalContainer {

    @ViewChild(ModalBodyContainer, {static: true}) modalBodyContainer: ModalBodyContainer;
    @Input("component") component: Type<ModalContainer>;
    @Input("componentProps") componentProps: any;
    @Input("maxWidth") maxWidth: any;
    closeModal = new EventEmitter();

    constructor(private componentFactoryResolver: ComponentFactoryResolver) {

    }

    ngOnInit(): void {
        const componentFactory = this.componentFactoryResolver.resolveComponentFactory(this.component);
        const viewContainerRef = this.modalBodyContainer.viewContainerRef;

        viewContainerRef.clear();

        const componentRef = viewContainerRef.createComponent(componentFactory);
        const instance = componentRef.instance;
        Object.assign(instance, this.componentProps || {});

        if (instance.closeModal) {
            instance.closeModal.subscribe((arg) => this.close(arg));
        }

    }

    close(arg?) {
        this.closeModal.next(arg)
    }

}