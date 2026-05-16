# post-effect-collection-move-post-effect-before

Lifecycle: single

Move a post effect before another post effect in the list.             Param 'id_move' is the id of the post effect to move.             Param 'id_before' is the id of a post effect before which the post effect should be moved.             If this is Guid.Empty, the post effect is moved to the end of the list.             If the post effect identified by 'id_before' is not found, the method will fail.             Returns true if successful, else false.
