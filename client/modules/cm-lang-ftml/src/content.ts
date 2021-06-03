import spellcheckerWASMRelativeURL from "spellchecker-wasm/lib/spellchecker-wasm.wasm?url"
import { Pref } from "wj-state"
import { decode, transfer, WorkerModule } from "worker-module"
import type { ContentModuleInterface } from "./worker/content.worker"

const spellcheckerWASMURL = new URL(
  spellcheckerWASMRelativeURL,
  import.meta.url
).toString()

async function url(imp: Promise<any>) {
  return new URL((await imp).default, import.meta.url).toString()
}

type DictionaryImporter = () => Promise<{ dict: string; bigram?: string }>

// prettier-ignore
const dicts: Record<string, DictionaryImporter> = {
  "en": async () => ({
    dict: await url(import("../vendor/dicts/en-merged.txt?url"))
  })
}

async function importWorker() {
  return (await import("./worker/content.worker?bundled-worker")).default
}

export class ContentWorker extends WorkerModule<ContentModuleInterface> {
  constructor() {
    super("ftml-lang-content-worker", importWorker, {
      persist: true,
      init: async () => {
        const { dict, bigram } = await dicts.en()
        await this.invoke("setSpellchecker", spellcheckerWASMURL, dict, bigram)
        const localDictionary = Pref.get<string[]>("cm-lang-ftml-user-dictionary", [])
        if (localDictionary.length) {
          await this.invoke("appendToDictionary", localDictionary)
        }
      }
    })
  }

  async extract(str: string) {
    return decode(await this.invoke("extract", transfer(str)))
  }

  async stats(str: string) {
    return await this.invoke("stats", transfer(str))
  }

  async setSpellchecker(locale: string) {
    if (!dicts.hasOwnProperty(locale)) throw new Error("Invalid locale specified!")
    const { dict, bigram } = await dicts[locale]()
    await this.invoke("setSpellchecker", spellcheckerWASMURL, dict, bigram)
  }

  async spellcheck(word: string) {
    return await this.invoke("spellcheck", transfer(word))
  }

  async spellcheckWords(str: string) {
    return await this.invoke("spellcheckWords", transfer(str))
  }

  async appendToDictionary(input: string | string[], frequency = 1000) {
    await this.invoke("appendToDictionary", input, frequency)
  }

  async saveToDictionary(word: string) {
    const localDictionary = Pref.get<string[]>("cm-lang-ftml-user-dictionary", [])
    // add our word but do a dedupe pass to catch edge cases
    const deduped = [...new Set([...localDictionary, word])]
    Pref.set("cm-lang-ftml-user-dictionary", deduped)
    // we already appened to dictionary when the spellchecker was started
    // so we just need to add the word
    await this.invoke("appendToDictionary", [word])
  }
}

export default new ContentWorker()
