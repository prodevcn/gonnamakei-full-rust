import {TestBed} from "@angular/core/testing";

import {ChallengeResolver} from "./challenge.resolver";

describe("ChallengeResolver", () => {
    let resolver: ChallengeResolver;

    beforeEach(() => {
        TestBed.configureTestingModule({});
        resolver = TestBed.inject(ChallengeResolver);
    });

    it("should be created", () => {
        expect(resolver).toBeTruthy();
    });
});
