This is the MIT-licensed crates.io release of Taffy 0.9.0 (upstream commit
599b1b7d7410d97e13bafc4c01fbe6d7a9cf060a). GPUI 0.2.2 pins that exact Taffy
version. The only upstream manifest change raises the minimum Grid dependency
to 1.0.1, the first version fixed for CVE-2026-42199 (GHSA-38c5-483c-4qqp).
Taffy 0.9.2 also moved to Grid 1.x but does not satisfy GPUI's pin.
Remove this override when GPUI can resolve a Taffy release using patched Grid.
