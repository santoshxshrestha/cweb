'use client';

import { useState, useEffect, useRef } from 'react';
import dynamic from 'next/dynamic';
import { initWasm, compileAndRunC } from '@/lib/wasmLoader';

// Dynamically import CodeEditor to avoid SSR issues with Ace Editor
const CodeEditor = dynamic(() => import('@/components/CodeEditor'), {
  ssr: false,
  loading: () => (
    <div className="flex items-center justify-center h-full bg-gray-900">
      <p className="text-white">Loading editor...</p>
    </div>
  ),
});

const DEFAULT_CODE = `#include <stdio.h>

int main() {
    printf("Hello, World!\\n");
    return 0;
}`;

// Available editor themes
const EDITOR_THEMES = [
  { value: 'monokai', label: 'Monokai' },
  { value: 'github', label: 'GitHub Light' },
  { value: 'dracula', label: 'Dracula' },
  { value: 'tomorrow_night', label: 'Tomorrow Night' },
  { value: 'solarized_dark', label: 'Solarized Dark' },
  { value: 'solarized_light', label: 'Solarized Light' },
  { value: 'terminal', label: 'Terminal' },
  { value: 'twilight', label: 'Twilight' },
] as const;

type EditorTheme = typeof EDITOR_THEMES[number]['value'];

export default function Home() {
  // Code editor state
  const [code, setCode] = useState(DEFAULT_CODE);
  const [output, setOutput] = useState('');
  const [isRunning, setIsRunning] = useState(false);
  const [theme, setTheme] = useState<EditorTheme>('monokai');
  const [vimMode, setVimMode] = useState(false);
  const [wasmReady, setWasmReady] = useState(false);

  // UI state
  const [isOutputVisible, setIsOutputVisible] = useState(true);
  const [isFullscreen, setIsFullscreen] = useState(false);
  const [editorWidth, setEditorWidth] = useState(66.67); // 2/3 of screen in percentage
  const [isResizing, setIsResizing] = useState(false);

  // Refs for resizing
  const containerRef = useRef<HTMLDivElement>(null);
  const resizeStartX = useRef<number>(0);
  const resizeStartWidth = useRef<number>(0);

  // Initialize WASM module on component mount
  useEffect(() => {
    const loadWasm = async () => {
      try {
        await initWasm();
        setWasmReady(true);
        console.log('WASM module loaded successfully');
      } catch (error) {
        console.error('Failed to load WASM:', error);
        setOutput(`Error: Failed to initialize compiler\n${error}`);
      }
    };

    loadWasm();
  }, []);

  // Handle mouse move during resize
  useEffect(() => {
    const handleMouseMove = (e: MouseEvent) => {
      if (!isResizing || !containerRef.current) return;

      const containerWidth = containerRef.current.offsetWidth;
      const deltaX = e.clientX - resizeStartX.current;
      const deltaPercent = (deltaX / containerWidth) * 100;
      const newWidth = Math.min(Math.max(resizeStartWidth.current + deltaPercent, 30), 90);
      
      setEditorWidth(newWidth);
    };

    const handleMouseUp = () => {
      setIsResizing(false);
    };

    if (isResizing) {
      document.addEventListener('mousemove', handleMouseMove);
      document.addEventListener('mouseup', handleMouseUp);
    }

    return () => {
      document.removeEventListener('mousemove', handleMouseMove);
      document.removeEventListener('mouseup', handleMouseUp);
    };
  }, [isResizing]);

  // Listen for fullscreen changes
  useEffect(() => {
    const handleFullscreenChange = () => {
      setIsFullscreen(!!document.fullscreenElement);
    };

    document.addEventListener('fullscreenchange', handleFullscreenChange);
    return () => document.removeEventListener('fullscreenchange', handleFullscreenChange);
  }, []);

  const handleRunCode = async () => {
    if (!wasmReady) {
      setOutput('Error: Compiler is still loading. Please wait...');
      return;
    }

    setIsRunning(true);
    setOutput('Compiling and running...\n');
    
    // Show output panel when running code
    if (!isOutputVisible) {
      setIsOutputVisible(true);
    }
    
    try {
      // Compile and run the C code using WASM
      const result = await compileAndRunC(code);
      
      if (result.success) {
        setOutput(`Output:\n${result.output}\n\nProgram completed successfully.`);
      } else {
        setOutput(`Compilation Error:\n${result.error || 'Unknown error'}`);
      }
    } catch (error) {
      setOutput(`Error: ${error}\n\nPlease check your code and try again.`);
    } finally {
      setIsRunning(false);
    }
  };

  const handleClearOutput = () => {
    setOutput('');
  };

  const toggleVimMode = () => {
    setVimMode(prev => !prev);
  };

  // Handle mouse down on resize handle
  const handleMouseDown = (e: React.MouseEvent) => {
    e.preventDefault();
    setIsResizing(true);
    resizeStartX.current = e.clientX;
    resizeStartWidth.current = editorWidth;
  };

  // Toggle fullscreen mode
  const toggleFullscreen = () => {
    if (!document.fullscreenElement) {
      document.documentElement.requestFullscreen();
      setIsFullscreen(true);
    } else {
      document.exitFullscreen();
      setIsFullscreen(false);
    }
  };

  // Toggle output panel visibility
  const toggleOutputPanel = () => {
    setIsOutputVisible(prev => !prev);
  };

  return (
    <div className="flex flex-col h-screen bg-gray-950">
      {/* Header */}
      <header className="flex items-center justify-between px-6 py-4 bg-gray-900 border-b border-gray-800">
        <div className="flex items-center gap-3">
          <div className="text-2xl font-bold text-white">WebC</div>
          <div className={`px-3 py-1 text-white text-sm rounded-full ${wasmReady ? 'bg-green-600' : 'bg-yellow-600'}`}>
            {wasmReady ? 'Ready' : 'Loading...'}
          </div>
        </div>
        <div className="flex items-center gap-3">
          {/* Theme Selector */}
          <select
            value={theme}
            onChange={(e) => setTheme(e.target.value as EditorTheme)}
            className="px-4 py-2 bg-gray-800 hover:bg-gray-700 text-white rounded-lg transition-colors cursor-pointer focus:outline-none focus:ring-2 focus:ring-blue-500"
          >
            {EDITOR_THEMES.map(t => (
              <option key={t.value} value={t.value}>{t.label}</option>
            ))}
          </select>

          {/* Vim Mode Toggle */}
          <button
            onClick={toggleVimMode}
            className={`px-4 py-2 rounded-lg transition-colors font-medium ${
              vimMode 
                ? 'bg-purple-600 hover:bg-purple-700 text-white' 
                : 'bg-gray-800 hover:bg-gray-700 text-white'
            }`}
            title="Toggle Vim mode"
          >
            Vim: {vimMode ? 'ON' : 'OFF'}
          </button>

          {/* Output Panel Toggle */}
          <button
            onClick={toggleOutputPanel}
            className={`px-4 py-2 rounded-lg transition-colors font-medium ${
              isOutputVisible 
                ? 'bg-blue-600 hover:bg-blue-700 text-white' 
                : 'bg-gray-800 hover:bg-gray-700 text-white'
            }`}
            title="Toggle output panel"
          >
            {isOutputVisible ? '📊 Hide Output' : '📊 Show Output'}
          </button>

          {/* Fullscreen Toggle */}
          <button
            onClick={toggleFullscreen}
            className="px-4 py-2 bg-gray-800 hover:bg-gray-700 text-white rounded-lg transition-colors"
            title="Toggle fullscreen"
          >
            {isFullscreen ? '⤓ Exit Fullscreen' : '⤢ Fullscreen'}
          </button>

          {/* Run Code Button */}
          <button
            onClick={handleRunCode}
            disabled={isRunning || !wasmReady}
            className="px-6 py-2 bg-green-600 hover:bg-green-700 disabled:bg-gray-600 disabled:cursor-not-allowed text-white font-semibold rounded-lg transition-colors flex items-center gap-2"
          >
            {isRunning ? (
              <>
                <span className="inline-block w-4 h-4 border-2 border-white border-t-transparent rounded-full animate-spin" />
                Running...
              </>
            ) : (
              <>
                <span>▶</span>
                Run Code
              </>
            )}
          </button>
        </div>
      </header>

      {/* Main Content */}
      <div ref={containerRef} className="flex flex-1 overflow-hidden">
        {/* Editor Panel */}
        <div 
          className="flex flex-col border-r border-gray-800 transition-all duration-200"
          style={{ width: isOutputVisible ? `${editorWidth}%` : '100%' }}
        >
          <div className="flex items-center justify-between px-4 py-2 bg-gray-900 border-b border-gray-800">
            <h2 className="text-sm font-semibold text-gray-300">main.c</h2>
            <div className="flex items-center gap-4">
              {vimMode && (
                <div className="px-2 py-1 bg-purple-600 text-white text-xs rounded font-medium">
                  VIM MODE
                </div>
              )}
              <div className="text-xs text-gray-500">
                Press Ctrl+Space for auto-completion
              </div>
            </div>
          </div>
          <div className="flex-1">
            <CodeEditor
              value={code}
              onChange={setCode}
              theme={theme}
              vimMode={vimMode}
            />
          </div>
        </div>

        {/* Resize Handle */}
        {isOutputVisible && (
          <div
            onMouseDown={handleMouseDown}
            className={`w-1 bg-gray-700 hover:bg-blue-500 cursor-col-resize transition-colors ${
              isResizing ? 'bg-blue-500' : ''
            }`}
            title="Drag to resize"
          />
        )}

        {/* Output Panel */}
        {isOutputVisible && (
          <div 
            className="flex flex-col bg-gray-900 transition-all duration-200"
            style={{ width: `${100 - editorWidth}%` }}
          >
            <div className="flex items-center justify-between px-4 py-2 bg-gray-800 border-b border-gray-700">
              <h2 className="text-sm font-semibold text-gray-300">Output</h2>
              <div className="flex gap-2">
                <button
                  onClick={handleClearOutput}
                  className="px-3 py-1 text-xs bg-gray-700 hover:bg-gray-600 text-white rounded transition-colors"
                >
                  Clear
                </button>
                <button
                  onClick={toggleOutputPanel}
                  className="px-3 py-1 text-xs bg-gray-700 hover:bg-gray-600 text-white rounded transition-colors"
                  title="Close output panel"
                >
                  ✕
                </button>
              </div>
            </div>
            <div className="flex-1 overflow-auto p-4">
              <pre className="text-sm text-gray-300 font-mono whitespace-pre-wrap">
                {output || 'Click "Run Code" to see output here...'}
              </pre>
            </div>
          </div>
        )}
      </div>

      {/* Footer */}
      <footer className="px-6 py-3 bg-gray-900 border-t border-gray-800 text-center text-xs text-gray-500">
        WebC - Write, compile, and run C programs in your browser using WebAssembly
      </footer>
    </div>
  );
}
