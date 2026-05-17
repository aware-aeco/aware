# RIBA stage spec mapping

A spec's stage drives both its content + its level of detail. AWARE-side
apps should respect this when filtering / aggregating.

## Stage 0-2: Strategic Definition / Briefing / Concept

- Specs at this stage are *outline* — narrative paragraphs, not
  clause-by-clause text
- Materials named in concept terms ("natural stone façade") not product
  level ("Portland Whitbed limestone, 80mm honed")
- Chorus exposes these as `stage: '0'`, `stage: '1'`, `stage: '2'`
- Use case: client-deck output, planning submissions

## Stage 3: Spatial Coordination (formerly Developed Design)

- First *real* spec — clauses start tightening
- Manufacturer-neutral but product-class specific ("clay brick to BS EN
  771-1 frost-resistant F2")
- Use case: tender stage, BIM data drops 1-2

## Stage 4: Technical Design

- The production spec — fully clause-level, manufacturer-aware where
  fixed, performance-only where contractor's choice
- The bulk of the spec authoring time goes here
- Use case: Issued-for-Construction package

## Stage 5: Manufacturing & Construction

- Site-side updates to the Stage 4 spec — RFI-driven changes, technical
  queries answered, substitutions logged
- Use case: variation tracking, NCR response

## Stage 6-7: Handover / Use

- O&M-flavoured spec content — maintenance regimes, replacement
  intervals
- Use case: COBie handover, asset register population

## Worked example: tender spec issue

```yaml
- id: stage4-specs
  agent: nbs-chorus
  command: spec.list
  inputs:
    project-id: '{{ secrets.acme-project-id }}'
    stage: '4'

- id: export-each
  for-each: '{{ stage4-specs.specs }}'      # v0.19 primitive
  do:
    - agent: nbs-chorus
      command: spec.export-pdf
      inputs:
        spec-id: '{{ item.id }}'
        output-path: '{{ run.tmp-dir }}/spec-{{ item.discipline }}.pdf'

- id: bundle
  agent: aware-runtime
  command: zip
  inputs:
    paths: '{{ export-each.outputs[*].path }}'
    output-path: '{{ run.tmp-dir }}/acme-tender-specs-{{ run.date }}.zip'

- id: upload
  agent: acc-docs
  command: files.upload
  inputs:
    project-id: '{{ secrets.acc-project-id }}'
    folder-id: '{{ secrets.acc-tender-folder-id }}'
    file-path: '{{ bundle.path }}'
  safety:
    transaction-group: tender-issue
    snapshot: false
```
