import {TestBed} from "@angular/core/testing";

import {GamePlayGuard} from "./game-play.guard";

describe("GamePlayGuard", () => {
    let guard: GamePlayGuard;

    beforeEach(() => {
        TestBed.configureTestingModule({});
        guard = TestBed.inject(GamePlayGuard);
    });

    it("should be created", () => {
        expect(guard).toBeTruthy();
    });
});
