# Project Philosophy

## Purpose

`ed318-canada-lab` exists to study ED-318 implementation strategies before attempting to automate them. The project treats national datasets, publication workflows, update lifecycles, distribution choices, and schema alignment as engineering evidence.

## Reference before software

A parser or validator should be designed only after the project understands both the reference model and real national publications. Version 0.3.0 therefore prioritizes evidence, terminology, provenance, and comparison over executable capability.

## Authority boundary

This repository is not EUROCAE, Transport Canada, NAV CANADA, ENAIRE, the Irish Aviation Authority, Air Navigation Services of the Czech Republic, or another competent authority. It does not create legal requirements or operational permissions.

## Evidence layers

The project keeps the following layers distinct:

1. the ED-318 standard;
2. machine-readable reference schemas;
3. synthetic reference examples;
4. real national implementations;
5. project-authored samples and analysis;
6. future Canadian proposals;
7. later software implementations.

No lower-authority layer may silently be presented as a higher-authority layer.

## National implementation profiles

A national profile includes more than JSON field names. It may include:

- the publishing authority;
- source datasets and legal instruments;
- thematic or regulatory organization;
- geometry and vertical-limit representation;
- update cadence and effective dates;
- integrity mechanisms such as checksums;
- full-file, tiled, or thematic distribution;
- deviations from the reference schemas;
- operational and technical limitations.

## Immutable-source and derived-sample principle

Complete source files are checked and described before sampling. Small repository samples are explicitly marked as derived. Their selection rule, source checksum, feature identifiers, and serialization method are recorded.

## Neutral comparison

The project does not rank countries as simply “correct” or “incorrect.” Ireland, Spain, and the Czech Republic demonstrate different implementation and publication strategies. Differences are documented as evidence for later engineering decisions.

## Canadian scope

Canadian work remains a future strategy study unless and until an authoritative Canadian ED-318 publication exists. Proposed Canadian structures must be labelled as project proposals, not official policy.

## Change discipline

Every accepted release identifies its baseline, reviewed files, modified files, semantic changes, checks, provenance additions, and unresolved risks.
