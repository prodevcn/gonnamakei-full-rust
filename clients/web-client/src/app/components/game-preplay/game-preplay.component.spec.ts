import {ComponentFixture, TestBed} from "@angular/core/testing";

import {GamePreplayComponent} from "./game-preplay.component";

describe("GamePreplayComponent", () => {
    let component: GamePreplayComponent;
    let fixture: ComponentFixture<GamePreplayComponent>;

    beforeEach(async () => {
        await TestBed.configureTestingModule({
            declarations: [GamePreplayComponent],
        })
            .compileComponents();
    });

    beforeEach(() => {
        fixture = TestBed.createComponent(GamePreplayComponent);
        component = fixture.componentInstance;
        fixture.detectChanges();
    });

    it("should create", () => {
        expect(component).toBeTruthy();
    });
});
