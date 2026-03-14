export interface WasmAnalyzer {
  analyze(source: string, file_path: string): string;
  version(): string;
}

// This will be imported dynamically in App.tsx
export type { WasmAnalyzer as default };
