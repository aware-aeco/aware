---
name: components-classifier
description: Classifier declarations from components
---

# Classifier

## Methods

- `getGroupData(classification: string, group: string)`
- `aggregateItems()`
- `addGroupItems(classification: string, group: string, items: ModelIdMap)`
- `setGroupQuery(classification: string, group: string, query: ClassificationGroupQuery)`
- `find(data: ClassifierIntersectionInput)`
- `aggregateItemRelations(classification: string, query: FRAGS.ItemsQueryParams, relation: string, config?: ClassifyItemRelationsConfig)`
- `byIfcBuildingStorey(config?: AddClassificationConfig)`
- `byCategory(config?: AddClassificationConfig)`
- `dispose()`
- `removeItems(modelIdMap: ModelIdMap, config?: RemoveClassifierItemsConfig)`
- `byModel(config?: AddClassificationConfig)`
