import {ComponentFixture, TestBed} from "@angular/core/testing";

import {TheProyectComponent} from "./the-proyect.component";

describe("TheProyectComponent", () => {
    let component: TheProyectComponent;
    let fixture: ComponentFixture<TheProyectComponent>;

    beforeEach(async () => {
        await TestBed.configureTestingModule({
            declarations: [TheProyectComponent],
        })
            .compileComponents();
    });

    beforeEach(() => {
        fixture = TestBed.createComponent(TheProyectComponent);
        component = fixture.componentInstance;
        fixture.detectChanges();
    });

    it("should create", () => {
        expect(component).toBeTruthy();
    });
});
