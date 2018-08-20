import {TestBed} from "@angular/core/testing";

import {GMIWallets} from "./wallet.service";

describe("WalletService", () => {
    let service: GMIWallets;

    beforeEach(() => {
        TestBed.configureTestingModule({});
        service = TestBed.inject(GMIWallets);
    });

    it("should be created", () => {
        expect(service).toBeTruthy();
    });
});
