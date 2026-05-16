---
name: yard-sketchup-pages
description: Sketchup::Pages API reference (YARD)
---

# Sketchup::Pages API reference

The Pages class contains methods for manipulating a collection of Pages (Named “scenes” in the UI.) in a model.

## Methods

- `add_frame_change_observer` — The add_frame_change_observer method is used to add a new frame change observer that is called with each frame of an animation, meaning the end user has clicked on a Scene tab (aka Page) inside SketchUp and the camera is animating to that scene.
- `remove_frame_change_observer` — The remove_frame_change_observer method is used to remove a frame change observer
- `[]` — The [] method retrieves a page by either name or index.
- `add` — The #add method is used to add a new Page object to the collection.
- `add_matchphoto_page` — The #add_matchphoto_page method is used to add a photomatch page to the model. This is an advanced feature that was added to support internal SketchUp work, so it is unlikely to be useful to you.
- `add_observer` — The add_observer method is used to add an observer to the Pages object. See the PagesObserver interface for more details.
- `count` — Since SketchUp 2014 the count method is inherited from Ruby's Enumerable mix-in module. Prior to that the #count method is an alias for #length.
- `each` — The #each method is used to iterate through pages in the model.
- `erase` — The #erase method is used to remove a page from the collection.
- `length` — The #length method is an alias for #size.
- `parent` — The parent method is used to determine the model for the Pages collection.
- `remove_observer` — The remove_observer method is used to remove an observer from the current object. See the PagesObserver interface for more details.
- `reorder` — The #reorder method is used to reorder an existing Sketchup::Page object inside collection.
- `selected_page` — The selected_page method is used to retrieve the currently selected page.
- `selected_page=` — The selected_page method is used to set the currently selected page. Once you set this, SketchUp will animate to that page as if the user had clicked on its scene tab.
- `show_frame_at` — In versions prior to SketchUp 2019 this method will crash if called when there are no pages in the model.
- `size` — The #size method is used to retrieve the number of pages.
- `slideshow_time` — The slideshow_time method is used to get the amount of time that a slideshow of all of the pages will take. This takes into account the transition time for each Page and the amount of time that each Page is displayed.
