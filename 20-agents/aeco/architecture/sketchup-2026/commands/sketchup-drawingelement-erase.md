# sketchup-drawingelement-erase

Lifecycle: single

When erasing multiple elements, it's faster to use Entities#erase_entities and erase in bulk than to iterate individual drawing elements calling #erase!.
