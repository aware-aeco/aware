---
name: idea-statica-ideastatica-rcsapi-model
description: This skill encodes the idea-statica 25.0 surface of the IdeaStatiCa.RcsApi.Model namespace — 54 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: CalculationType, CheckResult, CheckResultType, ConcreteCheckResult, ConcreteCheckResultBase, ConcreteCheckResultOverall, ConcreteCheckResultOverallItem, ConcreteCheckResults, and 46 more types.
---

# IdeaStatiCa.RcsApi.Model

Auto-generated from vendor docs for idea-statica 25.0. 54 types in this namespace.

## CalculationType (enum)

(No description provided in vendor docs for IdeaStatiCa.RcsApi.Model.CalculationType.)

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/25.1.5.1491/src/api-sdks/rcs-api/clients/csharp/docs/CalculationType.md)

### Values
- `notDefined` = `notDefined`
- `capacity` = `capacity`
- `response` = `response`
- `shear` = `shear`
- `torsion` = `torsion`
- `interaction` = `interaction`
- `fatigue` = `fatigue`
- `stressLimitationShort` = `stressLimitationShort`
- `stressLimitationLong` = `stressLimitationLong`
- `stressLimitationStage` = `stressLimitationStage`
- `crackWidthShort` = `crackWidthShort`
- `crackWidthLong` = `crackWidthLong`
- `crackWidthStage` = `crackWidthStage`
- `stiffnessShort` = `stiffnessShort`
- `stiffnessLong` = `stiffnessLong`
- `stiffnessStage` = `stiffnessStage`
- `deflection` = `deflection`
- `brittleFailure` = `brittleFailure`
- `detailing` = `detailing`
- `linearStressShort` = `linearStressShort`
- `linearStressLong` = `linearStressLong`
- `linearStressStage` = `linearStressStage`
- `crossSectionCharacteristicsShort` = `crossSectionCharacteristicsShort`
- `crossSectionCharacteristicsLong` = `crossSectionCharacteristicsLong`
- `crossSectionCharacteristicsStage` = `crossSectionCharacteristicsStage`
- `baumann` = `baumann`
- `overallColumnRecalculation` = `overallColumnRecalculation`
- `columnForces` = `columnForces`
- `prestressEffects` = `prestressEffects`
- `primaryforces` = `primaryforces`
- `longTermLosses` = `longTermLosses`
- `mnKappaDiagramULS` = `mnKappaDiagramULS`
- `mnKappaDiagramSLSShort` = `mnKappaDiagramSLSShort`
- `mnKappaDiagramSLSLong` = `mnKappaDiagramSLSLong`
- `creepAndShrinkageCoefficient` = `creepAndShrinkageCoefficient`
- `concreteCover` = `concreteCover`
- `crackingCalculation` = `crackingCalculation`
- `crackingCalculationShort` = `crackingCalculationShort`
- `crackingCalculationLong` = `crackingCalculationLong`
- `redistribution` = `redistribution`
- `initilaStateofCrossSection` = `initilaStateofCrossSection`
- `strutAngle` = `strutAngle`
- `designReinforcement` = `designReinforcement`
- `capacityMesh` = `capacityMesh`
- `stiffnessStageLong` = `stiffnessStageLong`
- `longTermLossesCoefficient` = `longTermLossesCoefficient`

## CheckResult (enum)

(No description provided in vendor docs for IdeaStatiCa.RcsApi.Model.CheckResult.)

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/25.1.5.1491/src/api-sdks/rcs-api/clients/csharp/docs/CheckResult.md)

### Values
- `failed` = `failed`
- `passed` = `passed`
- `passedWithWarnings` = `passedWithWarnings`
- `failedWithError` = `failedWithError`
- `checkIsOff` = `checkIsOff`
- `notDone` = `notDone`
- `checkIsNotNecesssary` = `checkIsNotNecesssary`

## CheckResultType (enum)

(No description provided in vendor docs for IdeaStatiCa.RcsApi.Model.CheckResultType.)

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/25.1.5.1491/src/api-sdks/rcs-api/clients/csharp/docs/CheckResultType.md)

### Values
- `capacity` = `capacity`
- `response` = `response`
- `shear` = `shear`
- `torsion` = `torsion`
- `interaction` = `interaction`
- `fatigue` = `fatigue`
- `stressLimitation` = `stressLimitation`
- `crackWidth` = `crackWidth`
- `detailing` = `detailing`
- `stiffness` = `stiffness`
- `deflection` = `deflection`

## ConcreteCheckResult (class)

(No description provided in vendor docs for IdeaStatiCa.RcsApi.Model.ConcreteCheckResult.)

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/25.1.5.1491/src/api-sdks/rcs-api/clients/csharp/docs/ConcreteCheckResult.md)

### Properties
- `ResultType` (CheckResultType, get/set) — (No description provided in vendor docs for ConcreteCheckResult.resultType.)
- `CheckResults` (List<ConcreteCheckResultBase>, get/set) — (No description provided in vendor docs for ConcreteCheckResult.checkResults.)

## ConcreteCheckResultBase (class)

(No description provided in vendor docs for IdeaStatiCa.RcsApi.Model.ConcreteCheckResultBase.)

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/25.1.5.1491/src/api-sdks/rcs-api/clients/csharp/docs/ConcreteCheckResultBase.md)

### Properties
- `InternalFores` (ResultOfInternalForces, get/set) — (No description provided in vendor docs for ConcreteCheckResultBase.internalFores.)
- `NonConformities` (List<NonConformity>, get/set) — (No description provided in vendor docs for ConcreteCheckResultBase.nonConformities.)
- `Result` (CheckResult, get/set) — (No description provided in vendor docs for ConcreteCheckResultBase.result.)
- `CheckValue` (double, get/set) — (No description provided in vendor docs for ConcreteCheckResultBase.checkValue.)
- `LimitCheckValue` (double, get/set) — (No description provided in vendor docs for ConcreteCheckResultBase.limitCheckValue.)
- `Check` (CalculationType, get/set) — (No description provided in vendor docs for ConcreteCheckResultBase.check.)

## ConcreteCheckResultOverall (class)

(No description provided in vendor docs for IdeaStatiCa.RcsApi.Model.ConcreteCheckResultOverall.)

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/25.1.5.1491/src/api-sdks/rcs-api/clients/csharp/docs/ConcreteCheckResultOverall.md)

### Properties
- `Checks` (List<ConcreteCheckResultOverallItem>, get/set) — (No description provided in vendor docs for ConcreteCheckResultOverall.checks.)

## ConcreteCheckResultOverallItem (class)

(No description provided in vendor docs for IdeaStatiCa.RcsApi.Model.ConcreteCheckResultOverallItem.)

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/25.1.5.1491/src/api-sdks/rcs-api/clients/csharp/docs/ConcreteCheckResultOverallItem.md)

### Properties
- `ResultType` (CheckResultType, get/set) — (No description provided in vendor docs for ConcreteCheckResultOverallItem.resultType.)
- `Result` (CheckResult, get/set) — (No description provided in vendor docs for ConcreteCheckResultOverallItem.result.)
- `CheckValue` (double, get/set) — (No description provided in vendor docs for ConcreteCheckResultOverallItem.checkValue.)

## ConcreteCheckResults (class)

(No description provided in vendor docs for IdeaStatiCa.RcsApi.Model.ConcreteCheckResults.)

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/25.1.5.1491/src/api-sdks/rcs-api/clients/csharp/docs/ConcreteCheckResults.md)

### Properties
- `CheckResults` (List<ConcreteCheckResult>, get/set) — (No description provided in vendor docs for ConcreteCheckResults.checkResults.)
- `Overall` (ConcreteCheckResultOverall, get/set) — (No description provided in vendor docs for ConcreteCheckResults.overall.)

## FatigueTypeOfPrestressingSteel (enum)

(No description provided in vendor docs for IdeaStatiCa.RcsApi.Model.FatigueTypeOfPrestressingSteel.)

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/25.1.5.1491/src/api-sdks/rcs-api/clients/csharp/docs/FatigueTypeOfPrestressingSteel.md)

### Values
- `postTensioningSingleStrandsInPlasticDucts` = `postTensioningSingleStrandsInPlasticDucts`
- `postTensioningStraightTendonsOrCurvedTendonsInPlasticDucts` = `postTensioningStraightTendonsOrCurvedTendonsInPlasticDucts`
- `postTensioningCurvedTendonsInSteelDucts` = `postTensioningCurvedTendonsInSteelDucts`
- `postTensioningSplicingDevices` = `postTensioningSplicingDevices`
- `preTensioning` = `preTensioning`
- `externalTendon` = `externalTendon`

## ImportIOMFile_request (class)

(No description provided in vendor docs for IdeaStatiCa.RcsApi.Model.ImportIOMFile_request.)

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/25.1.5.1491/src/api-sdks/rcs-api/clients/csharp/api/openapi.yaml)

### Properties
- `IomFile` (Stream, get/set) — (No description provided in vendor docs for ImportIOMFile_request.iomFile.)

## Loading (class)

(No description provided in vendor docs for IdeaStatiCa.RcsApi.Model.Loading.)

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/25.1.5.1491/src/api-sdks/rcs-api/clients/csharp/docs/Loading.md)

### Properties
- `LoadingType` (LoadingType, get/set) — (No description provided in vendor docs for Loading.loadingType.)
- `Id` (int, get/set) — (No description provided in vendor docs for Loading.id.)

## LoadingType (enum)

(No description provided in vendor docs for IdeaStatiCa.RcsApi.Model.LoadingType.)

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/25.1.5.1491/src/api-sdks/rcs-api/clients/csharp/docs/LoadingType.md)

### Values
- `loadCase` = `loadCase`
- `combination` = `combination`
- `resultClass` = `resultClass`

## MaterialDuct (enum)

(No description provided in vendor docs for IdeaStatiCa.RcsApi.Model.MaterialDuct.)

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/25.1.5.1491/src/api-sdks/rcs-api/clients/csharp/docs/MaterialDuct.md)

### Values
- `metal` = `metal`
- `plastic` = `plastic`

## NonConformity (class)

(No description provided in vendor docs for IdeaStatiCa.RcsApi.Model.NonConformity.)

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/25.1.5.1491/src/api-sdks/rcs-api/clients/csharp/docs/NonConformity.md)

### Properties
- `Guid` (Guid, get/set) — (No description provided in vendor docs for NonConformity.guid.)
- `Severity` (NonConformitySeverity, get/set) — (No description provided in vendor docs for NonConformity.severity.)

## NonConformityIssue (class)

(No description provided in vendor docs for IdeaStatiCa.RcsApi.Model.NonConformityIssue.)

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/25.1.5.1491/src/api-sdks/rcs-api/clients/csharp/docs/NonConformityIssue.md)

### Properties
- `Guid` (Guid, get/set) — (No description provided in vendor docs for NonConformityIssue.guid.)
- `Description` (string, get/set) — (No description provided in vendor docs for NonConformityIssue.description.)
- `Severity` (NonConformitySeverity, get/set) — (No description provided in vendor docs for NonConformityIssue.severity.)

## NonConformitySeverity (enum)

(No description provided in vendor docs for IdeaStatiCa.RcsApi.Model.NonConformitySeverity.)

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/25.1.5.1491/src/api-sdks/rcs-api/clients/csharp/docs/NonConformitySeverity.md)

### Values
- `info` = `info`
- `warning` = `warning`
- `error` = `error`
- `terminatedError` = `terminatedError`

## OpenElementId (class)

(No description provided in vendor docs for IdeaStatiCa.RcsApi.Model.OpenElementId.)

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/25.1.5.1491/src/api-sdks/rcs-api/clients/csharp/docs/OpenElementId.md)

### Properties
- `Id` (int, get/set) — (No description provided in vendor docs for OpenElementId.id.)

## OpenProject_request (class)

(No description provided in vendor docs for IdeaStatiCa.RcsApi.Model.OpenProject_request.)

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/25.1.5.1491/src/api-sdks/rcs-api/clients/csharp/api/openapi.yaml)

### Properties
- `RcsFile` (Stream, get/set) — Rcs file

## Point2D (class)

(No description provided in vendor docs for IdeaStatiCa.RcsApi.Model.Point2D.)

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/25.1.5.1491/src/api-sdks/rcs-api/clients/csharp/docs/Point2D.md)

### Properties
- `X` (double, get/set) — (No description provided in vendor docs for Point2D.x.)
- `Y` (double, get/set) — (No description provided in vendor docs for Point2D.y.)

## PolyLine2D (class)

(No description provided in vendor docs for IdeaStatiCa.RcsApi.Model.PolyLine2D.)

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/25.1.5.1491/src/api-sdks/rcs-api/clients/csharp/docs/PolyLine2D.md)

### Properties
- `StartPoint` (Point2D, get/set) — (No description provided in vendor docs for PolyLine2D.startPoint.)
- `Segments` (List<Segment2D>, get/set) — (No description provided in vendor docs for PolyLine2D.segments.)

## RcsCalculationParameters (class)

(No description provided in vendor docs for IdeaStatiCa.RcsApi.Model.RcsCalculationParameters.)

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/25.1.5.1491/src/api-sdks/rcs-api/clients/csharp/docs/RcsCalculationParameters.md)

### Properties
- `Sections` (List<int>, get/set) — (No description provided in vendor docs for RcsCalculationParameters.sections.)

## RcsCheckMember (class)

(No description provided in vendor docs for IdeaStatiCa.RcsApi.Model.RcsCheckMember.)

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/25.1.5.1491/src/api-sdks/rcs-api/clients/csharp/docs/RcsCheckMember.md)

### Properties
- `Id` (int, get/set) — (No description provided in vendor docs for RcsCheckMember.id.)
- `Name` (string, get/set) — (No description provided in vendor docs for RcsCheckMember.name.)

## RcsCrossSectionData (class)

(No description provided in vendor docs for IdeaStatiCa.RcsApi.Model.RcsCrossSectionData.)

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/25.1.5.1491/src/api-sdks/rcs-api/clients/csharp/docs/RcsCrossSectionData.md)

### Properties
- `Name` (string, get/set) — (No description provided in vendor docs for RcsCrossSectionData.name.)
- `CrossSectionRotation` (double, get/set) — (No description provided in vendor docs for RcsCrossSectionData.crossSectionRotation.)
- `Components` (List<RcsCssComponentData>, get/set) — (No description provided in vendor docs for RcsCrossSectionData.components.)

## RcsCssComponentData (class)

(No description provided in vendor docs for IdeaStatiCa.RcsApi.Model.RcsCssComponentData.)

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/25.1.5.1491/src/api-sdks/rcs-api/clients/csharp/docs/RcsCssComponentData.md)

### Properties
- `MaterialName` (string, get/set) — (No description provided in vendor docs for RcsCssComponentData.materialName.)
- `Phase` (int, get/set) — (No description provided in vendor docs for RcsCssComponentData.phase.)
- `Geometry` (Region2D, get/set) — (No description provided in vendor docs for RcsCssComponentData.geometry.)

## RcsMprlElement (class)

(No description provided in vendor docs for IdeaStatiCa.RcsApi.Model.RcsMprlElement.)

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/25.1.5.1491/src/api-sdks/rcs-api/clients/csharp/docs/RcsMprlElement.md)

### Properties
- `Name` (string, get/set) — (No description provided in vendor docs for RcsMprlElement.name.)

## RcsProject (class)

(No description provided in vendor docs for IdeaStatiCa.RcsApi.Model.RcsProject.)

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/25.1.5.1491/src/api-sdks/rcs-api/clients/csharp/docs/RcsProject.md)

### Properties
- `Sections` (List<RcsSection>, get/set) — (No description provided in vendor docs for RcsProject.sections.)
- `CheckMembers` (List<RcsCheckMember>, get/set) — (No description provided in vendor docs for RcsProject.checkMembers.)
- `ReinforcedCrossSections` (List<RcsReinforcedCrossSection>, get/set) — (No description provided in vendor docs for RcsProject.reinforcedCrossSections.)
- `ProjectId` (Guid, get/set) — (No description provided in vendor docs for RcsProject.projectId.)
- `ProjectData` (RcsProjectData, get/set) — (No description provided in vendor docs for RcsProject.projectData.)

## RcsProjectData (class)

(No description provided in vendor docs for IdeaStatiCa.RcsApi.Model.RcsProjectData.)

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/25.1.5.1491/src/api-sdks/rcs-api/clients/csharp/docs/RcsProjectData.md)

### Properties
- `ProjectName` (string, get/set) — (No description provided in vendor docs for RcsProjectData.projectName.)
- `DateOfCreate` (DateTime, get/set) — (No description provided in vendor docs for RcsProjectData.dateOfCreate.)
- `Description` (string, get/set) — (No description provided in vendor docs for RcsProjectData.description.)
- `Author` (string, get/set) — (No description provided in vendor docs for RcsProjectData.author.)
- `Code` (string, get/set) — (No description provided in vendor docs for RcsProjectData.code.)
- `ProjectNo` (string, get/set) — (No description provided in vendor docs for RcsProjectData.projectNo.)
- `TypeBridge` (string, get/set) — (No description provided in vendor docs for RcsProjectData.typeBridge.)
- `TypeOfStructure` (string, get/set) — (No description provided in vendor docs for RcsProjectData.typeOfStructure.)
- `FireResistanceCheck` (bool, get/set) — (No description provided in vendor docs for RcsProjectData.fireResistanceCheck.)

## RcsReinforcedBarData (class)

(No description provided in vendor docs for IdeaStatiCa.RcsApi.Model.RcsReinforcedBarData.)

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/25.1.5.1491/src/api-sdks/rcs-api/clients/csharp/docs/RcsReinforcedBarData.md)

### Properties
- `Diameter` (double, get/set) — (No description provided in vendor docs for RcsReinforcedBarData.diameter.)
- `Point` (Point2D, get/set) — (No description provided in vendor docs for RcsReinforcedBarData.point.)
- `MaterialName` (string, get/set) — (No description provided in vendor docs for RcsReinforcedBarData.materialName.)

## RcsReinforcedCrossSection (class)

(No description provided in vendor docs for IdeaStatiCa.RcsApi.Model.RcsReinforcedCrossSection.)

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/25.1.5.1491/src/api-sdks/rcs-api/clients/csharp/docs/RcsReinforcedCrossSection.md)

### Properties
- `Id` (int, get/set) — (No description provided in vendor docs for RcsReinforcedCrossSection.id.)
- `Name` (string, get/set) — (No description provided in vendor docs for RcsReinforcedCrossSection.name.)
- `CrossSectionId` (int, get/set) — (No description provided in vendor docs for RcsReinforcedCrossSection.crossSectionId.)

## RcsReinforcedCrossSectionImportData (class)

(No description provided in vendor docs for IdeaStatiCa.RcsApi.Model.RcsReinforcedCrossSectionImportData.)

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/25.1.5.1491/src/api-sdks/rcs-api/clients/csharp/docs/RcsReinforcedCrossSectionImportData.md)

### Properties
- `Setting` (RcsReinforcedCrosssSectionImportSetting, get/set) — (No description provided in vendor docs for RcsReinforcedCrossSectionImportData.setting.)
- `Template` (string, get/set) — (No description provided in vendor docs for RcsReinforcedCrossSectionImportData.template.)

## RcsReinforcedCrosssSectionImportSetting (class)

(No description provided in vendor docs for IdeaStatiCa.RcsApi.Model.RcsReinforcedCrosssSectionImportSetting.)

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/25.1.5.1491/src/api-sdks/rcs-api/clients/csharp/docs/RcsReinforcedCrosssSectionImportSetting.md)

### Properties
- `ReinforcedCrossSectionId` (int, get/set) — (No description provided in vendor docs for RcsReinforcedCrosssSectionImportSetting.reinforcedCrossSectionId.)
- `PartsToImport` (string, get/set) — (No description provided in vendor docs for RcsReinforcedCrosssSectionImportSetting.partsToImport.)

## RcsResultParameters (class)

(No description provided in vendor docs for IdeaStatiCa.RcsApi.Model.RcsResultParameters.)

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/25.1.5.1491/src/api-sdks/rcs-api/clients/csharp/docs/RcsResultParameters.md)

### Properties
- `Sections` (List<int>, get/set) — (No description provided in vendor docs for RcsResultParameters.sections.)

## RcsSection (class)

(No description provided in vendor docs for IdeaStatiCa.RcsApi.Model.RcsSection.)

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/25.1.5.1491/src/api-sdks/rcs-api/clients/csharp/docs/RcsSection.md)

### Properties
- `Id` (int, get/set) — (No description provided in vendor docs for RcsSection.id.)
- `Name` (string, get/set) — (No description provided in vendor docs for RcsSection.name.)
- `Description` (string, get/set) — (No description provided in vendor docs for RcsSection.description.)
- `CheckMemberId` (int, get/set) — (No description provided in vendor docs for RcsSection.checkMemberId.)
- `RcsId` (int, get/set) — (No description provided in vendor docs for RcsSection.rcsId.)

## RcsSectionLoading (class)

(No description provided in vendor docs for IdeaStatiCa.RcsApi.Model.RcsSectionLoading.)

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/25.1.5.1491/src/api-sdks/rcs-api/clients/csharp/docs/RcsSectionLoading.md)

### Properties
- `SectionId` (int, get/set) — (No description provided in vendor docs for RcsSectionLoading.sectionId.)
- `LoadingXml` (string, get/set) — (No description provided in vendor docs for RcsSectionLoading.loadingXml.)

## RcsSectionResultDetailed (class)

(No description provided in vendor docs for IdeaStatiCa.RcsApi.Model.RcsSectionResultDetailed.)

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/25.1.5.1491/src/api-sdks/rcs-api/clients/csharp/docs/RcsSectionResultDetailed.md)

### Properties
- `Id` (int, get/set) — (No description provided in vendor docs for RcsSectionResultDetailed.id.)
- `SectionResult` (SectionConcreteCheckResult, get/set) — (No description provided in vendor docs for RcsSectionResultDetailed.sectionResult.)
- `Issues` (List<NonConformityIssue>, get/set) — (No description provided in vendor docs for RcsSectionResultDetailed.issues.)

## RcsSectionResultOverview (class)

(No description provided in vendor docs for IdeaStatiCa.RcsApi.Model.RcsSectionResultOverview.)

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/25.1.5.1491/src/api-sdks/rcs-api/clients/csharp/docs/RcsSectionResultOverview.md)

### Properties
- `SectionId` (int, get/set) — (No description provided in vendor docs for RcsSectionResultOverview.sectionId.)
- `OverallItems` (List<ConcreteCheckResultOverallItem>, get/set) — (No description provided in vendor docs for RcsSectionResultOverview.overallItems.)

## RcsSetting (class)

(No description provided in vendor docs for IdeaStatiCa.RcsApi.Model.RcsSetting.)

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/25.1.5.1491/src/api-sdks/rcs-api/clients/csharp/docs/RcsSetting.md)

### Properties
- `Id` (int, get/set) — (No description provided in vendor docs for RcsSetting.id.)
- `Type` (string, get/set) — (No description provided in vendor docs for RcsSetting.type.)
- `Value` (object, get/set) — (No description provided in vendor docs for RcsSetting.value.)

## RcsStirrupsData (class)

(No description provided in vendor docs for IdeaStatiCa.RcsApi.Model.RcsStirrupsData.)

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/25.1.5.1491/src/api-sdks/rcs-api/clients/csharp/docs/RcsStirrupsData.md)

### Properties
- `Diameter` (double, get/set) — (No description provided in vendor docs for RcsStirrupsData.diameter.)
- `MaterialName` (string, get/set) — (No description provided in vendor docs for RcsStirrupsData.materialName.)
- `DiameterOfMandrel` (double, get/set) — (No description provided in vendor docs for RcsStirrupsData.diameterOfMandrel.)
- `IsClosed` (bool, get/set) — (No description provided in vendor docs for RcsStirrupsData.isClosed.)
- `ShearCheck` (bool, get/set) — (No description provided in vendor docs for RcsStirrupsData.shearCheck.)
- `TorsionCheck` (bool, get/set) — (No description provided in vendor docs for RcsStirrupsData.torsionCheck.)
- `Distance` (double, get/set) — (No description provided in vendor docs for RcsStirrupsData.distance.)
- `Geometry` (PolyLine2D, get/set) — (No description provided in vendor docs for RcsStirrupsData.geometry.)

## RcsTendonBarData (class)

(No description provided in vendor docs for IdeaStatiCa.RcsApi.Model.RcsTendonBarData.)

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/25.1.5.1491/src/api-sdks/rcs-api/clients/csharp/docs/RcsTendonBarData.md)

### Properties
- `Point` (Point2D, get/set) — (No description provided in vendor docs for RcsTendonBarData.point.)
- `MaterialName` (string, get/set) — (No description provided in vendor docs for RcsTendonBarData.materialName.)
- `NumStrandsInTendon` (int, get/set) — (No description provided in vendor docs for RcsTendonBarData.numStrandsInTendon.)
- `PrestressingOrder` (int, get/set) — (No description provided in vendor docs for RcsTendonBarData.prestressingOrder.)
- `TendonDuct` (RcsTendonDuctData, get/set) — (No description provided in vendor docs for RcsTendonBarData.tendonDuct.)

## RcsTendonDuctData (class)

(No description provided in vendor docs for IdeaStatiCa.RcsApi.Model.RcsTendonDuctData.)

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/25.1.5.1491/src/api-sdks/rcs-api/clients/csharp/docs/RcsTendonDuctData.md)

### Properties
- `Point` (Point2D, get/set) — (No description provided in vendor docs for RcsTendonDuctData.point.)
- `Diameter` (double, get/set) — (No description provided in vendor docs for RcsTendonDuctData.diameter.)
- `IsDebondingTube` (bool, get/set) — (No description provided in vendor docs for RcsTendonDuctData.isDebondingTube.)
- `MaterialDuct` (MaterialDuct, get/set) — (No description provided in vendor docs for RcsTendonDuctData.materialDuct.)

## ReferenceElement (class)

(No description provided in vendor docs for IdeaStatiCa.RcsApi.Model.ReferenceElement.)

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/25.1.5.1491/src/api-sdks/rcs-api/clients/csharp/docs/ReferenceElement.md)

### Properties
- `TypeName` (string, get/set) — (No description provided in vendor docs for ReferenceElement.typeName.)
- `Id` (int, get/set) — (No description provided in vendor docs for ReferenceElement.id.)
- `Element` (OpenElementId, get/set) — (No description provided in vendor docs for ReferenceElement.element.)

## Region2D (class)

(No description provided in vendor docs for IdeaStatiCa.RcsApi.Model.Region2D.)

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/25.1.5.1491/src/api-sdks/rcs-api/clients/csharp/docs/Region2D.md)

### Properties
- `Outline` (PolyLine2D, get/set) — (No description provided in vendor docs for Region2D.outline.)
- `Openings` (List<PolyLine2D>, get/set) — (No description provided in vendor docs for Region2D.openings.)

## ReinforcedBar (class)

(No description provided in vendor docs for IdeaStatiCa.RcsApi.Model.ReinforcedBar.)

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/25.1.5.1491/src/api-sdks/rcs-api/clients/csharp/docs/ReinforcedBar.md)

### Properties
- `Point` (Point2D, get/set) — (No description provided in vendor docs for ReinforcedBar.point.)
- `Diameter` (double, get/set) — (No description provided in vendor docs for ReinforcedBar.diameter.)
- `Material` (ReferenceElement, get/set) — (No description provided in vendor docs for ReinforcedBar.material.)

## ReinforcedCrossSection (class)

(No description provided in vendor docs for IdeaStatiCa.RcsApi.Model.ReinforcedCrossSection.)

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/25.1.5.1491/src/api-sdks/rcs-api/clients/csharp/docs/ReinforcedCrossSection.md)

### Properties
- `Name` (string, get/set) — (No description provided in vendor docs for ReinforcedCrossSection.name.)
- `CrossSection` (ReferenceElement, get/set) — (No description provided in vendor docs for ReinforcedCrossSection.crossSection.)
- `Bars` (List<ReinforcedBar>, get/set) — (No description provided in vendor docs for ReinforcedCrossSection.bars.)
- `Stirrups` (List<Stirrup>, get/set) — (No description provided in vendor docs for ReinforcedCrossSection.stirrups.)
- `TendonBars` (List<TendonBar>, get/set) — (No description provided in vendor docs for ReinforcedCrossSection.tendonBars.)
- `TendonDucts` (List<TendonDuct>, get/set) — (No description provided in vendor docs for ReinforcedCrossSection.tendonDucts.)
- `Id` (int, get/set) — (No description provided in vendor docs for ReinforcedCrossSection.id.)

## ReinforcedCrossSectionData (class)

(No description provided in vendor docs for IdeaStatiCa.RcsApi.Model.ReinforcedCrossSectionData.)

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/25.1.5.1491/src/api-sdks/rcs-api/clients/csharp/docs/ReinforcedCrossSectionData.md)

### Properties
- `Name` (string, get/set) — (No description provided in vendor docs for ReinforcedCrossSectionData.name.)
- `CrossSection` (RcsCrossSectionData, get/set) — (No description provided in vendor docs for ReinforcedCrossSectionData.crossSection.)
- `Bars` (List<RcsReinforcedBarData>, get/set) — (No description provided in vendor docs for ReinforcedCrossSectionData.bars.)
- `Stirrups` (List<RcsStirrupsData>, get/set) — (No description provided in vendor docs for ReinforcedCrossSectionData.stirrups.)
- `TendonBars` (List<RcsTendonBarData>, get/set) — (No description provided in vendor docs for ReinforcedCrossSectionData.tendonBars.)
- `TendonDucts` (List<RcsTendonDuctData>, get/set) — (No description provided in vendor docs for ReinforcedCrossSectionData.tendonDucts.)

## ResultOfInternalForces (class)

(No description provided in vendor docs for IdeaStatiCa.RcsApi.Model.ResultOfInternalForces.)

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/25.1.5.1491/src/api-sdks/rcs-api/clients/csharp/docs/ResultOfInternalForces.md)

### Properties
- `N` (double, get/set) — (No description provided in vendor docs for ResultOfInternalForces.n.)
- `Qy` (double, get/set) — (No description provided in vendor docs for ResultOfInternalForces.qy.)
- `Qz` (double, get/set) — (No description provided in vendor docs for ResultOfInternalForces.qz.)
- `Mx` (double, get/set) — (No description provided in vendor docs for ResultOfInternalForces.mx.)
- `My` (double, get/set) — (No description provided in vendor docs for ResultOfInternalForces.my.)
- `Mz` (double, get/set) — (No description provided in vendor docs for ResultOfInternalForces.mz.)
- `Loading` (ResultOfLoading, get/set) — (No description provided in vendor docs for ResultOfInternalForces.loading.)

## ResultOfLoading (class)

(No description provided in vendor docs for IdeaStatiCa.RcsApi.Model.ResultOfLoading.)

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/25.1.5.1491/src/api-sdks/rcs-api/clients/csharp/docs/ResultOfLoading.md)

### Properties
- `Items` (List<ResultOfLoadingItem>, get/set) — (No description provided in vendor docs for ResultOfLoading.items.)
- `LoadingType` (LoadingType, get/set) — (No description provided in vendor docs for ResultOfLoading.loadingType.)
- `Id` (int, get/set) — (No description provided in vendor docs for ResultOfLoading.id.)

## ResultOfLoadingItem (class)

(No description provided in vendor docs for IdeaStatiCa.RcsApi.Model.ResultOfLoadingItem.)

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/25.1.5.1491/src/api-sdks/rcs-api/clients/csharp/docs/ResultOfLoadingItem.md)

### Properties
- `Loading` (Loading, get/set) — (No description provided in vendor docs for ResultOfLoadingItem.loading.)
- `Coefficient` (double, get/set) — (No description provided in vendor docs for ResultOfLoadingItem.coefficient.)

## SectionConcreteCheckResult (class)

(No description provided in vendor docs for IdeaStatiCa.RcsApi.Model.SectionConcreteCheckResult.)

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/25.1.5.1491/src/api-sdks/rcs-api/clients/csharp/docs/SectionConcreteCheckResult.md)

### Properties
- `SectionId` (int, get/set) — (No description provided in vendor docs for SectionConcreteCheckResult.sectionId.)
- `ExtremeResults` (List<ConcreteCheckResults>, get/set) — (No description provided in vendor docs for SectionConcreteCheckResult.extremeResults.)

## Segment2D (class)

(No description provided in vendor docs for IdeaStatiCa.RcsApi.Model.Segment2D.)

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/25.1.5.1491/src/api-sdks/rcs-api/clients/csharp/docs/Segment2D.md)

### Properties
- `EndPoint` (Point2D, get/set) — (No description provided in vendor docs for Segment2D.endPoint.)

## Stirrup (class)

(No description provided in vendor docs for IdeaStatiCa.RcsApi.Model.Stirrup.)

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/25.1.5.1491/src/api-sdks/rcs-api/clients/csharp/docs/Stirrup.md)

### Properties
- `Geometry` (PolyLine2D, get/set) — (No description provided in vendor docs for Stirrup.geometry.)
- `Diameter` (double, get/set) — (No description provided in vendor docs for Stirrup.diameter.)
- `Material` (ReferenceElement, get/set) — (No description provided in vendor docs for Stirrup.material.)
- `AnchorageLenght` (double, get/set) — (No description provided in vendor docs for Stirrup.anchorageLenght.)
- `DiameterOfMandrel` (double, get/set) — (No description provided in vendor docs for Stirrup.diameterOfMandrel.)
- `IsClosed` (bool, get/set) — (No description provided in vendor docs for Stirrup.isClosed.)
- `Distance` (double, get/set) — (No description provided in vendor docs for Stirrup.distance.)
- `ShearCheck` (bool, get/set) — (No description provided in vendor docs for Stirrup.shearCheck.)
- `TorsionCheck` (bool, get/set) — (No description provided in vendor docs for Stirrup.torsionCheck.)

## TendonBar (class)

(No description provided in vendor docs for IdeaStatiCa.RcsApi.Model.TendonBar.)

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/25.1.5.1491/src/api-sdks/rcs-api/clients/csharp/docs/TendonBar.md)

### Properties
- `Id` (int, get/set) — (No description provided in vendor docs for TendonBar.id.)
- `TendonType` (TendonBarType, get/set) — (No description provided in vendor docs for TendonBar.tendonType.)
- `Point` (Point2D, get/set) — (No description provided in vendor docs for TendonBar.point.)
- `Material` (ReferenceElement, get/set) — (No description provided in vendor docs for TendonBar.material.)
- `PrestressingOrder` (int, get/set) — (No description provided in vendor docs for TendonBar.prestressingOrder.)
- `NumStrandsInTendon` (int, get/set) — (No description provided in vendor docs for TendonBar.numStrandsInTendon.)
- `PrestressReinforcementType` (FatigueTypeOfPrestressingSteel, get/set) — (No description provided in vendor docs for TendonBar.prestressReinforcementType.)
- `Phase` (int, get/set) — (No description provided in vendor docs for TendonBar.phase.)
- `TendonDuct` (TendonDuct, get/set) — (No description provided in vendor docs for TendonBar.tendonDuct.)

## TendonBarType (enum)

(No description provided in vendor docs for IdeaStatiCa.RcsApi.Model.TendonBarType.)

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/25.1.5.1491/src/api-sdks/rcs-api/clients/csharp/docs/TendonBarType.md)

### Values
- `posttensioned` = `posttensioned`
- `pretensioned` = `pretensioned`

## TendonDuct (class)

(No description provided in vendor docs for IdeaStatiCa.RcsApi.Model.TendonDuct.)

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/25.1.5.1491/src/api-sdks/rcs-api/clients/csharp/docs/TendonDuct.md)

### Properties
- `Id` (int, get/set) — (No description provided in vendor docs for TendonDuct.id.)
- `Point` (Point2D, get/set) — (No description provided in vendor docs for TendonDuct.point.)
- `MaterialDuct` (MaterialDuct, get/set) — (No description provided in vendor docs for TendonDuct.materialDuct.)
- `IsDebondingTube` (bool, get/set) — (No description provided in vendor docs for TendonDuct.isDebondingTube.)
- `Diameter` (double, get/set) — (No description provided in vendor docs for TendonDuct.diameter.)

