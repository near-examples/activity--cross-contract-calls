import { Word } from '../../models/word';
import { Context as VMContext, VM } from "near-sdk-as";

let text: string
let lang: string
let word: Word

describe("MODEL: Word", () => {

    beforeEach(() => {
        text = "sample"
        lang = "en-us"
        word = new Word(text, lang)
    })

    it("should allow instantiation", () => {
        expect(word instanceof Word).toBeTruthy()
    });

    it("should expose the word as text", () => {
        expect(word.text).toStrictEqual(text)
        expect(word.lang).toStrictEqual(lang)
    })
});

