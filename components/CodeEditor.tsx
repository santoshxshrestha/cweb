'use client';

import React, { useEffect, useRef } from 'react';
import AceEditor from 'react-ace';

// Import Ace Editor modes and themes
import 'ace-builds/src-noconflict/mode-c_cpp';
import 'ace-builds/src-noconflict/theme-monokai';
import 'ace-builds/src-noconflict/theme-github';
import 'ace-builds/src-noconflict/ext-language_tools';
import 'ace-builds/src-noconflict/ext-searchbox';
import 'ace-builds/src-noconflict/keybinding-vim';

interface CodeEditorProps {
  value: string;
  onChange: (value: string) => void;
  theme?: 'monokai' | 'github';
  readOnly?: boolean;
  vimMode?: boolean;
}

const CodeEditor: React.FC<CodeEditorProps> = ({
  value,
  onChange,
  theme = 'monokai',
  readOnly = false,
  vimMode = false,
}) => {
  const editorRef = useRef<AceEditor>(null);

  useEffect(() => {
    // Configure auto-completion for C programming
    if (editorRef.current && editorRef.current.editor) {
      const editor = editorRef.current.editor;
      
      // Enable auto-completion
      editor.setOptions({
        enableBasicAutocompletion: true,
        enableLiveAutocompletion: true,
        enableSnippets: true,
      });

      // Add custom C keywords and standard library functions for better auto-completion
      const langTools = require('ace-builds/src-noconflict/ext-language_tools');
      
      const cKeywords = [
        // C keywords
        'auto', 'break', 'case', 'char', 'const', 'continue', 'default', 'do',
        'double', 'else', 'enum', 'extern', 'float', 'for', 'goto', 'if',
        'int', 'long', 'register', 'return', 'short', 'signed', 'sizeof', 'static',
        'struct', 'switch', 'typedef', 'union', 'unsigned', 'void', 'volatile', 'while',
        
        // Common standard library functions
        'printf', 'scanf', 'fprintf', 'fscanf', 'sprintf', 'sscanf',
        'fgets', 'fputs', 'gets', 'puts',
        'fopen', 'fclose', 'fread', 'fwrite', 'fseek', 'ftell', 'rewind',
        'malloc', 'calloc', 'realloc', 'free',
        'strlen', 'strcpy', 'strncpy', 'strcat', 'strncat', 'strcmp', 'strncmp',
        'memcpy', 'memmove', 'memset', 'memcmp',
        'atoi', 'atof', 'atol', 'strtol', 'strtod',
        'abs', 'labs', 'fabs', 'pow', 'sqrt', 'exp', 'log', 'sin', 'cos', 'tan',
        'exit', 'abort', 'system', 'getenv',
        'rand', 'srand', 'time',
      ];

      const cCompletions = cKeywords.map(keyword => ({
        caption: keyword,
        value: keyword,
        meta: 'C',
        score: 1000,
      }));

      const customCompleter = {
        getCompletions: function(editor: any, session: any, pos: any, prefix: any, callback: any) {
          callback(null, cCompletions);
        },
      };

      langTools.addCompleter(customCompleter);
    }
  }, []);

  useEffect(() => {
    // Enable or disable Vim mode
    if (editorRef.current && editorRef.current.editor) {
      const editor = editorRef.current.editor;
      if (vimMode) {
        editor.setKeyboardHandler('ace/keyboard/vim');
      } else {
        editor.setKeyboardHandler(null);
      }
    }
  }, [vimMode]);

  return (
    <AceEditor
      ref={editorRef}
      mode="c_cpp"
      theme={theme}
      name="c-code-editor"
      onChange={onChange}
      value={value}
      width="100%"
      height="100%"
      fontSize={14}
      showPrintMargin={false}
      showGutter={true}
      highlightActiveLine={true}
      readOnly={readOnly}
      setOptions={{
        enableBasicAutocompletion: true,
        enableLiveAutocompletion: true,
        enableSnippets: true,
        showLineNumbers: true,
        tabSize: 4,
        useWorker: false,
      }}
      editorProps={{ $blockScrolling: true }}
    />
  );
};

export default CodeEditor;
