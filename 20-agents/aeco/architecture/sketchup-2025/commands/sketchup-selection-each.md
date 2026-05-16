# sketchup-selection-each

Lifecycle: single

Don't remove content from this collection while iterating over it with #each. This would change the size of the collection and cause elemnts to be skipped as the indices change. Instead copy the current collection to an array using to_a and then use each on the array, when removing content.
