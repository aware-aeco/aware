---
name: idea-statica-ideastatica-connectionapi-model
description: This skill encodes the idea-statica 26.0 surface of the IdeaStatiCa.ConnectionApi.Model namespace — 100 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: AnchorGrid, AnchorType, BaseTemplateConversion, BeamData, BendData, BoltGrid, BoltShearType, BucklingRes, and 92 more types.
---

# IdeaStatiCa.ConnectionApi.Model

Auto-generated from vendor docs for idea-statica 26.0. 100 types in this namespace.

## AnchorGrid (class)

(No description provided in vendor docs for IdeaStatiCa.ConnectionApi.Model.AnchorGrid.)

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/AnchorGrid.md)

### Properties
- `ShearInThread` (bool, get/set) — (No description provided in vendor docs for AnchorGrid.shearInThread.)
- `ConcreteBlock` (ConcreteBlock, get/set) — (No description provided in vendor docs for AnchorGrid.concreteBlock.)
- `AnchorType` (AnchorType, get/set) — (No description provided in vendor docs for AnchorGrid.anchorType.)
- `AnchorInstallationProcess` (InstallationProcessTypeEnum, get/set) — (No description provided in vendor docs for AnchorGrid.anchorInstallationProcess.)
- `WasherSize` (double, get/set) — (No description provided in vendor docs for AnchorGrid.washerSize.)
- `AnchoringLength` (double, get/set) — (No description provided in vendor docs for AnchorGrid.anchoringLength.)
- `HookLength` (double, get/set) — (No description provided in vendor docs for AnchorGrid.hookLength.)
- `HeadedStudHeadDiameter` (double, get/set) — (No description provided in vendor docs for AnchorGrid.headedStudHeadDiameter.)
- `ReinforcementMandrelDiameter` (double, get/set) — (No description provided in vendor docs for AnchorGrid.reinforcementMandrelDiameter.)
- `BoltAssembly` (ReferenceElement, get/set) — (No description provided in vendor docs for AnchorGrid.boltAssembly.)
- `Origin` (Point3D, get/set) — (No description provided in vendor docs for AnchorGrid.origin.)
- `AxisX` (Vector3D, get/set) — (No description provided in vendor docs for AnchorGrid.axisX.)
- `AxisY` (Vector3D, get/set) — (No description provided in vendor docs for AnchorGrid.axisY.)
- `AxisZ` (Vector3D, get/set) — (No description provided in vendor docs for AnchorGrid.axisZ.)
- `Positions` (List<Point3D>, get/set) — (No description provided in vendor docs for AnchorGrid.positions.)
- `ConnectedParts` (List<ReferenceElement>, get/set) — (No description provided in vendor docs for AnchorGrid.connectedParts.)
- `Name` (string, get/set) — (No description provided in vendor docs for AnchorGrid.name.)
- `Length` (double, get/set) — (No description provided in vendor docs for AnchorGrid.length.)
- `Id` (int, get/set) — (No description provided in vendor docs for AnchorGrid.id.)

## AnchorType (enum)

(No description provided in vendor docs for IdeaStatiCa.ConnectionApi.Model.AnchorType.)

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/AnchorType.md)

### Values
- `straight` = `straight`
- `washerPlateCircular` = `washerPlateCircular`
- `washerPlateRectangular` = `washerPlateRectangular`
- `hook` = `hook`
- `headedStud` = `headedStud`
- `reinforcement` = `reinforcement`
- `generalAnchor` = `generalAnchor`

## BaseTemplateConversion (class)

(No description provided in vendor docs for IdeaStatiCa.ConnectionApi.Model.BaseTemplateConversion.)

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/BaseTemplateConversion.md)

### Properties
- `OriginalValue` (string, get/set) — (No description provided in vendor docs for BaseTemplateConversion.originalValue.)
- `OriginalTemplateId` (string, get/set) — (No description provided in vendor docs for BaseTemplateConversion.originalTemplateId.)
- `NewValue` (string, get/set) — (No description provided in vendor docs for BaseTemplateConversion.newValue.)
- `Description` (string, get/set) — (No description provided in vendor docs for BaseTemplateConversion.description.)
- `NewTemplateId` (string, get/set) — (No description provided in vendor docs for BaseTemplateConversion.newTemplateId.)

## BeamData (class)

(No description provided in vendor docs for IdeaStatiCa.ConnectionApi.Model.BeamData.)

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/BeamData.md)

### Properties
- `Name` (string, get/set) — (No description provided in vendor docs for BeamData.name.)
- `Plates` (List<PlateData>, get/set) — (No description provided in vendor docs for BeamData.plates.)
- `CrossSectionType` (string, get/set) — (No description provided in vendor docs for BeamData.crossSectionType.)
- `MprlName` (string, get/set) — (No description provided in vendor docs for BeamData.mprlName.)
- `OriginalModelId` (string, get/set) — (No description provided in vendor docs for BeamData.originalModelId.)
- `Cuts` (List<CutData>, get/set) — (No description provided in vendor docs for BeamData.cuts.)
- `IsAdded` (bool, get/set) — (No description provided in vendor docs for BeamData.isAdded.)
- `AddedMemberLength` (double, get/set) — (No description provided in vendor docs for BeamData.addedMemberLength.)
- `IsNegativeObject` (bool, get/set) — (No description provided in vendor docs for BeamData.isNegativeObject.)
- `AddedMember` (ReferenceElement, get/set) — (No description provided in vendor docs for BeamData.addedMember.)
- `MirrorY` (bool, get/set) — (No description provided in vendor docs for BeamData.mirrorY.)
- `RefLineInCenterOfGravity` (bool, get/set) — (No description provided in vendor docs for BeamData.refLineInCenterOfGravity.)
- `IsBearingMember` (bool, get/set) — (No description provided in vendor docs for BeamData.isBearingMember.)
- `AutoAddCutByWorkplane` (bool, get/set) — (No description provided in vendor docs for BeamData.autoAddCutByWorkplane.)
- `Id` (int, get/set) — (No description provided in vendor docs for BeamData.id.)

## BendData (class)

(No description provided in vendor docs for IdeaStatiCa.ConnectionApi.Model.BendData.)

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/BendData.md)

### Properties
- `Plate1Id` (int, get/set) — (No description provided in vendor docs for BendData.plate1Id.)
- `Plate2Id` (int, get/set) — (No description provided in vendor docs for BendData.plate2Id.)
- `Radius` (double, get/set) — (No description provided in vendor docs for BendData.radius.)
- `Point1OfSideBoundary1` (Point3D, get/set) — (No description provided in vendor docs for BendData.point1OfSideBoundary1.)
- `Point2OfSideBoundary1` (Point3D, get/set) — (No description provided in vendor docs for BendData.point2OfSideBoundary1.)
- `EndFaceNormal1` (Vector3D, get/set) — (No description provided in vendor docs for BendData.endFaceNormal1.)
- `Point1OfSideBoundary2` (Point3D, get/set) — (No description provided in vendor docs for BendData.point1OfSideBoundary2.)
- `Point2OfSideBoundary2` (Point3D, get/set) — (No description provided in vendor docs for BendData.point2OfSideBoundary2.)

## BoltGrid (class)

(No description provided in vendor docs for IdeaStatiCa.ConnectionApi.Model.BoltGrid.)

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/BoltGrid.md)

### Properties
- `ShearInThread` (bool, get/set) — (No description provided in vendor docs for BoltGrid.shearInThread.)
- `BoltInteraction` (BoltShearType, get/set) — (No description provided in vendor docs for BoltGrid.boltInteraction.)
- `BoltAssembly` (ReferenceElement, get/set) — (No description provided in vendor docs for BoltGrid.boltAssembly.)
- `Origin` (Point3D, get/set) — (No description provided in vendor docs for BoltGrid.origin.)
- `AxisX` (Vector3D, get/set) — (No description provided in vendor docs for BoltGrid.axisX.)
- `AxisY` (Vector3D, get/set) — (No description provided in vendor docs for BoltGrid.axisY.)
- `AxisZ` (Vector3D, get/set) — (No description provided in vendor docs for BoltGrid.axisZ.)
- `Positions` (List<Point3D>, get/set) — (No description provided in vendor docs for BoltGrid.positions.)
- `ConnectedParts` (List<ReferenceElement>, get/set) — (No description provided in vendor docs for BoltGrid.connectedParts.)
- `Name` (string, get/set) — (No description provided in vendor docs for BoltGrid.name.)
- `Length` (double, get/set) — (No description provided in vendor docs for BoltGrid.length.)
- `Id` (int, get/set) — (No description provided in vendor docs for BoltGrid.id.)

## BoltShearType (enum)

(No description provided in vendor docs for IdeaStatiCa.ConnectionApi.Model.BoltShearType.)

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/BoltShearType.md)

### Values
- `bearing` = `bearing`
- `interaction` = `interaction`
- `friction` = `friction`

## BucklingRes (class)

(No description provided in vendor docs for IdeaStatiCa.ConnectionApi.Model.BucklingRes.)

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/BucklingRes.md)

### Properties
- `LoadCaseId` (int, get/set) — (No description provided in vendor docs for BucklingRes.loadCaseId.)
- `Shape` (int, get/set) — (No description provided in vendor docs for BucklingRes.shape.)
- `Factor` (double, get/set) — (No description provided in vendor docs for BucklingRes.factor.)

## CheckResAnchor (class)

(No description provided in vendor docs for IdeaStatiCa.ConnectionApi.Model.CheckResAnchor.)

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/CheckResAnchor.md)

### Properties
- `Name` (string, get/set) — (No description provided in vendor docs for CheckResAnchor.name.)
- `UnityCheck` (double, get/set) — (No description provided in vendor docs for CheckResAnchor.unityCheck.)
- `CheckStatus` (bool, get/set) — (No description provided in vendor docs for CheckResAnchor.checkStatus.)

## CheckResBolt (class)

(No description provided in vendor docs for IdeaStatiCa.ConnectionApi.Model.CheckResBolt.)

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/CheckResBolt.md)

### Properties
- `Name` (string, get/set) — (No description provided in vendor docs for CheckResBolt.name.)
- `UnityCheck` (double, get/set) — (No description provided in vendor docs for CheckResBolt.unityCheck.)
- `CheckStatus` (bool, get/set) — (No description provided in vendor docs for CheckResBolt.checkStatus.)

## CheckResConcreteBlock (class)

(No description provided in vendor docs for IdeaStatiCa.ConnectionApi.Model.CheckResConcreteBlock.)

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/CheckResConcreteBlock.md)

### Properties
- `Name` (string, get/set) — (No description provided in vendor docs for CheckResConcreteBlock.name.)
- `UnityCheck` (double, get/set) — (No description provided in vendor docs for CheckResConcreteBlock.unityCheck.)
- `CheckStatus` (bool, get/set) — (No description provided in vendor docs for CheckResConcreteBlock.checkStatus.)
- `LoadCaseId` (int, get/set) — (No description provided in vendor docs for CheckResConcreteBlock.loadCaseId.)

## CheckResPlate (class)

(No description provided in vendor docs for IdeaStatiCa.ConnectionApi.Model.CheckResPlate.)

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/CheckResPlate.md)

### Properties
- `Name` (string, get/set) — (No description provided in vendor docs for CheckResPlate.name.)
- `CheckStatus` (bool, get/set) — (No description provided in vendor docs for CheckResPlate.checkStatus.)
- `LoadCaseId` (int, get/set) — (No description provided in vendor docs for CheckResPlate.loadCaseId.)
- `MaxStrain` (double, get/set) — (No description provided in vendor docs for CheckResPlate.maxStrain.)
- `MaxStress` (double, get/set) — (No description provided in vendor docs for CheckResPlate.maxStress.)
- `Items` (List<int>, get/set) — (No description provided in vendor docs for CheckResPlate.items.)

## CheckResSummary (class)

(No description provided in vendor docs for IdeaStatiCa.ConnectionApi.Model.CheckResSummary.)

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/CheckResSummary.md)

### Properties
- `CheckValue` (double, get/set) — (No description provided in vendor docs for CheckResSummary.checkValue.)
- `CheckStatus` (bool, get/set) — (No description provided in vendor docs for CheckResSummary.checkStatus.)
- `LoadCaseId` (int, get/set) — (No description provided in vendor docs for CheckResSummary.loadCaseId.)
- `Name` (string, get/set) — (No description provided in vendor docs for CheckResSummary.name.)
- `UnityCheckMessage` (string, get/set) — (No description provided in vendor docs for CheckResSummary.unityCheckMessage.)
- `Skipped` (bool, get/set) — (No description provided in vendor docs for CheckResSummary.skipped.)

## CheckResWeld (class)

(No description provided in vendor docs for IdeaStatiCa.ConnectionApi.Model.CheckResWeld.)

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/CheckResWeld.md)

### Properties
- `Name` (string, get/set) — (No description provided in vendor docs for CheckResWeld.name.)
- `Id` (int, get/set) — (No description provided in vendor docs for CheckResWeld.id.)
- `UnityCheck` (double, get/set) — (No description provided in vendor docs for CheckResWeld.unityCheck.)
- `CheckStatus` (bool, get/set) — (No description provided in vendor docs for CheckResWeld.checkStatus.)
- `LoadCaseId` (int, get/set) — (No description provided in vendor docs for CheckResWeld.loadCaseId.)
- `Items` (List<int>, get/set) — (No description provided in vendor docs for CheckResWeld.items.)

## ConAlignedPlate (class)

(No description provided in vendor docs for IdeaStatiCa.ConnectionApi.Model.ConAlignedPlate.)

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/ConAlignedPlate.md)

### Properties
- `PlateSide` (ConAlignedPlateSideCodeEnum, get/set) — (No description provided in vendor docs for ConAlignedPlate.plateSide.)
- `MemberId` (int, get/set) — (No description provided in vendor docs for ConAlignedPlate.memberId.)
- `PartType` (ConMemberPlatePartTypeEnum, get/set) — (No description provided in vendor docs for ConAlignedPlate.partType.)

## ConAlignedPlateSideCodeEnum (enum)

(No description provided in vendor docs for IdeaStatiCa.ConnectionApi.Model.ConAlignedPlateSideCodeEnum.)

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/ConAlignedPlateSideCodeEnum.md)

### Values
- `default` = `default`
- `lowerSide` = `lowerSide`
- `upperSide` = `upperSide`
- `bothSides` = `bothSides`
- `center` = `center`

## ConAnalysisTypeEnum (enum)

(No description provided in vendor docs for IdeaStatiCa.ConnectionApi.Model.ConAnalysisTypeEnum.)

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/ConAnalysisTypeEnum.md)

### Values
- `stress_Strain` = `stress_Strain`
- `stiffness` = `stiffness`
- `capacity_Design` = `capacity_Design`
- `fatigues` = `fatigues`
- `total_Design` = `total_Design`
- `horizontalTying` = `horizontalTying`
- `capacityLoadLevels` = `capacityLoadLevels`
- `fireRestance` = `fireRestance`
- `buckling` = `buckling`

## ConConnection (class)

(No description provided in vendor docs for IdeaStatiCa.ConnectionApi.Model.ConConnection.)

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/ConConnection.md)

### Properties
- `Id` (int, get/set) — (No description provided in vendor docs for ConConnection.id.)
- `Identifier` (string, get/set) — (No description provided in vendor docs for ConConnection.identifier.)
- `Name` (string, get/set) — (No description provided in vendor docs for ConConnection.name.)
- `Description` (string, get/set) — (No description provided in vendor docs for ConConnection.description.)
- `AnalysisType` (ConAnalysisTypeEnum, get/set) — (No description provided in vendor docs for ConConnection.analysisType.)
- `IsCalculated` (bool, get) — (No description provided in vendor docs for ConConnection.isCalculated.)
- `IncludeBuckling` (bool, get/set) — (No description provided in vendor docs for ConConnection.includeBuckling.)

## ConConnectionLibrarySearchParameters (class)

(No description provided in vendor docs for IdeaStatiCa.ConnectionApi.Model.ConConnectionLibrarySearchParameters.)

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/ConConnectionLibrarySearchParameters.md)

### Properties
- `Members` (List<int>, get/set) — (No description provided in vendor docs for ConConnectionLibrarySearchParameters.members.)
- `InPredefinedSet` (bool, get/set) — (No description provided in vendor docs for ConConnectionLibrarySearchParameters.inPredefinedSet.)
- `InCompanySet` (bool, get/set) — (No description provided in vendor docs for ConConnectionLibrarySearchParameters.inCompanySet.)
- `InPersonalSet` (bool, get/set) — (No description provided in vendor docs for ConConnectionLibrarySearchParameters.inPersonalSet.)
- `HasBolts` (SearchOption, get/set) — (No description provided in vendor docs for ConConnectionLibrarySearchParameters.hasBolts.)
- `HasWelds` (SearchOption, get/set) — (No description provided in vendor docs for ConConnectionLibrarySearchParameters.hasWelds.)
- `HasAnchor` (SearchOption, get/set) — (No description provided in vendor docs for ConConnectionLibrarySearchParameters.hasAnchor.)
- `HasClipAngles` (SearchOption, get/set) — (No description provided in vendor docs for ConConnectionLibrarySearchParameters.hasClipAngles.)
- `IsMoment` (SearchOption, get/set) — (No description provided in vendor docs for ConConnectionLibrarySearchParameters.isMoment.)
- `IsShear` (SearchOption, get/set) — (No description provided in vendor docs for ConConnectionLibrarySearchParameters.isShear.)
- `IsTruss` (SearchOption, get/set) — (No description provided in vendor docs for ConConnectionLibrarySearchParameters.isTruss.)
- `IsParametric` (SearchOption, get/set) — (No description provided in vendor docs for ConConnectionLibrarySearchParameters.isParametric.)

## ConConnectionTemplate (class)

(No description provided in vendor docs for IdeaStatiCa.ConnectionApi.Model.ConConnectionTemplate.)

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/ConConnectionTemplate.md)

### Properties
- `LibraryTemplateId` (Guid, get/set) — (No description provided in vendor docs for ConConnectionTemplate.libraryTemplateId.)
- `TemplateId` (int, get/set) — (No description provided in vendor docs for ConConnectionTemplate.templateId.)
- `Members` (List<ConItem>, get/set) — (No description provided in vendor docs for ConConnectionTemplate.members.)
- `Operations` (List<ConItem>, get/set) — (No description provided in vendor docs for ConConnectionTemplate.operations.)
- `ParameterKeys` (List<string>, get/set) — (No description provided in vendor docs for ConConnectionTemplate.parameterKeys.)
- `CommonProperties` (ConOperationCommonProperties, get/set) — (No description provided in vendor docs for ConConnectionTemplate.commonProperties.)

## ConConversionSettings (class)

(No description provided in vendor docs for IdeaStatiCa.ConnectionApi.Model.ConConversionSettings.)

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/ConConversionSettings.md)

### Properties
- `TargetDesignCode` (CountryCode, get/set) — (No description provided in vendor docs for ConConversionSettings.targetDesignCode.)
- `Concrete` (List<ConversionMapping>, get/set) — (No description provided in vendor docs for ConConversionSettings.concrete.)
- `CrossSections` (List<ConversionMapping>, get/set) — (No description provided in vendor docs for ConConversionSettings.crossSections.)
- `Fasteners` (List<ConversionMapping>, get/set) — (No description provided in vendor docs for ConConversionSettings.fasteners.)
- `Steel` (List<ConversionMapping>, get/set) — (No description provided in vendor docs for ConConversionSettings.steel.)
- `Welds` (List<ConversionMapping>, get/set) — (No description provided in vendor docs for ConConversionSettings.welds.)
- `BoltGrade` (List<ConversionMapping>, get/set) — (No description provided in vendor docs for ConConversionSettings.boltGrade.)

## ConDesignItem (class)

(No description provided in vendor docs for IdeaStatiCa.ConnectionApi.Model.ConDesignItem.)

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/ConDesignItem.md)

### Properties
- `Version` (string, get/set) — (No description provided in vendor docs for ConDesignItem.version.)
- `Name` (string, get/set) — (No description provided in vendor docs for ConDesignItem.name.)
- `DesignCode` (string, get/set) — (No description provided in vendor docs for ConDesignItem.designCode.)
- `ConDesignSetId` (Guid, get/set) — (No description provided in vendor docs for ConDesignItem.conDesignSetId.)
- `ConDesignItemId` (Guid, get/set) — (No description provided in vendor docs for ConDesignItem.conDesignItemId.)

## ConDesignSet (class)

(No description provided in vendor docs for IdeaStatiCa.ConnectionApi.Model.ConDesignSet.)

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/ConDesignSet.md)

### Properties
- `Id` (Guid, get/set) — (No description provided in vendor docs for ConDesignSet.id.)
- `Name` (string, get/set) — (No description provided in vendor docs for ConDesignSet.name.)
- `Description` (string, get/set) — (No description provided in vendor docs for ConDesignSet.description.)
- `OwnerId` (string, get/set) — (No description provided in vendor docs for ConDesignSet.ownerId.)
- `Type` (string, get/set) — (No description provided in vendor docs for ConDesignSet.type.)

## ConDesignSetType (enum)

(No description provided in vendor docs for IdeaStatiCa.ConnectionApi.Model.ConDesignSetType.)

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/ConDesignSetType.md)

### Values
- `private` = `private`
- `company` = `company`

## ConItem (class)

(No description provided in vendor docs for IdeaStatiCa.ConnectionApi.Model.ConItem.)

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/ConItem.md)

### Properties
- `Id` (int, get/set) — (No description provided in vendor docs for ConItem.id.)
- `Name` (string, get/set) — (No description provided in vendor docs for ConItem.name.)
- `Active` (bool, get/set) — (No description provided in vendor docs for ConItem.active.)

## ConLoadEffect (class)

(No description provided in vendor docs for IdeaStatiCa.ConnectionApi.Model.ConLoadEffect.)

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/ConLoadEffect.md)

### Properties
- `IsPercentage` (bool, get/set) — (No description provided in vendor docs for ConLoadEffect.isPercentage.)
- `MemberLoadings` (List<ConLoadEffectMemberLoad>, get/set) — (No description provided in vendor docs for ConLoadEffect.memberLoadings.)
- `Id` (int, get/set) — (No description provided in vendor docs for ConLoadEffect.id.)
- `Name` (string, get/set) — (No description provided in vendor docs for ConLoadEffect.name.)
- `Active` (bool, get/set) — (No description provided in vendor docs for ConLoadEffect.active.)

## ConLoadEffectMemberLoad (class)

(No description provided in vendor docs for IdeaStatiCa.ConnectionApi.Model.ConLoadEffectMemberLoad.)

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/ConLoadEffectMemberLoad.md)

### Properties
- `MemberId` (int, get/set) — (No description provided in vendor docs for ConLoadEffectMemberLoad.memberId.)
- `Position` (ConLoadEffectPositionEnum, get/set) — (No description provided in vendor docs for ConLoadEffectMemberLoad.position.)
- `SectionLoad` (ConLoadEffectSectionLoad, get/set) — (No description provided in vendor docs for ConLoadEffectMemberLoad.sectionLoad.)

## ConLoadEffectPositionEnum (enum)

(No description provided in vendor docs for IdeaStatiCa.ConnectionApi.Model.ConLoadEffectPositionEnum.)

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/ConLoadEffectPositionEnum.md)

### Values
- `End` = `End`
- `Begin` = `Begin`

## ConLoadEffectSectionLoad (class)

(No description provided in vendor docs for IdeaStatiCa.ConnectionApi.Model.ConLoadEffectSectionLoad.)

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/ConLoadEffectSectionLoad.md)

### Properties
- `N` (double, get/set) — (No description provided in vendor docs for ConLoadEffectSectionLoad.n.)
- `Vy` (double, get/set) — (No description provided in vendor docs for ConLoadEffectSectionLoad.vy.)
- `Vz` (double, get/set) — (No description provided in vendor docs for ConLoadEffectSectionLoad.vz.)
- `Mx` (double, get/set) — (No description provided in vendor docs for ConLoadEffectSectionLoad.mx.)
- `My` (double, get/set) — (No description provided in vendor docs for ConLoadEffectSectionLoad.my.)
- `Mz` (double, get/set) — (No description provided in vendor docs for ConLoadEffectSectionLoad.mz.)

## ConLoadSettings (class)

(No description provided in vendor docs for IdeaStatiCa.ConnectionApi.Model.ConLoadSettings.)

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/ConLoadSettings.md)

### Properties
- `LoadsInEquilibrium` (bool, get/set) — (No description provided in vendor docs for ConLoadSettings.loadsInEquilibrium.)
- `LoadsInPercentage` (bool, get/set) — (No description provided in vendor docs for ConLoadSettings.loadsInPercentage.)

## ConMember (class)

(No description provided in vendor docs for IdeaStatiCa.ConnectionApi.Model.ConMember.)

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/ConMember.md)

### Properties
- `IsContinuous` (bool, get/set) — (No description provided in vendor docs for ConMember.isContinuous.)
- `CrossSectionId` (int, get/set) — (No description provided in vendor docs for ConMember.crossSectionId.)
- `MirrorY` (bool, get/set) — (No description provided in vendor docs for ConMember.mirrorY.)
- `MirrorZ` (bool, get/set) — (No description provided in vendor docs for ConMember.mirrorZ.)
- `IsBearing` (bool, get/set) — (No description provided in vendor docs for ConMember.isBearing.)
- `Position` (ConMemberPosition, get/set) — (No description provided in vendor docs for ConMember.position.)
- `Model` (ConMemberModel, get/set) — (No description provided in vendor docs for ConMember.model.)
- `StiffnessAnalysis` (ConStiffnessAnalysis, get/set) — (No description provided in vendor docs for ConMember.stiffnessAnalysis.)
- `Id` (int, get/set) — (No description provided in vendor docs for ConMember.id.)
- `Name` (string, get/set) — (No description provided in vendor docs for ConMember.name.)
- `Active` (bool, get/set) — (No description provided in vendor docs for ConMember.active.)

## ConMemberAlignmentTypeEnum (enum)

(No description provided in vendor docs for IdeaStatiCa.ConnectionApi.Model.ConMemberAlignmentTypeEnum.)

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/ConMemberAlignmentTypeEnum.md)

### Values
- `none` = `none`
- `toMemberPlate` = `toMemberPlate`
- `toMemberPlateRotateThenTranslate` = `toMemberPlateRotateThenTranslate`

## ConMemberForcesInEnum (enum)

(No description provided in vendor docs for IdeaStatiCa.ConnectionApi.Model.ConMemberForcesInEnum.)

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/ConMemberForcesInEnum.md)

### Values
- `position` = `position`
- `node` = `node`
- `bolts` = `bolts`
- `selectedMemberFace` = `selectedMemberFace`

## ConMemberModel (class)

(No description provided in vendor docs for IdeaStatiCa.ConnectionApi.Model.ConMemberModel.)

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/ConMemberModel.md)

### Properties
- `ModelType` (string, get/set) — (No description provided in vendor docs for ConMemberModel.modelType.)
- `ForcesIn` (ConMemberForcesInEnum, get/set) — (No description provided in vendor docs for ConMemberModel.forcesIn.)
- `X` (double, get/set) — (No description provided in vendor docs for ConMemberModel.x.)
- `ConnectedMemberId` (int, get/set) — (No description provided in vendor docs for ConMemberModel.connectedMemberId.)

## ConMemberPlacementDefinitionTypeEnum (enum)

(No description provided in vendor docs for IdeaStatiCa.ConnectionApi.Model.ConMemberPlacementDefinitionTypeEnum.)

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/ConMemberPlacementDefinitionTypeEnum.md)

### Values
- `none` = `none`
- `directionVector` = `directionVector`
- `rotationsOfX` = `rotationsOfX`
- `lcs` = `lcs`

## ConMemberPlatePartTypeEnum (enum)

(No description provided in vendor docs for IdeaStatiCa.ConnectionApi.Model.ConMemberPlatePartTypeEnum.)

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/ConMemberPlatePartTypeEnum.md)

### Values
- `notSpecified` = `notSpecified`
- `topFlange` = `topFlange`
- `bottomFlange` = `bottomFlange`
- `bothFlanges` = `bothFlanges`
- `web` = `web`
- `allCssParts` = `allCssParts`
- `basePlate` = `basePlate`
- `endPlate` = `endPlate`
- `plateWidener` = `plateWidener`
- `stiffener` = `stiffener`
- `rib` = `rib`
- `gussetPlate` = `gussetPlate`
- `finPlate` = `finPlate`
- `flange` = `flange`
- `cssArcSegment` = `cssArcSegment`
- `isStub` = `isStub`
- `splice` = `splice`
- `tonguePlate` = `tonguePlate`
- `lidPlate` = `lidPlate`
- `generalPlate` = `generalPlate`
- `doubler` = `doubler`
- `endPlateOnFlanges` = `endPlateOnFlanges`
- `backingPlate` = `backingPlate`
- `insertedPlate` = `insertedPlate`
- `isNegative` = `isNegative`

## ConMemberPosition (class)

(No description provided in vendor docs for IdeaStatiCa.ConnectionApi.Model.ConMemberPosition.)

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/ConMemberPosition.md)

### Properties
- `DefinedBy` (ConMemberPlacementDefinitionTypeEnum, get/set) — (No description provided in vendor docs for ConMemberPosition.definedBy.)
- `AxisX` (Vector3D, get/set) — (No description provided in vendor docs for ConMemberPosition.axisX.)
- `BetaDirection` (double, get/set) — (No description provided in vendor docs for ConMemberPosition.betaDirection.)
- `GamaPitch` (double, get/set) — (No description provided in vendor docs for ConMemberPosition.gamaPitch.)
- `AlphaRotation` (double, get/set) — (No description provided in vendor docs for ConMemberPosition.alphaRotation.)
- `OffsetEx` (double, get/set) — (No description provided in vendor docs for ConMemberPosition.offsetEx.)
- `OffsetEy` (double, get/set) — (No description provided in vendor docs for ConMemberPosition.offsetEy.)
- `OffsetEz` (double, get/set) — (No description provided in vendor docs for ConMemberPosition.offsetEz.)
- `Align` (ConMemberAlignmentTypeEnum, get/set) — (No description provided in vendor docs for ConMemberPosition.align.)
- `AlignedPlate` (ConAlignedPlate, get/set) — (No description provided in vendor docs for ConMemberPosition.alignedPlate.)
- `RelatedPlate` (ConAlignedPlate, get/set) — (No description provided in vendor docs for ConMemberPosition.relatedPlate.)

## ConMprlCrossSection (class)

(No description provided in vendor docs for IdeaStatiCa.ConnectionApi.Model.ConMprlCrossSection.)

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/ConMprlCrossSection.md)

### Properties
- `MaterialName` (string, get/set) — (No description provided in vendor docs for ConMprlCrossSection.materialName.)
- `MprlName` (string, get/set) — (No description provided in vendor docs for ConMprlCrossSection.mprlName.)

## ConMprlElement (class)

(No description provided in vendor docs for IdeaStatiCa.ConnectionApi.Model.ConMprlElement.)

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/ConMprlElement.md)

### Properties
- `MprlName` (string, get/set) — (No description provided in vendor docs for ConMprlElement.mprlName.)

## ConNonConformityIssue (class)

(No description provided in vendor docs for IdeaStatiCa.ConnectionApi.Model.ConNonConformityIssue.)

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/ConNonConformityIssue.md)

### Properties
- `OperationId` (int, get/set) — (No description provided in vendor docs for ConNonConformityIssue.operationId.)
- `Description` (string, get/set) — (No description provided in vendor docs for ConNonConformityIssue.description.)
- `Details` (string, get/set) — (No description provided in vendor docs for ConNonConformityIssue.details.)
- `Severity` (ConNonConformityIssueSeverity, get/set) — (No description provided in vendor docs for ConNonConformityIssue.severity.)

## ConNonConformityIssueSeverity (enum)

(No description provided in vendor docs for IdeaStatiCa.ConnectionApi.Model.ConNonConformityIssueSeverity.)

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/ConNonConformityIssueSeverity.md)

### Values
- `info` = `info`
- `warning` = `warning`
- `error` = `error`
- `terminatedError` = `terminatedError`

## ConOperation (class)

(No description provided in vendor docs for IdeaStatiCa.ConnectionApi.Model.ConOperation.)

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/ConOperation.md)

### Properties
- `IsImported` (bool, get/set) — (No description provided in vendor docs for ConOperation.isImported.)
- `OperationType` (string, get/set) — (No description provided in vendor docs for ConOperation.operationType.)
- `Id` (int, get/set) — (No description provided in vendor docs for ConOperation.id.)
- `Name` (string, get/set) — (No description provided in vendor docs for ConOperation.name.)
- `Active` (bool, get/set) — (No description provided in vendor docs for ConOperation.active.)

## ConOperationCommonProperties (class)

(No description provided in vendor docs for IdeaStatiCa.ConnectionApi.Model.ConOperationCommonProperties.)

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/ConOperationCommonProperties.md)

### Properties
- `WeldMaterialId` (int, get/set) — (No description provided in vendor docs for ConOperationCommonProperties.weldMaterialId.)
- `PlateMaterialId` (int, get/set) — (No description provided in vendor docs for ConOperationCommonProperties.plateMaterialId.)
- `BoltAssemblyId` (int, get/set) — (No description provided in vendor docs for ConOperationCommonProperties.boltAssemblyId.)

## ConProductionCost (class)

(No description provided in vendor docs for IdeaStatiCa.ConnectionApi.Model.ConProductionCost.)

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/ConProductionCost.md)

### Properties
- `TotalEstimatedCost` (double, get/set) — (No description provided in vendor docs for ConProductionCost.totalEstimatedCost.)

## ConProject (class)

(No description provided in vendor docs for IdeaStatiCa.ConnectionApi.Model.ConProject.)

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/ConProject.md)

### Properties
- `ProjectId` (Guid, get/set) — (No description provided in vendor docs for ConProject.projectId.)
- `ProjectInfo` (ConProjectData, get/set) — (No description provided in vendor docs for ConProject.projectInfo.)
- `Connections` (List<ConConnection>, get/set) — (No description provided in vendor docs for ConProject.connections.)

## ConProjectData (class)

(No description provided in vendor docs for IdeaStatiCa.ConnectionApi.Model.ConProjectData.)

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/ConProjectData.md)

### Properties
- `Name` (string, get/set) — (No description provided in vendor docs for ConProjectData.name.)
- `Description` (string, get/set) — (No description provided in vendor docs for ConProjectData.description.)
- `ProjectNumber` (string, get/set) — (No description provided in vendor docs for ConProjectData.projectNumber.)
- `Author` (string, get/set) — (No description provided in vendor docs for ConProjectData.author.)
- `DesignCode` (string, get/set) — (No description provided in vendor docs for ConProjectData.designCode.)
- `Date` (DateTime, get/set) — (No description provided in vendor docs for ConProjectData.date.)

## ConResultSummary (class)

(No description provided in vendor docs for IdeaStatiCa.ConnectionApi.Model.ConResultSummary.)

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/ConResultSummary.md)

### Properties
- `Id` (int, get/set) — (No description provided in vendor docs for ConResultSummary.id.)
- `Passed` (bool, get/set) — (No description provided in vendor docs for ConResultSummary.passed.)
- `ResultSummary` (List<CheckResSummary>, get/set) — (No description provided in vendor docs for ConResultSummary.resultSummary.)

## ConStiffnessAnalysis (class)

(No description provided in vendor docs for IdeaStatiCa.ConnectionApi.Model.ConStiffnessAnalysis.)

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/ConStiffnessAnalysis.md)

### Properties
- `TheoreticalLengthZ` (double, get/set) — (No description provided in vendor docs for ConStiffnessAnalysis.theoreticalLengthZ.)
- `TheoreticalLengthY` (double, get/set) — (No description provided in vendor docs for ConStiffnessAnalysis.theoreticalLengthY.)

## ConTemplateApplyParam (class)

(No description provided in vendor docs for IdeaStatiCa.ConnectionApi.Model.ConTemplateApplyParam.)

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/ConTemplateApplyParam.md)

### Properties
- `ConnectionTemplate` (string, get/set) — (No description provided in vendor docs for ConTemplateApplyParam.connectionTemplate.)
- `Mapping` (TemplateConversions, get/set) — (No description provided in vendor docs for ConTemplateApplyParam.mapping.)

## ConTemplateApplyResult (class)

(No description provided in vendor docs for IdeaStatiCa.ConnectionApi.Model.ConTemplateApplyResult.)

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/ConTemplateApplyResult.md)

### Properties
- `AppliedWithoutIssues` (bool, get/set) — (No description provided in vendor docs for ConTemplateApplyResult.appliedWithoutIssues.)
- `TemplateModel` (ConConnectionTemplate, get/set) — (No description provided in vendor docs for ConTemplateApplyResult.templateModel.)
- `Issues` (List<ConNonConformityIssue>, get/set) — (No description provided in vendor docs for ConTemplateApplyResult.issues.)

## ConTemplateMappingGetParam (class)

(No description provided in vendor docs for IdeaStatiCa.ConnectionApi.Model.ConTemplateMappingGetParam.)

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/ConTemplateMappingGetParam.md)

### Properties
- `Template` (string, get/set) — (No description provided in vendor docs for ConTemplateMappingGetParam.template.)
- `MemberIds` (List<int>, get/set) — (No description provided in vendor docs for ConTemplateMappingGetParam.memberIds.)

## ConTemplatePublishParam (class)

(No description provided in vendor docs for IdeaStatiCa.ConnectionApi.Model.ConTemplatePublishParam.)

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/ConTemplatePublishParam.md)

### Properties
- `Name` (string, get/set) — (No description provided in vendor docs for ConTemplatePublishParam.name.)
- `Author` (string, get/set) — (No description provided in vendor docs for ConTemplatePublishParam.author.)
- `CompanyName` (string, get/set) — (No description provided in vendor docs for ConTemplatePublishParam.companyName.)
- `DesignSetType` (ConDesignSetType, get/set) — (No description provided in vendor docs for ConTemplatePublishParam.designSetType.)

## ConWeldSizingMethodEnum (enum)

(No description provided in vendor docs for IdeaStatiCa.ConnectionApi.Model.ConWeldSizingMethodEnum.)

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/ConWeldSizingMethodEnum.md)

### Values
- `fullStrength` = `fullStrength`
- `minimumDuctility` = `minimumDuctility`
- `overStrengthFactor` = `overStrengthFactor`
- `capacityEstimation` = `capacityEstimation`

## ConcreteBlock (class)

(No description provided in vendor docs for IdeaStatiCa.ConnectionApi.Model.ConcreteBlock.)

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/ConcreteBlock.md)

### Properties
- `Lenght` (double, get/set) — (No description provided in vendor docs for ConcreteBlock.lenght.)
- `Width` (double, get/set) — (No description provided in vendor docs for ConcreteBlock.width.)
- `Height` (double, get/set) — (No description provided in vendor docs for ConcreteBlock.height.)
- `Material` (string, get/set) — (No description provided in vendor docs for ConcreteBlock.material.)

## ConcreteBlockData (class)

(No description provided in vendor docs for IdeaStatiCa.ConnectionApi.Model.ConcreteBlockData.)

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/ConcreteBlockData.md)

### Properties
- `Id` (int, get/set) — (No description provided in vendor docs for ConcreteBlockData.id.)
- `Name` (string, get/set) — (No description provided in vendor docs for ConcreteBlockData.name.)
- `Depth` (double, get/set) — (No description provided in vendor docs for ConcreteBlockData.depth.)
- `Material` (string, get/set) — (No description provided in vendor docs for ConcreteBlockData.material.)
- `Center` (Point3D, get/set) — (No description provided in vendor docs for ConcreteBlockData.center.)
- `OutlinePoints` (List<Point2D>, get/set) — (No description provided in vendor docs for ConcreteBlockData.outlinePoints.)
- `Origin` (Point3D, get/set) — (No description provided in vendor docs for ConcreteBlockData.origin.)
- `AxisX` (Vector3D, get/set) — (No description provided in vendor docs for ConcreteBlockData.axisX.)
- `AxisY` (Vector3D, get/set) — (No description provided in vendor docs for ConcreteBlockData.axisY.)
- `AxisZ` (Vector3D, get/set) — (No description provided in vendor docs for ConcreteBlockData.axisZ.)
- `Region` (string, get/set) — (No description provided in vendor docs for ConcreteBlockData.region.)
- `OriginalModelId` (string, get/set) — (No description provided in vendor docs for ConcreteBlockData.originalModelId.)

## ConnectionCheckRes (class)

(No description provided in vendor docs for IdeaStatiCa.ConnectionApi.Model.ConnectionCheckRes.)

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/ConnectionCheckRes.md)

### Properties
- `CheckResSummary` (List<CheckResSummary>, get/set) — (No description provided in vendor docs for ConnectionCheckRes.checkResSummary.)
- `CheckResPlate` (List<CheckResPlate>, get/set) — (No description provided in vendor docs for ConnectionCheckRes.checkResPlate.)
- `CheckResWeld` (List<CheckResWeld>, get/set) — (No description provided in vendor docs for ConnectionCheckRes.checkResWeld.)
- `CheckResBolt` (List<CheckResBolt>, get/set) — (No description provided in vendor docs for ConnectionCheckRes.checkResBolt.)
- `CheckResAnchor` (List<CheckResAnchor>, get/set) — (No description provided in vendor docs for ConnectionCheckRes.checkResAnchor.)
- `CheckResConcreteBlock` (List<CheckResConcreteBlock>, get/set) — (No description provided in vendor docs for ConnectionCheckRes.checkResConcreteBlock.)
- `BucklingResults` (List<BucklingRes>, get/set) — (No description provided in vendor docs for ConnectionCheckRes.bucklingResults.)
- `Name` (string, get/set) — (No description provided in vendor docs for ConnectionCheckRes.name.)
- `ConnectionID` (Guid, get/set) — (No description provided in vendor docs for ConnectionCheckRes.connectionID.)
- `Id` (int, get/set) — (No description provided in vendor docs for ConnectionCheckRes.id.)
- `Messages` (OpenMessages, get/set) — (No description provided in vendor docs for ConnectionCheckRes.messages.)

## ConnectionData (class)

(No description provided in vendor docs for IdeaStatiCa.ConnectionApi.Model.ConnectionData.)

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/ConnectionData.md)

### Properties
- `ConnectionPoint` (ReferenceElement, get/set) — (No description provided in vendor docs for ConnectionData.connectionPoint.)
- `Beams` (List<BeamData>, get/set) — (No description provided in vendor docs for ConnectionData.beams.)
- `Plates` (List<PlateData>, get/set) — (No description provided in vendor docs for ConnectionData.plates.)
- `FoldedPlates` (List<FoldedPlateData>, get/set) — (No description provided in vendor docs for ConnectionData.foldedPlates.)
- `BoltGrids` (List<BoltGrid>, get/set) — (No description provided in vendor docs for ConnectionData.boltGrids.)
- `AnchorGrids` (List<AnchorGrid>, get/set) — (No description provided in vendor docs for ConnectionData.anchorGrids.)
- `PinGrids` (List<PinGrid>, get/set) — (No description provided in vendor docs for ConnectionData.pinGrids.)
- `Welds` (List<WeldData>, get/set) — (No description provided in vendor docs for ConnectionData.welds.)
- `ConcreteBlocks` (List<ConcreteBlockData>, get/set) — (No description provided in vendor docs for ConnectionData.concreteBlocks.)
- `CutBeamByBeams` (List<CutBeamByBeamData>, get/set) — (No description provided in vendor docs for ConnectionData.cutBeamByBeams.)

## ConversionMapping (class)

(No description provided in vendor docs for IdeaStatiCa.ConnectionApi.Model.ConversionMapping.)

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/ConversionMapping.md)

### Properties
- `SourceValue` (string, get/set) — (No description provided in vendor docs for ConversionMapping.sourceValue.)
- `TargetValue` (string, get/set) — (No description provided in vendor docs for ConversionMapping.targetValue.)

## CountryCode (enum)

(No description provided in vendor docs for IdeaStatiCa.ConnectionApi.Model.CountryCode.)

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/CountryCode.md)

### Values
- `none` = `none`
- `ecen` = `ecen`
- `india` = `india`
- `sia` = `sia`
- `american` = `american`
- `canada` = `canada`
- `australia` = `australia`
- `rus` = `rus`
- `chn` = `chn`
- `hkg` = `hkg`

## CutBeamByBeamData (class)

(No description provided in vendor docs for IdeaStatiCa.ConnectionApi.Model.CutBeamByBeamData.)

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/CutBeamByBeamData.md)

### Properties
- `Name` (string, get/set) — (No description provided in vendor docs for CutBeamByBeamData.name.)
- `ModifiedObject` (ReferenceElement, get/set) — (No description provided in vendor docs for CutBeamByBeamData.modifiedObject.)
- `CuttingObject` (ReferenceElement, get/set) — (No description provided in vendor docs for CutBeamByBeamData.cuttingObject.)
- `IsWeld` (bool, get/set) — (No description provided in vendor docs for CutBeamByBeamData.isWeld.)
- `WebWeld` (WeldDefinition, get/set) — (No description provided in vendor docs for CutBeamByBeamData.webWeld.)
- `FlangeWeld` (WeldDefinition, get/set) — (No description provided in vendor docs for CutBeamByBeamData.flangeWeld.)
- `Offset` (double, get/set) — (No description provided in vendor docs for CutBeamByBeamData.offset.)
- `Method` (CutMethod, get/set) — (No description provided in vendor docs for CutBeamByBeamData.method.)
- `Orientation` (CutOrientation, get/set) — (No description provided in vendor docs for CutBeamByBeamData.orientation.)
- `PlaneOnCuttingObject` (DistanceComparison, get/set) — (No description provided in vendor docs for CutBeamByBeamData.planeOnCuttingObject.)
- `CutPart` (CutPart, get/set) — (No description provided in vendor docs for CutBeamByBeamData.cutPart.)
- `ExtendBeforeCut` (bool, get/set) — (No description provided in vendor docs for CutBeamByBeamData.extendBeforeCut.)

## CutData (class)

(No description provided in vendor docs for IdeaStatiCa.ConnectionApi.Model.CutData.)

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/CutData.md)

### Properties
- `PlanePoint` (Point3D, get/set) — (No description provided in vendor docs for CutData.planePoint.)
- `NormalVector` (Vector3D, get/set) — (No description provided in vendor docs for CutData.normalVector.)
- `Direction` (CutOrientation, get/set) — (No description provided in vendor docs for CutData.direction.)
- `Offset` (double, get/set) — (No description provided in vendor docs for CutData.offset.)

## CutMethod (enum)

(No description provided in vendor docs for IdeaStatiCa.ConnectionApi.Model.CutMethod.)

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/CutMethod.md)

### Values
- `boundingBox` = `boundingBox`
- `surface` = `surface`
- `mitre` = `mitre`
- `surfaceAll` = `surfaceAll`

## CutOrientation (enum)

(No description provided in vendor docs for IdeaStatiCa.ConnectionApi.Model.CutOrientation.)

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/CutOrientation.md)

### Values
- `default` = `default`
- `parallel` = `parallel`
- `perpendicular` = `perpendicular`

## CutPart (enum)

(No description provided in vendor docs for IdeaStatiCa.ConnectionApi.Model.CutPart.)

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/CutPart.md)

### Values
- `begin` = `begin`
- `end` = `end`

## DistanceComparison (enum)

(No description provided in vendor docs for IdeaStatiCa.ConnectionApi.Model.DistanceComparison.)

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/DistanceComparison.md)

### Values
- `closer` = `closer`
- `farther` = `farther`
- `same` = `same`

## DrawData (class)

(No description provided in vendor docs for IdeaStatiCa.ConnectionApi.Model.DrawData.)

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/DrawData.md)

### Properties
- `Groups` (List<IGroup>, get/set) — (No description provided in vendor docs for DrawData.groups.)
- `Vertices` (List<double>, get/set) — (No description provided in vendor docs for DrawData.vertices.)
- `Normals` (List<double>, get/set) — (No description provided in vendor docs for DrawData.normals.)

## FoldedPlateData (class)

(No description provided in vendor docs for IdeaStatiCa.ConnectionApi.Model.FoldedPlateData.)

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/FoldedPlateData.md)

### Properties
- `Plates` (List<PlateData>, get/set) — (No description provided in vendor docs for FoldedPlateData.plates.)
- `Bends` (List<BendData>, get/set) — (No description provided in vendor docs for FoldedPlateData.bends.)

## IGroup (class)

(No description provided in vendor docs for IdeaStatiCa.ConnectionApi.Model.IGroup.)

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/IGroup.md)

### Properties
- `Faces` (List<int>, get/set) — (No description provided in vendor docs for IGroup.faces.)
- `Selected` (Selected, get/set) — (No description provided in vendor docs for IGroup.selected.)
- `Lines` (List<Line>, get/set) — (No description provided in vendor docs for IGroup.lines.)
- `Priority` (int, get/set) — (No description provided in vendor docs for IGroup.priority.)
- `Text` (List<Text>, get/set) — (No description provided in vendor docs for IGroup.text.)

## IdeaParameter (class)

(No description provided in vendor docs for IdeaStatiCa.ConnectionApi.Model.IdeaParameter.)

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/IdeaParameter.md)

### Properties
- `Key` (string, get/set) — (No description provided in vendor docs for IdeaParameter.key.)
- `Expression` (string, get/set) — (No description provided in vendor docs for IdeaParameter.expression.)
- `Default` (string, get/set) — (No description provided in vendor docs for IdeaParameter.default.)
- `Value` (object, get/set) — (No description provided in vendor docs for IdeaParameter.value.)
- `Unit` (string, get/set) — (No description provided in vendor docs for IdeaParameter.unit.)
- `ParameterType` (string, get/set) — (No description provided in vendor docs for IdeaParameter.parameterType.)
- `Description` (string, get/set) — (No description provided in vendor docs for IdeaParameter.description.)
- `ValidationStatus` (string, get/set) — (No description provided in vendor docs for IdeaParameter.validationStatus.)
- `IsVisible` (bool, get/set) — (No description provided in vendor docs for IdeaParameter.isVisible.)
- `LowerBound` (string, get/set) — (No description provided in vendor docs for IdeaParameter.lowerBound.)
- `UpperBound` (string, get/set) — (No description provided in vendor docs for IdeaParameter.upperBound.)
- `ParameterValidation` (IdeaParameterValidation, get/set) — (No description provided in vendor docs for IdeaParameter.parameterValidation.)

## IdeaParameterUpdate (class)

(No description provided in vendor docs for IdeaStatiCa.ConnectionApi.Model.IdeaParameterUpdate.)

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/IdeaParameterUpdate.md)

### Properties
- `Key` (string, get/set) — (No description provided in vendor docs for IdeaParameterUpdate.key.)
- `Expression` (string, get/set) — (No description provided in vendor docs for IdeaParameterUpdate.expression.)

## IdeaParameterValidation (class)

(No description provided in vendor docs for IdeaStatiCa.ConnectionApi.Model.IdeaParameterValidation.)

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/IdeaParameterValidation.md)

### Properties
- `ValidationExpression` (string, get/set) — (No description provided in vendor docs for IdeaParameterValidation.validationExpression.)
- `ValidationExpressionEvaluated` (bool, get/set) — (No description provided in vendor docs for IdeaParameterValidation.validationExpressionEvaluated.)
- `LowerBound` (string, get/set) — (No description provided in vendor docs for IdeaParameterValidation.lowerBound.)
- `LowerBoundEvaluated` (double, get/set) — (No description provided in vendor docs for IdeaParameterValidation.lowerBoundEvaluated.)
- `UpperBound` (string, get/set) — (No description provided in vendor docs for IdeaParameterValidation.upperBound.)
- `UpperBoundEvaluated` (double, get/set) — (No description provided in vendor docs for IdeaParameterValidation.upperBoundEvaluated.)
- `ValidationStatus` (string, get/set) — (No description provided in vendor docs for IdeaParameterValidation.validationStatus.)
- `Message` (string, get/set) — (No description provided in vendor docs for IdeaParameterValidation.message.)

## IdeaParameterValidationResponse (class)

(No description provided in vendor docs for IdeaStatiCa.ConnectionApi.Model.IdeaParameterValidationResponse.)

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/IdeaParameterValidationResponse.md)

### Properties
- `Key` (string, get/set) — (No description provided in vendor docs for IdeaParameterValidationResponse.key.)
- `Message` (string, get/set) — (No description provided in vendor docs for IdeaParameterValidationResponse.message.)
- `ValidationStatus` (string, get/set) — (No description provided in vendor docs for IdeaParameterValidationResponse.validationStatus.)

## ImportIOM_request (class)

(No description provided in vendor docs for IdeaStatiCa.ConnectionApi.Model.ImportIOM_request.)

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/api/openapi.yaml)

### Properties
- `ContainerXmlFile` (Stream, get/set) — IdeaRS.OpenModel.OpenModelContainer serialized to xml
- `ConnectionsToCreate` (List<int>, get/set) — (No description provided in vendor docs for ImportIOM_request.ConnectionsToCreate.)

## InstallationProcessTypeEnum (enum)

(No description provided in vendor docs for IdeaStatiCa.ConnectionApi.Model.InstallationProcessTypeEnum.)

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/InstallationProcessTypeEnum.md)

### Values
- `postInstalled` = `postInstalled`
- `castInPlace` = `castInPlace`

## Line (class)

(No description provided in vendor docs for IdeaStatiCa.ConnectionApi.Model.Line.)

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/Line.md)

### Properties
- `Color` (List<int>, get/set) — (No description provided in vendor docs for Line.color.)
- `Pairs` (List<int>, get/set) — (No description provided in vendor docs for Line.pairs.)
- `Thickness` (double, get/set) — (No description provided in vendor docs for Line.thickness.)

## MessageNumber (enum)

(No description provided in vendor docs for IdeaStatiCa.ConnectionApi.Model.MessageNumber.)

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/MessageNumber.md)

### Values
- `unspecified` = `unspecified`
- `information` = `information`
- `warning` = `warning`
- `warnNoPropertyInData` = `warnNoPropertyInData`
- `warnValueOutOfRange` = `warnValueOutOfRange`
- `warnCurveCount` = `warnCurveCount`
- `warnReinforcementBarsCollision` = `warnReinforcementBarsCollision`
- `error` = `error`
- `errNoOpenObject` = `errNoOpenObject`
- `errDataObjectNotCreated` = `errDataObjectNotCreated`
- `errNoObjectInOpenModel` = `errNoObjectInOpenModel`
- `errNoReferenceObjectInOpenModel` = `errNoReferenceObjectInOpenModel`
- `errNoEquivalentObjectInDataModel` = `errNoEquivalentObjectInDataModel`
- `errNoCrossSectionParameter` = `errNoCrossSectionParameter`
- `errBoltsTooClose` = `errBoltsTooClose`
- `errBoltsTooCloseEdge` = `errBoltsTooCloseEdge`
- `errContactsAngle` = `errContactsAngle`
- `errIncorrentMaterialE` = `errIncorrentMaterialE`
- `errIncorrectMaterialEGP` = `errIncorrectMaterialEGP`
- `errPreloadedBoltGrade` = `errPreloadedBoltGrade`
- `errValueOutOfRange` = `errValueOutOfRange`
- `errCurveZeroPoint` = `errCurveZeroPoint`
- `errCurveFunction` = `errCurveFunction`
- `errCurveDecreaseFunction` = `errCurveDecreaseFunction`
- `errCurveDerivation` = `errCurveDerivation`
- `errCurveNotSet` = `errCurveNotSet`
- `errValidPolyline` = `errValidPolyline`
- `errWarningLoad` = `errWarningLoad`
- `errTimeout` = `errTimeout`
- `errNoInLibrary` = `errNoInLibrary`
- `errBadWeldMaterialProperty` = `errBadWeldMaterialProperty`
- `errOperation` = `errOperation`
- `reserved` = `reserved`

## OpenElementId (class)

(No description provided in vendor docs for IdeaStatiCa.ConnectionApi.Model.OpenElementId.)

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/OpenElementId.md)

### Properties
- `Id` (int, get/set) — (No description provided in vendor docs for OpenElementId.id.)

## OpenMessage (class)

(No description provided in vendor docs for IdeaStatiCa.ConnectionApi.Model.OpenMessage.)

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/OpenMessage.md)

### Properties
- `Number` (MessageNumber, get/set) — (No description provided in vendor docs for OpenMessage.number.)
- `Description` (string, get/set) — (No description provided in vendor docs for OpenMessage.description.)

## OpenMessages (class)

(No description provided in vendor docs for IdeaStatiCa.ConnectionApi.Model.OpenMessages.)

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/OpenMessages.md)

### Properties
- `Messages` (List<OpenMessage>, get/set) — (No description provided in vendor docs for OpenMessages.messages.)

## OpenProject_request (class)

(No description provided in vendor docs for IdeaStatiCa.ConnectionApi.Model.OpenProject_request.)

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/api/openapi.yaml)

### Properties
- `IdeaConFile` (Stream, get/set) — The IdeaCon file to open.

## ParameterUpdateResponse (class)

(No description provided in vendor docs for IdeaStatiCa.ConnectionApi.Model.ParameterUpdateResponse.)

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/ParameterUpdateResponse.md)

### Properties
- `SetToModel` (bool, get/set) — (No description provided in vendor docs for ParameterUpdateResponse.setToModel.)
- `Parameters` (List<IdeaParameter>, get/set) — (No description provided in vendor docs for ParameterUpdateResponse.parameters.)
- `FailedValidations` (List<IdeaParameterValidationResponse>, get/set) — (No description provided in vendor docs for ParameterUpdateResponse.failedValidations.)

## PinGrid (class)

(No description provided in vendor docs for IdeaStatiCa.ConnectionApi.Model.PinGrid.)

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/PinGrid.md)

### Properties
- `IsReplaceable` (bool, get/set) — (No description provided in vendor docs for PinGrid.isReplaceable.)
- `Pin` (ReferenceElement, get/set) — (No description provided in vendor docs for PinGrid.pin.)
- `Origin` (Point3D, get/set) — (No description provided in vendor docs for PinGrid.origin.)
- `AxisX` (Vector3D, get/set) — (No description provided in vendor docs for PinGrid.axisX.)
- `AxisY` (Vector3D, get/set) — (No description provided in vendor docs for PinGrid.axisY.)
- `AxisZ` (Vector3D, get/set) — (No description provided in vendor docs for PinGrid.axisZ.)
- `Positions` (List<Point3D>, get/set) — (No description provided in vendor docs for PinGrid.positions.)
- `ConnectedParts` (List<ReferenceElement>, get/set) — (No description provided in vendor docs for PinGrid.connectedParts.)
- `Name` (string, get/set) — (No description provided in vendor docs for PinGrid.name.)
- `Length` (double, get/set) — (No description provided in vendor docs for PinGrid.length.)
- `Id` (int, get/set) — (No description provided in vendor docs for PinGrid.id.)

## PlateData (class)

(No description provided in vendor docs for IdeaStatiCa.ConnectionApi.Model.PlateData.)

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/PlateData.md)

### Properties
- `Name` (string, get/set) — (No description provided in vendor docs for PlateData.name.)
- `Thickness` (double, get/set) — (No description provided in vendor docs for PlateData.thickness.)
- `Material` (string, get/set) — (No description provided in vendor docs for PlateData.material.)
- `OutlinePoints` (List<Point2D>, get/set) — (No description provided in vendor docs for PlateData.outlinePoints.)
- `Origin` (Point3D, get/set) — (No description provided in vendor docs for PlateData.origin.)
- `AxisX` (Vector3D, get/set) — (No description provided in vendor docs for PlateData.axisX.)
- `AxisY` (Vector3D, get/set) — (No description provided in vendor docs for PlateData.axisY.)
- `AxisZ` (Vector3D, get/set) — (No description provided in vendor docs for PlateData.axisZ.)
- `Region` (string, get/set) — (No description provided in vendor docs for PlateData.region.)
- `Geometry` (Region2D, get/set) — (No description provided in vendor docs for PlateData.geometry.)
- `OriginalModelId` (string, get/set) — (No description provided in vendor docs for PlateData.originalModelId.)
- `IsNegativeObject` (bool, get/set) — (No description provided in vendor docs for PlateData.isNegativeObject.)
- `Id` (int, get/set) — (No description provided in vendor docs for PlateData.id.)

## Point2D (class)

(No description provided in vendor docs for IdeaStatiCa.ConnectionApi.Model.Point2D.)

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/Point2D.md)

### Properties
- `X` (double, get/set) — (No description provided in vendor docs for Point2D.x.)
- `Y` (double, get/set) — (No description provided in vendor docs for Point2D.y.)

## Point3D (class)

(No description provided in vendor docs for IdeaStatiCa.ConnectionApi.Model.Point3D.)

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/Point3D.md)

### Properties
- `X` (double, get/set) — (No description provided in vendor docs for Point3D.x.)
- `Y` (double, get/set) — (No description provided in vendor docs for Point3D.y.)
- `Z` (double, get/set) — (No description provided in vendor docs for Point3D.z.)
- `Id` (int, get/set) — (No description provided in vendor docs for Point3D.id.)

## PolyLine2D (class)

(No description provided in vendor docs for IdeaStatiCa.ConnectionApi.Model.PolyLine2D.)

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/PolyLine2D.md)

### Properties
- `StartPoint` (Point2D, get/set) — (No description provided in vendor docs for PolyLine2D.startPoint.)
- `Segments` (List<Segment2D>, get/set) — (No description provided in vendor docs for PolyLine2D.segments.)

## ReferenceElement (class)

(No description provided in vendor docs for IdeaStatiCa.ConnectionApi.Model.ReferenceElement.)

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/ReferenceElement.md)

### Properties
- `TypeName` (string, get/set) — (No description provided in vendor docs for ReferenceElement.typeName.)
- `Id` (int, get/set) — (No description provided in vendor docs for ReferenceElement.id.)
- `Element` (OpenElementId, get/set) — (No description provided in vendor docs for ReferenceElement.element.)

## Region2D (class)

(No description provided in vendor docs for IdeaStatiCa.ConnectionApi.Model.Region2D.)

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/Region2D.md)

### Properties
- `Outline` (PolyLine2D, get/set) — (No description provided in vendor docs for Region2D.outline.)
- `Openings` (List<PolyLine2D>, get/set) — (No description provided in vendor docs for Region2D.openings.)

## SearchOption (enum)

(No description provided in vendor docs for IdeaStatiCa.ConnectionApi.Model.SearchOption.)

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/SearchOption.md)

### Values
- `ignore` = `ignore`
- `must` = `must`
- `mustNot` = `mustNot`

## Segment2D (class)

(No description provided in vendor docs for IdeaStatiCa.ConnectionApi.Model.Segment2D.)

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/Segment2D.md)

### Properties
- `EndPoint` (Point2D, get/set) — (No description provided in vendor docs for Segment2D.endPoint.)

## Selected (class)

(No description provided in vendor docs for IdeaStatiCa.ConnectionApi.Model.Selected.)

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/Selected.md)

### Properties
- `Id` (string, get/set) — (No description provided in vendor docs for Selected.id.)
- `Type` (SelectedType, get/set) — (No description provided in vendor docs for Selected.type.)

## SelectedType (class)

(No description provided in vendor docs for IdeaStatiCa.ConnectionApi.Model.SelectedType.)

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/SelectedType.md)

### Properties
- `Kind` (string, get) — (No description provided in vendor docs for SelectedType.kind.)

## TemplateConversions (class)

(No description provided in vendor docs for IdeaStatiCa.ConnectionApi.Model.TemplateConversions.)

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/TemplateConversions.md)

### Properties
- `Conversions` (List<BaseTemplateConversion>, get/set) — (No description provided in vendor docs for TemplateConversions.conversions.)
- `CountryCode` (string, get/set) — (No description provided in vendor docs for TemplateConversions.countryCode.)

## Text (class)

(No description provided in vendor docs for IdeaStatiCa.ConnectionApi.Model.Text.)

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/Text.md)

### Properties
- `Color` (List<int>, get/set) — (No description provided in vendor docs for Text.color.)
- `Value` (string, get/set) — (No description provided in vendor docs for Text.value.)
- `Position` (TextPosition, get/set) — (No description provided in vendor docs for Text.position.)
- `FontSize` (double, get/set) — (No description provided in vendor docs for Text.fontSize.)
- `Tag` (double, get/set) — (No description provided in vendor docs for Text.tag.)

## TextPosition (class)

(No description provided in vendor docs for IdeaStatiCa.ConnectionApi.Model.TextPosition.)

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/TextPosition.md)

### Properties
- `Origin` (List<double>, get/set) — (No description provided in vendor docs for TextPosition.origin.)
- `Angles` (List<double>, get/set) — (No description provided in vendor docs for TextPosition.angles.)

## UpdateFromIOM_request (class)

(No description provided in vendor docs for IdeaStatiCa.ConnectionApi.Model.UpdateFromIOM_request.)

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/api/openapi.yaml)

### Properties
- `ContainerXmlFile` (Stream, get/set) — IdeaRS.OpenModel.OpenModelContainer serialized to xml

## Vector3D (class)

(No description provided in vendor docs for IdeaStatiCa.ConnectionApi.Model.Vector3D.)

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/Vector3D.md)

### Properties
- `X` (double, get/set) — (No description provided in vendor docs for Vector3D.x.)
- `Y` (double, get/set) — (No description provided in vendor docs for Vector3D.y.)
- `Z` (double, get/set) — (No description provided in vendor docs for Vector3D.z.)

## WeldData (class)

(No description provided in vendor docs for IdeaStatiCa.ConnectionApi.Model.WeldData.)

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/WeldData.md)

### Properties
- `Id` (int, get/set) — (No description provided in vendor docs for WeldData.id.)
- `Name` (string, get/set) — (No description provided in vendor docs for WeldData.name.)
- `Thickness` (double, get/set) — (No description provided in vendor docs for WeldData.thickness.)
- `Material` (string, get/set) — (No description provided in vendor docs for WeldData.material.)
- `WeldMaterial` (ReferenceElement, get/set) — (No description provided in vendor docs for WeldData.weldMaterial.)
- `WeldType` (WeldType, get/set) — (No description provided in vendor docs for WeldData.weldType.)
- `ConnectedPartIds` (List<string>, get/set) — (No description provided in vendor docs for WeldData.connectedPartIds.)
- `Start` (Point3D, get/set) — (No description provided in vendor docs for WeldData.start.)
- `End` (Point3D, get/set) — (No description provided in vendor docs for WeldData.end.)

## WeldDefinition (class)

(No description provided in vendor docs for IdeaStatiCa.ConnectionApi.Model.WeldDefinition.)

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/WeldDefinition.md)

### Properties
- `Thickness` (double, get/set) — (No description provided in vendor docs for WeldDefinition.thickness.)
- `WeldType` (WeldType, get/set) — (No description provided in vendor docs for WeldDefinition.weldType.)
- `WeldMaterial` (ReferenceElement, get/set) — (No description provided in vendor docs for WeldDefinition.weldMaterial.)

## WeldType (enum)

(No description provided in vendor docs for IdeaStatiCa.ConnectionApi.Model.WeldType.)

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/WeldType.md)

### Values
- `notSpecified` = `notSpecified`
- `fillet` = `fillet`
- `doubleFillet` = `doubleFillet`
- `bevel` = `bevel`
- `pjp` = `pjp`
- `pjpRear` = `pjpRear`
- `lengthAtHaunch` = `lengthAtHaunch`
- `filletRear` = `filletRear`
- `contact` = `contact`
- `intermittent` = `intermittent`

