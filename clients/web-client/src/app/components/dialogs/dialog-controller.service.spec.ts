import {TestBed} from "@angular/core/testing";

import {DialogControllerService} from "./dialog-controller.service";

describe("DialogControllerService", () => {
    let service: DialogControllerService;

    beforeEach(() => {
        TestBed.configureTestingModule({});
        service = TestBed.inject(DialogControllerService);
    });

    it("should be created", () => {
        expect(service).toBeTruthy();
    });
});
