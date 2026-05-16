# light-manager-support-set-light-solo

Lifecycle: single

First checks to see if we are in "solo mode" - which means that there are any lights that respond "true" to IsInSoloStorage.             If in solo mode:              If bSolo = true               Sets this light on.              If bSolo = false               If this is the last light "on", forces all lights out of solo mode.               If there are other lights on, turns this light off.             If not in solo mode:              If bSolo = true               Forces all lights into solo storage and sets this light on.              If bSolo = false               This shouldn't happen.  Will cause an ASSERT
