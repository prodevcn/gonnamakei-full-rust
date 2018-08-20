import {ComponentFixture, TestBed} from "@angular/core/testing";

import {WaitlistPromptComponent} from "./waitlist-prompt.component";

describe("WaitlistPromptComponent", () => {
    let component: WaitlistPromptComponent;
    let fixture: ComponentFixture<WaitlistPromptComponent>;

    beforeEach(async () => {
        await TestBed.configureTestingModule({
            declarations: [WaitlistPromptComponent],
        })
            .compileComponents();
    });

    beforeEach(() => {
        fixture = TestBed.createComponent(WaitlistPromptComponent);
        component = fixture.componentInstance;
        fixture.detectChanges();
    });

    it("should create", () => {
        expect(component).toBeTruthy();
    });
});
