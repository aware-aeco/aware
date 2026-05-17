# The AWARE Decalog

Nine structural truths AWARE is built on. They are short on purpose — short enough to fit in your head, blunt enough to argue with. Every design decision can be checked against this list. If a decision contradicts one of these, the decision is wrong, not the rule.

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

## 6. Ship agents, not apps

The unit of distribution is the **agent**, not the app. Apps are scratchpads — useful for figuring out what the composition should be. The artifact other people install, fork, and compose into their own pipelines is an agent.

If you find yourself shipping an app, you stopped one step too early. Mark it `exposes-as-agent: true` and ship it as an agent instead.

## 7. Build app that is an agent

The endpoint of composition is **an app that is an agent.** Every app should be authored as if its destiny is to become one node in someone else's bigger graph.

This is what makes AWARE compositional all the way down: agents compose into apps, apps expose as agents, those agents compose into bigger apps. There is no terminal layer where composition stops and "the product" begins.

## 8. Closed apps that wrap an LLM are training their replacements

Closed-source apps that wrap an LLM and gate it behind a UI are **training their replacements.** Every prompt the user types into the wrapper is a prompt they could have typed into the model directly. The UI is the moat that disappears the moment users realize the model underneath is the product.

AWARE apps don't have this problem by construction: the composition is plain text, the model is visible, the wrapper is the file. There is nothing hidden to defend, so there is no replacement to train.

## 9. AI composes the plan; deterministic code is the plan

The 2026-05-17 persona audit surfaced this as the single most important boundary an AECO substrate must hold. The structural engineer named it as a walk-away condition; the BIM manager named it as the dealbreaker that prevents any production adoption:

> "If AWARE's pitch is 'AI is the runtime' (decalog #2), and that means an LLM is in the validator path for *'is this drawing's tolerance within 5%'*, then I will not adopt it under any circumstance. PE seals and steel deliveries do not survive hallucinations. For any AECO write-back, the runtime must be deterministic, traceable, and testable. **The LLM can compose the *plan*. It cannot *be* the plan.**"

Truth #9 is that distinction.

**Composing the plan** — an LLM authors the `.flo`, picks the agents, wires the topology. Non-deterministic, conversational, iterative.

**Being the plan** — the `.flo` runs. Every node is a typed agent invocation, a deterministic atom, or a substrate primitive (`for-each` / `compare` / `assert` / `snapshot` / `model-lock`). The runtime walks the DAG; no LLM is in the loop.

**Hard rules this implies:**

- Validators (`assert:` nodes) must be deterministic — atoms, agent commands, or inline glue. **No `think-node` / `smart-node` allowed.** Truth #9 makes this constraint structural, not stylistic.
- Approval gates (`approve:` nodes) post to a human channel; the *decision* is human, the *gating* is deterministic. An LLM may compose the message body but may not be the approver.
- Safety-contract pre-flight checks (worksharing, snapshot, transaction-group, audit-stamp) execute as Rust code in the runtime — never delegated to an LLM.
- The engineering envelope's `signed-output` receipt is produced by deterministic hashing + signing. An LLM may produce the *report* the receipt seals; it never produces the receipt itself.

What this leaves to AI: composition, summarisation, natural-language answers, suggestion. What this denies AI: validation, approval, sealing, gating, code-check decisions.

This is the rule that lets a structural engineer carry an AWARE-produced calc package into a court hearing. Without it, AWARE is a productivity tool; with it, AWARE is a deliverable.

---

These nine are the decalog. They are not goals to be balanced against other goals. They are constraints. Anything that breaks one is rejected, regardless of how clever, profitable, or popular the alternative looks.
