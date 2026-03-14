# Rust Pattern Viz - Web Demo

Interactive browser-based demonstration of the Rust Pattern Visualizer, powered by WebAssembly.

## Features

- **Real-time Analysis**: Edit Rust code in the browser and analyze patterns instantly
- **Monaco Editor**: Full-featured code editor with Rust syntax highlighting
- **Example Code**: Pre-loaded examples demonstrating different Rust patterns
- **Pattern Detection**: Visualize error handling, iterators, lifetimes, traits, and more
- **Decision Trees**: See the reasoning behind detected patterns
- **Zero Backend**: Runs entirely in the browser using WebAssembly

## Development

```bash
# Install dependencies
cd web-demo
npm install

# Build WASM module (run from project root)
cd ..
npm run wasm:build

# Start development server
cd web-demo
npm run dev
```

Open http://localhost:3000 to see the demo.

## Building for Production

```bash
# From web-demo directory
npm run wasm:build  # Build WASM first
npm run build       # Build web assets
npm run preview     # Preview production build
```

## Deployment

The demo is automatically deployed to GitHub Pages via GitHub Actions when pushing to `main`:

```bash
git add .
git commit -m "Update web demo"
git push origin main
```

Visit: `https://yourusername.github.io/rust-pattern-viz/`

## Architecture

```
┌─────────────────────────────────────┐
│         Browser (User)              │
│  ┌──────────────────────────────┐   │
│  │   React UI (TypeScript)      │   │
│  │  - Monaco Editor             │   │
│  │  - Pattern Visualization     │   │
│  └──────────┬───────────────────┘   │
│             │ JS API calls           │
│  ┌──────────▼───────────────────┐   │
│  │   WebAssembly Module         │   │
│  │   (rust-pattern-viz)         │   │
│  │  - Code Analysis             │   │
│  │  - Pattern Detection         │   │
│  │  - Returns JSON              │   │
│  └──────────────────────────────┘   │
└─────────────────────────────────────┘
```

## Technologies

- **Rust + wasm-bindgen**: Core analyzer compiled to WebAssembly
- **React**: UI framework
- **TypeScript**: Type-safe frontend code
- **Monaco Editor**: VS Code's editor in the browser
- **Vite**: Fast build tool and dev server
- **GitHub Pages**: Static hosting

## Browser Requirements

- Modern browser with WebAssembly support (Chrome 57+, Firefox 52+, Safari 11+, Edge 16+)
- JavaScript enabled

## License

MIT OR Apache-2.0
