# Sharing strategy

Dropbox shares come in three shapes. AECO workflows use all three.

## 1. Shared link (URL-based)

Best for: ad-hoc sends to clients, contractors who don't have a Dropbox account.

```yaml
- id: link
  agent: dropbox
  command: sharing.create-link
  inputs:
    remote-path: '/Projects/Acme/Tender/2026-05-18.zip'
    access: view
    expires-at: '2026-06-18T23:59:59Z'   # auto-revoke after 30 days
    password: '{{ secrets.tender-password }}'
```

Output URL goes straight into the consultant email. Dropbox tracks who clicks (in the Business audit log) so you can answer "did the contractor open the tender?"

## 2. Shared folder (member-based)

Best for: ongoing collaboration — consultants who need view/edit access for the project's duration.

```yaml
- id: share-tender
  agent: dropbox
  command: sharing.add-folder-member
  inputs:
    folder-path: '/Projects/Acme/Tender'
    emails:
      - struct@consultant.com
      - mep@consultant.com
    access: viewer
    send-notification: false   # we Slack them separately
```

Members appear in the Dropbox Business audit log; permissions can be revoked at any time.

## 3. Team space (org-internal)

Best for: in-firm collaboration. Already set up by IT; no AWARE-side action needed beyond uploading to the right path.

## Common pitfalls

- **Shared link expiry only works on Business tier.** Free / Plus accounts always create permanent links. The transport will return the URL but the `expires-at` field will be empty.
- **`access: edit` shares are dangerous.** Anyone with the URL + password can overwrite files. Default to `view`; only use `edit` for active co-design with a consultant you trust + revoke immediately after.
- **Password-protected links lull users.** Dropbox passwords aren't bulletproof — for actually confidential data (anything with PII, security drawings, defence) use SharePoint with conditional access or ACC Docs with project-membership gates instead.

## Audit trail

Every link create / share / revoke appears in the Dropbox Business
team-events log. For AWARE-issued shares, the receipt JSONL (from
v0.25 panel review + v0.21 output-seal) records:
- Which URL was created
- Who created it (operator)
- When
- Expiry + access level

So the client-issue audit trail spans both Dropbox (operational) and
AWARE (decision-making).
