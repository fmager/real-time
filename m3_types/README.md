# 1️⃣ Types
```u8``` rarely matters, but ```[u8]``` sure does.
Correctness
Precision
Bandwidth
Speed
Size
Energy Consumption
Maintain consistency and reduce bugs in calculations by locking in the types - Rust circumvents this by removing implicit casting
Choosing the right types enables us to use certain accelerators, GPU's are in general much faster if you use float 32's instead of float 64's and can be even faster if you manage to lower your precision enough to use tensor cores
