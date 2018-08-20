import {TestBed} from "@angular/core/testing";

import {BetSendResponseResolver} from "./bet-send-response.resolver";

describe("BetSendResponseResolver", () => {
    let resolver: BetSendResponseResolver;

    beforeEach(() => {
        TestBed.configureTestingModule({});
        resolver = TestBed.inject(BetSendResponseResolver);
    });

    it("should be created", () => {
        expect(resolver).toBeTruthy();
    });
});
