# Aconex Mail is not email

The most common Aconex confusion. Critical to get right when wiring into AECO workflows.

## What Aconex Mail is

- **The legally-binding project correspondence register.** Every mail item is contractually attributable, time-stamped, court-admissible. UK BD-funded projects often *require* all project correspondence to flow through Aconex Mail for the audit trail.
- **Strongly typed.** Each mail has a type code — `DrawingSubmission`, `RFI`, `TQ` (Technical Query), `Variation`, `SiteInstruction`, `NCR` (Non-Conformance Report). The type drives required fields + downstream workflows.
- **Persistent.** Mails never disappear. Closed = archived in the register; not deleted.
- **One-to-many or many-to-one.** Aconex supports proper threading — a single TQ can be sent to multiple consultants, each respond, the chain remains intact.

## What it's not

- ❌ Outlook email. Don't pipe project-significant correspondence through `outlook.mail.send` if Aconex is the contractual CDE — it won't appear in the project register.
- ❌ Slack / Teams. Casual coordination only.
- ❌ Procore RFIs. Procore RFIs live in Procore — *parallel* register on the GC's platform. Some projects run both (audit trail in Aconex, day-to-day in Procore).

## When to reach for which

| Communication | Tool |
|---|---|
| "Hey, can we move the meeting?" | Outlook / Teams |
| "Standard daily coordination across consultants" | Outlook / Teams |
| "Formal request for an instruction — needs an answer on record" | Aconex Mail (`mail.send`, type: TQ) |
| "Submit a drawing revision to the contract administrator" | Aconex Mail (type: DrawingSubmission) |
| "Issue a Site Instruction without cost impact" | Aconex Mail (type: SiteInstruction) — *and* ASI in Procore if dual-platform |
| "Notify of a non-conformance" | Aconex Mail (type: NCR) |

## Threading

`mail.respond` preserves the mail-id chain. The output `response-mail-id` is a new mail that references the original — Aconex's UI shows them as a thread. **Do not** use `mail.send` for a response; that creates a new disconnected thread.

## Types are project-specific

Each project's Aconex setup enables a subset of mail types. Use `mail.list --status any` first if uncertain which types are enabled.

## Auth scope

`aware connect aconex` uses OAuth 2.0. Aconex requires an organisation-level OAuth App registration — talk to your Oracle account manager. For tenant/organisation-restricted environments, the device-code flow (v0.13) works:

```bash
aware connect aconex --device-code
```
