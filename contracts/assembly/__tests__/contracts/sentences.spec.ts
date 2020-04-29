import { reverseWordOne, reverseWordTwo, reverseWordThree } from '../../contracts/sentences';
import { Word } from '../../models/word';
import { Context as VMContext, VM } from "near-sdk-as";

// const text = "sample sentence"
// const reversedText = "elpmas ecnetnes"
const text = "sample"
const lang = "en-us"
const reversedText = "elpmas"

describe("CONTRACT: Sentences", () => {
    it("should reverse a sentence (one)", () => {
        reverseWordOne()
        log(VM.outcome())
        log(VM.logs())
    });

    it("should reverse a sentence (two)", () => {
        reverseWordTwo()
        log(VM.outcome())
        log(VM.logs())
    });

    xit("should reverse a sentence (three)", () => {
        reverseWordThree()
        log(VM.outcome())
        log(VM.logs())
    });
});
