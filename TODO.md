# TODO

## Completed Tasks

1. Implement core cryptographic algorithms:
   - NewHope (Lattice-based)
   - SPHINCS+ (Hash-based)
   - McEliece (Code-based)
   - SIKE (Isogeny-based)

2. Develop a basic profiling tool:
   - Measure execution time for each algorithm.
   - Integrated profiling into the core library.

3. Develop core library functionality:
   - Created `CryptoToolkit` to manage algorithms and profiling.

4. Improve documentation:
   - Updated `README.md` with detailed information and usage examples.
   - Added detailed comments and documentation within the code.
   - Created an `examples/` directory with usage examples.

## Next Steps

1. Expand and improve profiling tools:
   - Add memory usage profiling.
   - Benchmark algorithms under different conditions.

2. Enhance cryptographic algorithms:
   - Implement more advanced features for each algorithm (e.g., key generation, full signature verification, etc.).
   - Optimize performance for specific use cases.

3. Extend API functionality:
   - Create a RESTful API for integrating with external systems.
   - Add support for standard protocols (e.g., TLS).

4. Develop additional examples and tutorials:
   - Add more complex examples covering full cryptographic operations.
   - Create tutorials to guide users through implementing the toolkit in their projects.

5. Explore deployment options:
   - Package the library for distribution on `crates.io`.
   - Consider creating a web interface for easy access to the toolkit's features.
