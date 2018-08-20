import {TestBed} from "@angular/core/testing";

import {ChallengeDetailGuard} from "./challenge-detail.guard";

describe("ChallengeDetailGuard", () => {
    let guard: ChallengeDetailGuard;

    beforeEach(() => {
        TestBed.configureTestingModule({});
        guard = TestBed.inject(ChallengeDetailGuard);
    });

    it("should be created", () => {
        expect(guard).toBeTruthy();
    });
});
