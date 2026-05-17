# First hour — Designer

Goal: Monday concept shots posting to Teams automatically, no dev help.

## What you need

- Rhino 8 (any active .3dm)
- Microsoft 365 (or Google Workspace for Chat)
- 30 minutes

## 0-10 min: install AWARE

```powershell
npm install -g @aware-aeco/cli
aware --version

aware agent install aware-aeco
```

For renders later, install one of the prep agents:

```powershell
# If you use Enscape:
aware agent install enscape-prep

# If you use Twinmotion:
aware agent install twinmotion-prep
```

(Both are "honest" agents — neither tool has a scriptable render API, so AWARE preps the scene + routes output; you render in the desktop app. See `tapir-vs-official-api.md` in the agent's skills folder for the why.)

## 10-20 min: connect Teams

```powershell
aware connect microsoft-365 --device-code
```

For Rhino: no `aware connect` needed. The agent talks to Rhino locally.

## 20-35 min: name your views

The `designer-monday-shots` reference app expects three named views in your Rhino file: `SE Perspective`, `NW Perspective`, `Aerial Bird-eye`. In Rhino:

1. Position the camera
2. *View → Named Views → Save As...*
3. Name it `SE Perspective` (exact match — case sensitive)
4. Repeat for the other two

This one-time setup means every Monday's shot uses the *same* camera — no drift across weeks.

If your project's view names differ, edit the reference app:

```yaml
- id: shot-se
  agent: rhino-8
  command: view.capture
  inputs:
    view-name: 'Your View Name Here'
```

## 35-50 min: install + customise

```powershell
aware app install https://github.com/aware-aeco/aware/raw/main/30-apps/_examples/designer-monday-shots.flo
```

Edit `~/.aware/apps/designer-monday-shots/designer-monday-shots.flo`:

```yaml
inputs:
  rhino-file: '\\office\projects\acme\acme.3dm'
  shot-dir: '\\office\projects\acme\monday-shots'
  teams-team-id: 'Design Studio'
```

## 50-60 min: dry-run + ship

```powershell
aware app run designer-monday-shots --dry-run
```

Then for real:

```powershell
aware app run designer-monday-shots
```

Three PNGs land in `\\office\projects\acme\monday-shots\2026-05-17-{se|nw|aerial}.png` and a card posts to the design-review Teams channel.

## What you've achieved

- Every Monday morning: 3 consistent perspective shots in your design-review channel
- Replaces the Sunday-night "what did I change this week" panic ritual
- Frees the principal's review time from chasing screenshots

## Next

- Add an Enscape pass: install `enscape-prep`, add a `scene.prep` node before `view.capture` to apply consistent sun + layer state
- Compare A vs B options: duplicate the captures with a different `rhino-file` input, post both side-by-side
- For parametric studies, look at `rhino-8.gh.sweep` — the audit named this as the Colibri-without-the-plugin verb
