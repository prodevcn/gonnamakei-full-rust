import {GameImageUrlPipe} from "./game-image-url.pipe";

describe("GameImageUrlPipe", () => {
    it("create an instance", () => {
        const pipe = new GameImageUrlPipe();
        expect(pipe).toBeTruthy();
    });
});
