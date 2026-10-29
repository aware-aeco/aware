# field-domain-points-by-uv-set-grid-coordinates

Lifecycle: single

Set u and v coordinates that specify a grid on the surface.    The display of the grid is controlled by AnalysisDisplayColoredSurfaceSettings::getShowGridLines().    If AnalysisDisplayColoredSurfaceSettings::getShowGridLines() returns true and both sets are empty    then a grid will be displayed using a default spacing; if only one of the sets is non-empty, then    only the corresponding set of grid lines will be displayed, i.e. the grid will consist solely of    parallel lines at the specified coordinates.
