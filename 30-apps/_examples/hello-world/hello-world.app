app:           hello-world
version:       0.1.0
display-name:  AWARE Hello World
description: |
  Smoke-test composition that fires a single message into Tekla's status
  bar via the curated `tekla.send-status` verb. Proves the AWARE runtime
  can drive a real desktop host end-to-end — first time the substrate
  actually executes something, not just describes it.

  Spike scope (v0.29.0): Tekla only. Teams + email destinations are
  defined as inputs but commented out until `aware connect microsoft-365`
  is run. Uncomment the m365 nodes once OAuth is wired.

exposes-as-agent: false

requires:
  - tekla@0.1.x

requires-permissions:
  network:
    - localhost:9999       # Tekla COM bridge (Open API)

layout: linear

nodes:
  - id: tekla-2025-status
    agent: tekla
    command: send-status
    config:
      message: "Hello AWARE 2025 — substrate executes end-to-end"
      version: "2025.0"

  - id: tekla-2026-status
    agent: tekla
    command: send-status
    config:
      message: "Hello AWARE 2026 — substrate executes end-to-end"
      version: "2026.0"

  # ── Disabled until `aware connect microsoft-365` is run ──────────────
  # - id: email-pawel
  #   agent: microsoft-365
  #   command: outlook.mail.send
  #   inputs:
  #     to:       ["pawel@floless.io"]
  #     subject:  "AWARE Hello World"
  #     body:     "{{ inputs.message }}"
  #
  # - id: teams-notify
  #   agent: microsoft-365
  #   command: teams.chat.post-message
  #   inputs:
  #     chat_id: "{{ inputs.teams-chat-id }}"
  #     message: "{{ inputs.message }}"
