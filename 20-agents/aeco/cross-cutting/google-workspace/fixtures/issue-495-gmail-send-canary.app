app: issue-495-gmail-send-canary
version: 0.1.0
description: Controlled live-provider canary for the Gmail send contract.

requires:
  - google-workspace@2.x

layout: linear
nodes:
  - id: send
    agent: google-workspace
    command: gmail.send
    config:
      to: ["replace-with-controlled-recipient@example.invalid"]
      bcc: ["replace-with-controlled-bcc@example.invalid"]
      subject: "AWARE Gmail canary — replace-with-unique-marker"
      body: |
        Controlled AWARE issue #495 Gmail canary.
        Attempt marker: replace-with-unique-attempt-id
      content-type: text
      attempt-id: replace-with-unique-attempt-id
    safety:
      transaction-group: issue-495-gmail-canary
      snapshot: false

connections: []
