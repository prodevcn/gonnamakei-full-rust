import {SolAmountPipe} from "./sol-amount.pipe";

describe("SolAmountPipe", () => {
    it("create an instance", () => {
        const pipe = new SolAmountPipe();
        expect(pipe).toBeTruthy();
    });
});
