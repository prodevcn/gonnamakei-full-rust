import {ComponentFixture, TestBed} from "@angular/core/testing";

import {Challenge2Component} from "./challenge2.component";

describe("Challenge2Component", () => {
    let component: Challenge2Component;
    let fixture: ComponentFixture<Challenge2Component>;

    beforeEach(async () => {
        await TestBed.configureTestingModule({
            declarations: [Challenge2Component],
        })
            .compileComponents();
    });

    beforeEach(() => {
        fixture = TestBed.createComponent(Challenge2Component);
        component = fixture.componentInstance;
        fixture.detectChanges();
    });

    it("should create", () => {
        expect(component).toBeTruthy();
    });
});
