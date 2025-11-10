# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

aegnt-27 is a Rust library for achieving peak human authenticity through 27 distinct behavioral patterns. It includes modules for mouse movement humanization, typing pattern humanization, audio processing, visual enhancement, and AI detection resistance. The project also provides an MCP (Model Context Protocol) server for integration with Claude.

## Architecture

The project consists of three main components:

1. **Core Rust Library** (`src/`): Implements humanization algorithms across 27 behavioral patterns
   - Mouse movement humanization (7 patterns)
   - Typing pattern humanization (7 patterns) 
   - Audio processing humanization (7 patterns)
   - Visual enhancement (6 patterns)
   - Detection resistance and authenticity validation

2. **MCP Server** (`mcp-server/`): TypeScript/Bun-based server for Claude integration
   - Exposes aegnt-27 functionality as MCP tools
   - Provides real-time humanization capabilities

3. **Commercial Engines** (`proprietary-engines/`): High-performance proprietary implementations
   - Enhanced authenticity algorithms (95%+ human authenticity)
   - Optimized for production use

## Commands

### Rust Development
```bash
# Build the library
cargo build --release

# Build with all features
cargo build --release --features=full

# Build with specific features
cargo build --release --features="mouse,typing,detection"

# Run tests
cargo test

# Run benchmarks
cargo bench

# Build commercial engines (proprietary features)
./scripts/build-commercial.sh

# Run simple performance test
cargo run --bin simple_perf_test
```

### MCP Server Development
```bash
# Install dependencies (using Bun - recommended)
cd mcp-server && bun install

# Build the MCP server
bun run build

# Development mode with hot reload
bun run dev

# Run MCP server
bun run start

# Run tests
bun test

# Lint TypeScript code
bun run lint

# Format code
bun run format
```

## Key Module Features

### Feature Flags
- `default`: Basic humanization and detection validation
- `full`: All features including ML models and video processing
- `mouse`: Mouse movement humanization
- `typing`: Typing pattern humanization
- `audio`: Audio processing humanization
- `visual`: Visual enhancement
- `detection`: AI detection resistance
- `persistence`: State persistence
- `ml-models`: Machine learning models (requires candle/tch)
- `video-processing`: Video processing (requires opencv/ffmpeg)
- `network-features`: Network capabilities (requires reqwest/hyper)

### Core Modules
- `src/lib.rs`: Main library entry point and engine builder
- `src/mouse.rs`: Mouse movement humanization (Bezier curves, micro-movements, drift)
- `src/typing.rs`: Typing pattern humanization (keystroke timing, error injection)
- `src/audio.rs`: Audio processing (breathing patterns, vocal variations)
- `src/visual.rs`: Visual enhancement (gaze patterns, attention modeling)
- `src/detection.rs`: AI detection resistance algorithms
- `src/authenticity.rs`: Human authenticity validation
- `src/config.rs`: Configuration management
- `src/error.rs`: Error types and handling

## Environment Variables

For commercial builds:
- `AEGNT27_COMMERCIAL_BUILD=1`: Enable commercial build mode
- `AEGNT27_COMMERCIAL_LICENSE`: Commercial license key (format: COMM-*)

## Testing Approach

The project uses multiple testing strategies:
- Unit tests: In each module file (run with `cargo test`)
- Integration tests: In `examples/` directory
- Performance tests: Using Criterion benchmarks (`cargo bench`)
- MCP server tests: Using Bun test runner (`bun test` in mcp-server/)

## Performance Targets

- Mouse movement: <2ms latency, 96% authenticity
- Typing patterns: <1ms latency, 95% authenticity  
- Audio processing: Real-time, 94% authenticity
- Visual enhancement: 30fps capable, 93% authenticity
- Detection resistance: 98% evasion rate
- Memory usage: <200MB for full feature set

## Integration with Claude

To use aegnt-27 tools in Claude Desktop, add to your config:
```json
{
  "mcpServers": {
    "aegnt27": {
      "command": "bun",
      "args": ["/path/to/aegnt27/mcp-server/dist/index.js"]
    }
  }
}
```

Then use tools like:
- `achieve_mouse_authenticity`: Create natural mouse paths
- `achieve_typing_authenticity`: Generate realistic typing patterns
- `validate_authenticity`: Check content authenticity score