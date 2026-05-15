# The AWARE Decalog

Five structural truths AWARE is built on. They are short on purpose — short enough to fit in your head, blunt enough to argue with. Every design decision can be checked against this list. If a decision contradicts one of these, the decision is wrong, not the rule.

If AWARE ever ships something that breaks one of these, this document becomes a lie and the project is over.

---

## 1. App = text

Not metaphorically. **Literally.** The whole app is a file you can read in Notepad.

Plain English describing intent. No binaries. No bundles. No vendor IP locked inside the format. If you can't open it with `cat`, it's not an AWARE app.

## 2. AI = runtime

AI is not a feature inside an app. It is the **thing that executes the text.**

The app is the script; AI is the engine. Without AI, AWARE apps don't run — same way Java apps don't run without a JVM. Swap the AI (Claude, GPT, Llama, what comes next) and AWARE keeps working. The text outlives the model.

## 3. Open source = inherent

Open source is not a license decision; it is a **structural property of the format.**

Because the app is text, sharing it is sending the file. Hiding an AWARE app means literally not sending the file. There is no such thing as a "proprietary AWARE app" — only AWARE apps you choose not to share. The instant you share one, it is open by construction.

## 4. No vendor in the loop

No installer. No proprietary format. No walled garden. No "AWARE Cloud" tier you must pay to unlock real features.

The substrate is **text + a runtime you can swap.** If Anthropic shut down tomorrow, AWARE keeps running on Codex, OpenCode, or whatever comes next. The user owns the text. The user owns the runtime. The vendor disappears.

## 5. AECO is the wedge, not the limit

AWARE starts in AECO because that's where the inertia is heaviest and the leverage is biggest. But the substrate doesn't know what AECO is — it just runs paragraphs.

**The pattern works for anything an AI can do — which is everything.** AECO is the proving ground. The framework is for any vertical that has software and knowledge to compose.

---

These five are the decalog. They are not goals to be balanced against other goals. They are constraints. Anything that breaks one is rejected, regardless of how clever, profitable, or popular the alternative looks.
