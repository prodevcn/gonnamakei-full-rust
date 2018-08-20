import {TestBed} from "@angular/core/testing";

import {ChallengeBetResolver} from "./challenge-bet.resolver";

describe("ChallengeBetResolver", () => {
    let resolver: ChallengeBetResolver;

    beforeEach(() => {
        TestBed.configureTestingModule({});
        resolver = TestBed.inject(ChallengeBetResolver);
    });

    it("should be created", () => {
        expect(resolver).toBeTruthy();
    });
});
