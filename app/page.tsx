'use client';

import { useState, useEffect, useRef } from 'react';
import dynamic from 'next/dynamic';
import { initWasm, compileAndRunC, provideInput } from '@/lib/wasmLoader';

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

  // Input state
  const [needsInput, setNeedsInput] = useState(false);
  const [inputPrompt, setInputPrompt] = useState('');
  const [userInput, setUserInput] = useState('');
  const inputRef = useRef<HTMLInputElement>(null);

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

  // Listen for Ctrl+Enter to run code
  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if (e.ctrlKey && e.key === 'Enter') {
        e.preventDefault();
        handleRunCode();
      }
    };

    document.addEventListener('keydown', handleKeyDown);
    return () => document.removeEventListener('keydown', handleKeyDown);
  }, [code, wasmReady, isRunning]); // Dependencies needed for handleRunCode

  const handleRunCode = async () => {
    if (!wasmReady) {
      setOutput('Error: Compiler is still loading. Please wait...');
      return;
    }

    setIsRunning(true);
    setOutput('Compiling and running...\n');
    setNeedsInput(false);
    
    // Show output panel when running code
    if (!isOutputVisible) {
      setIsOutputVisible(true);
    }
    
    try {
      // Compile and run the C code using WASM
      const result = await compileAndRunC(code);
      
      if (result.success) {
        setOutput(`Output:\n${result.output}\n\nProgram completed successfully.`);
      } else if (result.needs_input) {
        // Program needs input
        setNeedsInput(true);
        setInputPrompt(result.needs_input);
        setOutput(result.output || 'Waiting for input...');
        setTimeout(() => inputRef.current?.focus(), 100);
      } else {
        setOutput(`Compilation Error:\n${result.error || 'Unknown error'}`);
      }
    } catch (error) {
      setOutput(`Error: ${error}\n\nPlease check your code and try again.`);
    } finally {
      if (!needsInput) {
        setIsRunning(false);
      }
    }
  };

  const handleProvideInput = async (e: React.FormEvent) => {
    e.preventDefault();
    
    if (!userInput.trim()) {
      return;
    }
    
    // Append input to output
    setOutput(prev => prev + `\n> ${userInput}\n`);
    
    try {
      const result = await provideInput(userInput);
      
      if (result.success) {
        setOutput(prev => prev + result.output + '\n\nProgram completed successfully.');
        setNeedsInput(false);
        setIsRunning(false);
      } else if (result.needs_input) {
        // Program needs more input
        setInputPrompt(result.needs_input);
        setOutput(prev => prev + (result.output || ''));
        setTimeout(() => inputRef.current?.focus(), 100);
      } else {
        setOutput(prev => prev + `\nError: ${result.error || 'Unknown error'}`);
        setNeedsInput(false);
        setIsRunning(false);
      }
      
      setUserInput('');
    } catch (error) {
      setOutput(prev => prev + `\nError: ${error}`);
      setNeedsInput(false);
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
    <div className="flex h-screen bg-gray-950">
      {/* Main Content Area */}
      <div ref={containerRef} className="flex flex-1 overflow-hidden">
        {/* Editor Panel */}
        <div 
          className="flex flex-col border-r border-gray-800 transition-all duration-200"
          style={{ width: isOutputVisible ? `${editorWidth}%` : '100%' }}
        >
          <div className="flex items-center justify-between px-3 py-1 bg-gray-900 border-b border-gray-800">
            <div className="flex items-center gap-2">
              <div className="text-lg font-bold text-white">CWeb</div>
              <div 
                className={`w-2 h-2 rounded-full ${wasmReady ? 'bg-green-500 shadow-lg shadow-green-500/50' : 'bg-red-500 shadow-lg shadow-red-500/50'}`}
                title={wasmReady ? 'Compiler ready' : 'Loading compiler...'}
              />
              <span className="text-xs text-gray-500">main.c</span>
            </div>
            <div className="flex items-center gap-2">
              {/* Theme Selector */}
              <select
                value={theme}
                onChange={(e) => setTheme(e.target.value as EditorTheme)}
                className="px-2 py-0.5 text-xs bg-gray-800 hover:bg-gray-700 text-white rounded transition-colors cursor-pointer focus:outline-none focus:ring-1 focus:ring-blue-500"
                title="Select theme"
              >
                {EDITOR_THEMES.map(t => (
                  <option key={t.value} value={t.value}>{t.label}</option>
                ))}
              </select>
              {vimMode && (
                <div className="px-2 py-0.5 bg-purple-600 text-white text-xs rounded">
                  VIM
                </div>
              )}
              <div className="text-xs text-gray-500">
                Ctrl+Space: autocomplete
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
            <div className="flex items-center justify-between px-3 py-1 bg-gray-800 border-b border-gray-700">
              <h2 className="text-xs font-semibold text-gray-300">Output</h2>
              <div className="flex gap-1">
                <button
                  onClick={handleClearOutput}
                  className="px-2 py-0.5 text-xs bg-gray-700 hover:bg-gray-600 text-white rounded transition-colors"
                >
                  Clear
                </button>
                <button
                  onClick={toggleOutputPanel}
                  className="px-2 py-0.5 text-xs bg-gray-700 hover:bg-gray-600 text-white rounded transition-colors"
                  title="Close output panel"
                >
                  ✕
                </button>
              </div>
            </div>
            <div className="flex-1 overflow-auto p-3">
              <pre className="text-xs text-gray-300 font-mono whitespace-pre-wrap">
                {output || 'Click "Run" to see output here...'}
              </pre>
              
              {/* Input Form */}
              {needsInput && (
                <form onSubmit={handleProvideInput} className="mt-3 flex gap-2">
                  <input
                    ref={inputRef}
                    type="text"
                    value={userInput}
                    onChange={(e) => setUserInput(e.target.value)}
                    placeholder={inputPrompt || 'Enter input...'}
                    className="flex-1 px-2 py-1 text-xs bg-gray-800 text-white border border-gray-600 rounded focus:outline-none focus:ring-1 focus:ring-blue-500"
                    autoFocus
                  />
                  <button
                    type="submit"
                    className="px-3 py-1 text-xs bg-blue-600 hover:bg-blue-700 text-white rounded transition-colors"
                  >
                    Submit
                  </button>
                </form>
              )}
            </div>
          </div>
        )}
      </div>

      {/* Right Control Panel */}
      <div className="flex flex-col gap-0.5 p-0.5 bg-gray-900 border-l border-gray-800">
        {/* Run Button */}
        <button
          onClick={handleRunCode}
          disabled={isRunning || !wasmReady}
          className="px-1.5 py-2 text-xs bg-green-600 hover:bg-green-700 disabled:bg-gray-600 disabled:cursor-not-allowed text-white font-semibold rounded transition-colors flex flex-col items-center gap-0.5 min-w-[48px]"
          title="Run code (Ctrl+Enter)"
        >
          {isRunning ? (
            <>
              <span className="inline-block w-3 h-3 border-2 border-white border-t-transparent rounded-full animate-spin" />
              <span className="text-[9px]">Run...</span>
            </>
          ) : (
            <>
              <span className="text-sm">▶</span>
              <span className="text-[9px]">Run</span>
            </>
          )}
        </button>

        {/* Vim Mode Toggle */}
        <button
          onClick={toggleVimMode}
          className={`px-1.5 py-2 text-[9px] rounded transition-colors flex flex-col items-center gap-0.5 ${
            vimMode 
              ? 'bg-purple-600 hover:bg-purple-700 text-white' 
              : 'bg-gray-800 hover:bg-gray-700 text-white'
          }`}
          title="Toggle Vim mode"
        >
          <svg className="w-3 h-3" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
            <rect x="3" y="3" width="18" height="18" rx="2" />
            <path d="M9 9l3 3-3 3M15 12h-3" />
          </svg>
          <span>Vim</span>
        </button>

        {/* Output Panel Toggle */}
        <button
          onClick={toggleOutputPanel}
          className={`px-1.5 py-2 text-[9px] rounded transition-colors flex flex-col items-center gap-0.5 ${
            isOutputVisible 
              ? 'bg-blue-600 hover:bg-blue-700 text-white' 
              : 'bg-gray-800 hover:bg-gray-700 text-white'
          }`}
          title="Toggle output panel"
        >
          <svg className="w-3 h-3" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
            <rect x="2" y="4" width="20" height="16" rx="2" />
            <path d="M6 8l4 4-4 4M12 16h6" />
          </svg>
          <span>Out</span>
        </button>

        {/* Fullscreen Toggle */}
        <button
          onClick={toggleFullscreen}
          className="px-1.5 py-2 text-[9px] bg-gray-800 hover:bg-gray-700 text-white rounded transition-colors flex flex-col items-center gap-0.5"
          title="Toggle fullscreen"
        >
          {isFullscreen ? (
            <svg className="w-3 h-3" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
              <path d="M8 3v3a2 2 0 0 1-2 2H3M21 8h-3a2 2 0 0 1-2-2V3M3 16h3a2 2 0 0 1 2 2v3M16 21v-3a2 2 0 0 1 2-2h3" />
            </svg>
          ) : (
            <svg className="w-3 h-3" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
              <path d="M3 3h6v6M21 3h-6v6M21 21h-6v-6M3 21h6v-6" />
            </svg>
          )}
          <span>{isFullscreen ? 'Exit' : 'Full'}</span>
        </button>
      </div>
    </div>
  );
}
