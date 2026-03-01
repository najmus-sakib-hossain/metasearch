# Engine Display Fix - Completed

## Problem
Failed engines were showing as "unknown" in the UI instead of their actual names.

## Solution
Modified `crates/metasearch-server/src/orchestrator.rs` to preserve engine names even when they fail or timeout.

### Changes Made

1. **Modified the futures to return engine name in all cases**:
   - Changed the return type from `Option<(name, weight, results)>` to `(name, Option<(name, weight, results)>)`
   - Now the engine name is always returned, even on failure

2. **Updated the result processing**:
   - Changed `while let Some(result)` to `while let Some((engine_name, result))`
   - Failed engines now push the actual `engine_name` instead of `"unknown".to_string()`

## Result
- ✅ Failed engines now display their actual names in the UI
- ✅ The sidebar "Show failed engines" section will show proper engine names
- ✅ Yahoo and other working engines continue to work correctly
- ✅ Better debugging and monitoring capabilities

## Testing
1. Build: `cargo build --release`
2. Run: `cargo run --release --bin metasearch -- serve`
3. Visit: http://localhost:8888
4. Search for anything and check the sidebar to see failed engine names

## UI Features
The results page now shows:
- **Top info bar**: Lists all engines that responded successfully (e.g., Yahoo)
- **Sidebar - Engines section**: 
  - Shows count of responded engines
  - Shows count of failed engines
  - Expandable "Show failed engines" list with actual engine names (no more "unknown")
