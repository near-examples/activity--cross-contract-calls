import { reverseWordOne } from '../../contracts/sentences';
import { Word } from '../../models/word';
import { Context as VMContext, VM } from "near-sdk-as";

// const text = "sample sentence"
// const reversedText = "elpmas ecnetnes"
const text = "sample"
const lang = "en-us"
const reversedText = "elpmas"

describe("CONTRACT: Sentences", () => {
    it("should reverse a sentence", () => {
        reverseWordOne()
        log(VM.outcome())
    });
});
