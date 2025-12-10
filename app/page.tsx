'use client';

import { useState } from 'react';
import dynamic from 'next/dynamic';

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

export default function Home() {
  const [code, setCode] = useState(DEFAULT_CODE);
  const [output, setOutput] = useState('');
  const [isRunning, setIsRunning] = useState(false);
  const [theme, setTheme] = useState<'monokai' | 'github'>('monokai');
  const [vimMode, setVimMode] = useState(false);

  const handleRunCode = async () => {
    setIsRunning(true);
    setOutput('Compiling and running...\n');
    
    // TODO: Implement actual C compilation and execution
    // This is a placeholder for the backend API call
    setTimeout(() => {
      setOutput('Output:\nHello, World!\n\nProgram exited successfully.');
      setIsRunning(false);
    }, 1500);
  };

  const handleClearOutput = () => {
    setOutput('');
  };

  const toggleTheme = () => {
    setTheme(prev => prev === 'monokai' ? 'github' : 'monokai');
  };

  const toggleVimMode = () => {
    setVimMode(prev => !prev);
  };

  return (
    <div className="flex flex-col h-screen bg-gray-950">
      {/* Header */}
      <header className="flex items-center justify-between px-6 py-4 bg-gray-900 border-b border-gray-800">
        <div className="flex items-center gap-3">
          <div className="text-2xl font-bold text-white">C Compiler</div>
          <div className="px-3 py-1 bg-blue-600 text-white text-sm rounded-full">Online</div>
        </div>
        <div className="flex items-center gap-3">
          <button
            onClick={toggleVimMode}
            className={`px-4 py-2 rounded-lg transition-colors font-medium ${
              vimMode 
                ? 'bg-purple-600 hover:bg-purple-700 text-white' 
                : 'bg-gray-800 hover:bg-gray-700 text-white'
            }`}
          >
            Vim: {vimMode ? 'ON' : 'OFF'}
          </button>
          <button
            onClick={toggleTheme}
            className="px-4 py-2 bg-gray-800 hover:bg-gray-700 text-white rounded-lg transition-colors"
          >
            Theme: {theme === 'monokai' ? 'Dark' : 'Light'}
          </button>
          <button
            onClick={handleRunCode}
            disabled={isRunning}
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
      <div className="flex flex-1 overflow-hidden">
        {/* Editor Panel */}
        <div className="flex flex-col w-2/3 border-r border-gray-800">
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

        {/* Output Panel */}
        <div className="flex flex-col w-1/3 bg-gray-900">
          <div className="flex items-center justify-between px-4 py-2 bg-gray-800 border-b border-gray-700">
            <h2 className="text-sm font-semibold text-gray-300">Output</h2>
            <button
              onClick={handleClearOutput}
              className="px-3 py-1 text-xs bg-gray-700 hover:bg-gray-600 text-white rounded transition-colors"
            >
              Clear
            </button>
          </div>
          <div className="flex-1 overflow-auto p-4">
            <pre className="text-sm text-gray-300 font-mono whitespace-pre-wrap">
              {output || 'Click "Run Code" to see output here...'}
            </pre>
          </div>
        </div>
      </div>

      {/* Footer */}
      <footer className="px-6 py-3 bg-gray-900 border-t border-gray-800 text-center text-xs text-gray-500">
        C Online Compiler - Write, compile, and run C programs in your browser
      </footer>
    </div>
  );
}
