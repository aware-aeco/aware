# gh-canvas-schedule-regen

Lifecycle: single

Schedule a regen to occur after the specified number of milliseconds have elapsed.   If a Regen is called in this time frame, the schedule will be cleared. Only a single   schedule can be active at any time, so any call to ScheduleRegen will clear   existing schedules.
