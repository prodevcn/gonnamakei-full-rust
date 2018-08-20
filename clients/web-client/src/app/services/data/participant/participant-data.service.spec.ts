import {TestBed} from "@angular/core/testing";

import {ParticipantDataService} from "./participant-data.service";

describe("ParticipantService", () => {
    let service: ParticipantDataService;

    beforeEach(() => {
        TestBed.configureTestingModule({});
        service = TestBed.inject(ParticipantDataService);
    });

    it("should be created", () => {
        expect(service).toBeTruthy();
    });
});
