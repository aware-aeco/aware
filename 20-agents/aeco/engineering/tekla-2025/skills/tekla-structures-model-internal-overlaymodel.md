---
name: tekla-tekla-structures-model-internal-overlaymodel
description: This skill encodes the tekla 2025.0 surface of the Tekla.Structures.Model.Internal.OverlayModel namespace — 4 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: OverlayModelEvents, IOverlayModelEvents, OverlayModelCollectionChangesAction, OverlayModelEvents.OverlayModelCollectionChangedDelegate.
---

# Tekla.Structures.Model.Internal.OverlayModel

Auto-generated from vendor docs for tekla 2025.0. 4 types in this namespace.

## IOverlayModelEvents (interface)

Interface for the OverlayModelEvents class allows the user to register event listeners for OverlayModel events.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/1bd72aa5-d22c-71c1-a538-e7690d486747)

### Events
#### `OverlayModelCollectionChanged` (`Tekla.Structures.Model.Internal.OverlayModel.OverlayModelEvents.OverlayModelCollectionChangedDelegate`)

**Signature:** `event OverlayModelEvents.OverlayModelCollectionChangedDelegate OverlayModelCollectionChanged`

The OverlayModelCollectionChanged event is raised when an overlay model is changed (added/updated/removed).

[Docs](https://developer.tekla.com/topic/en/18/43/cddddf87-f612-8b80-8651-c769dd483482)

## OverlayModelCollectionChangesAction (enum)

The action of the overlay model collection changes

[Vendor docs](https://developer.tekla.com/topic/en/18/43/876446b4-e809-c164-7f0f-fc6e88211e01)

### Values
- `Added` = `0` — Added action
- `Removed` = `1` — Removed action
- `Updated` = `2` — Updated action

## OverlayModelEvents (class)

Event class to allow the user to subscribe for OverlayModel events.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/209f899a-9f34-2277-f9d1-0bd4864eb214)

### Constructors
- `public OverlayModelEvents()` — Initializes a new instance of the OverlayModelEvents class

### Events
#### `OverlayModelCollectionChanged` (`Tekla.Structures.Model.Internal.OverlayModel.OverlayModelEvents.OverlayModelCollectionChangedDelegate`)

**Signature:** `public event OverlayModelEvents.OverlayModelCollectionChangedDelegate OverlayModelCollectionChanged`

The OverlayModelCollectionChanged event is raised when an overlay model is changed (added/updated/removed).

[Docs](https://developer.tekla.com/topic/en/18/43/750c1e81-7bca-6605-4513-12880f3fe584)

## OverlayModelEvents.OverlayModelCollectionChangedDelegate (delegate)

The delegate for the OverlayModelCollectionChanged event

[Vendor docs](https://developer.tekla.com/topic/en/18/43/450cc808-78d9-f139-b6a4-0593c47218e5)

