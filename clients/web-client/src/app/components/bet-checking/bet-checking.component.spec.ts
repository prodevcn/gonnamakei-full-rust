import {ComponentFixture, TestBed} from "@angular/core/testing";

import {BetCheckingComponent} from "./bet-checking.component";

describe("BetCheckingComponent", () => {
    let component: BetCheckingComponent;
    let fixture: ComponentFixture<BetCheckingComponent>;

    beforeEach(async () => {
        await TestBed.configureTestingModule({
            declarations: [BetCheckingComponent],
        })
            .compileComponents();
    });

    beforeEach(() => {
        fixture = TestBed.createComponent(BetCheckingComponent);
        component = fixture.componentInstance;
        fixture.detectChanges();
    });

    it("should create", () => {
        expect(component).toBeTruthy();
    });
});
