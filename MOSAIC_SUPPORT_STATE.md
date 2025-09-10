# Mosaic API support in nvapi-sys (sys crate)

AI Generated Analysis Report
Date: 2025-09-10
Scope: `sys` crate (`nvapi-sys`), branch `quadro_gsync_improvements`

## Summary

- No Mosaic API FFI wrappers are implemented in the sys crate.
  - There are no `nvapi_fn!` declarations for any `NvAPI_Mosaic_*` functions nor for the legacy `NvAPI_*MosaicTopology` functions under `sys/src`.
  - There is no `mosaic.rs` module and no Mosaic-related symbols exported from `sys/src/lib.rs`.
- No Mosaic-related structs/enums/typedefs are defined.
  - `sys/src/types.rs` does not contain types like `NV_MOSAIC_DISPLAY_GRID`, `NV_MOSAIC_TOPOLOGY`, `NV_MOSAIC_VIEWPORT`, grid/topology enums, or capability structs.
- Only references present are:
  - Status codes mentioning Mosaic in `sys/src/status.rs` (e.g., `NVAPI_MOSAIC_NOT_ACTIVE`, `NVAPI_TOPO_NOT_POSSIBLE`).
  - The function ID ordinals for Mosaic entry points in `sys/src/nvid.rs` (listed below).

Conclusion: Mosaic is not currently implemented in `nvapi-sys`; only the ordinals and a couple of status codes exist. To use Mosaic, new FFI wrappers and supporting types need to be added.

## Evidence and file pointers

- `sys/src/nvapi.rs`: contains many `nvapi_fn!` wrappers, but none for Mosaic (verified; 0 matches for `Mosaic`).
- `sys/src/types.rs`: no Mosaic-related types (verified; 0 matches for `MOSAIC`, `GRID`, `VIEWPORT` relevant to Mosaic).
- `sys/src/status.rs`: includes Mosaic-related errors:
  - `NVAPI_TOPO_NOT_POSSIBLE` — “The mosaic topology is not possible given the current state of the hardware.”
  - `NVAPI_MOSAIC_NOT_ACTIVE` — “The requested action cannot be performed without Mosaic being enabled.”
- `sys/src/nvid.rs`: declares ordinals for Mosaic functions (IDs only; wrappers absent):
  - `NvAPI_Mosaic_GetSupportedTopoInfo`
  - `NvAPI_Mosaic_GetTopoGroup`
  - `NvAPI_Mosaic_GetOverlapLimits`
  - `NvAPI_Mosaic_SetCurrentTopo`
  - `NvAPI_Mosaic_GetCurrentTopo`
  - `NvAPI_Mosaic_EnableCurrentTopo`
  - `NvAPI_Mosaic_SetGridTopology`
  - `NvAPI_Mosaic_GetMosaicCapabilities`
  - `NvAPI_Mosaic_GetDisplayCapabilities`
  - `NvAPI_Mosaic_EnumGridTopologies`
  - `NvAPI_Mosaic_GetDisplayViewportsByResolution`
  - `NvAPI_Mosaic_GetMosaicViewports`
  - `NvAPI_Mosaic_SetDisplayGrids`
  - `NvAPI_Mosaic_ValidateDisplayGridsWithSLI`
  - `NvAPI_Mosaic_ValidateDisplayGrids`
  - `NvAPI_Mosaic_EnumDisplayModes`
  - `NvAPI_Mosaic_ChooseGpuTopologies`
  - `NvAPI_Mosaic_EnumDisplayGrids`
  - `NvAPI_GetSupportedMosaicTopologies`
  - `NvAPI_GetCurrentMosaicTopology`
  - `NvAPI_SetCurrentMosaicTopology`
  - `NvAPI_EnableCurrentMosaicTopology`

## Steps needed to implement Mosaic

- Define required structs/enums as per NVAPI Mosaic documentation (e.g., display grids, viewports, topologies, capability structs, and their `version` constants via `MAKE_NVAPI_VERSION`).
- Add `nvapi_fn!` wrappers in a new `sys/src/mosaic.rs` (or similar) that bind to the above ordinals in `nvid.rs` and export the functions from `sys/src/lib.rs`.
- Validate ABI layouts (packing/alignment) and mark functions as `unsafe extern "C"` per existing pattern.
- Add minimal tests or examples (e.g., query current topology) guarded for environments where Mosaic is available.

## Verdict

- Current state: Mosaic API — Not implemented (no callable sys-layer functions/types).
- Low-effort next step: add a thin wrapper for one read-only call (e.g., `NvAPI_GetCurrentMosaicTopology`) to prove the path, then layer additional functions and types.
