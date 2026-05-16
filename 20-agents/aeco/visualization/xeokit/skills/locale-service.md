---
name: xeokit-sdk-locale-service
description: LocaleService declarations from xeokit-sdk
---

# LocaleService

## Methods

- `loadMessages(messages?: any)`
- `clearMessages()`
- `translate(msg: string, args?: any)`
- `translatePlurals(msg: string, count: number, args?: any)`
- `fire(event: string, value: any, forget?: boolean)`
- `on(event: string, callback: Function)`
- `off(subId: string)`
- `on(event: "updated", callback: (e: LocaleService | string) => void)`
