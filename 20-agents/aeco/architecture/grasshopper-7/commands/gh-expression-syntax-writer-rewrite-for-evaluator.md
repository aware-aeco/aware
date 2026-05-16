# gh-expression-syntax-writer-rewrite-for-evaluator

Lifecycle: single

Rewrite the expression so that all temporary keywords and symbols are replaced by evaluator-specific chars.  The expression will become less readable, do not let the user see the result of this function.  You need to rewrite the expression with this function if you intend to feed it into the Evaluator.
