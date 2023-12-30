import React, { useState, useEffect } from 'react';
import ReactDOM from 'react-dom/client';
import { MantineProvider, Container, Title, Paper, Button, Group } from '@mantine/core';
import Editor, { useMonaco, OnChange } from '@monaco-editor/react';
import type { editor as EditorType } from 'monaco-editor';
import * as blockpipe from 'blockpipe-language';

function App() {
  const [code, setCode] = useState<string>('// Write your BlockPipe code here');
  const [result, setResult] = useState<string>('// Results will be shown here');
  const monaco = useMonaco();

  const handleEditorChange: OnChange = (value: string | undefined, _: EditorType.IModelContentChangedEvent) => {
    if (value !== undefined) {
      setCode(value);
    }
  };

  const handleRunClick = (): void => {
    // Execute BlockPipe code and update the result
    try {
      const output = blockpipe.wasm_interpret_from_string(code, undefined, true);
      setResult(output);
    } catch (error) {
      setResult(`Error: ${(error as Error).message}`);
    }
  };

  useEffect(() => {
    if (monaco) {
      monaco.languages.register({ id: 'blockpipe' });
      monaco.languages.setMonarchTokensProvider('blockpipe', {
        tokenizer: {
          root: [
            [/\(/, 'leftParen'],
            [/\)/, 'rightParen'],
            [/\{/, 'leftBrace'],
            [/\}/, 'rightBrace'],
            [/\|/, 'pipe'],
            [/\|\*/, 'pipeStar'],
            [/\:/, 'colon'],
            [/\$[a-z_][a-zA-Z0-9_]*/, 'identifier'],
            [/\"([^"\\]|\\.)*\"/, 'stringLiteral'],
            [/\b(T|F)\b/, 'booleanLiteral'],
            [/\b-?[0-9]+\b/, 'integerLiteral'],
            [/\b-?[0-9]+\.[0-9]+\b/, 'floatLiteral'],
            [/\btype\b/, 'keyword'],
            [/\bpaste\b/, 'keyword'],
          ],
        },
        autoClosingPairs: [
          { open: '(', close: ')' },
          { open: '{', close: '}' },
          { open: '[', close: ']' },
          { open: '"', close: '"' },
          // ... any other pairs you need
        ]
      });

      monaco.editor.defineTheme('blockpipetheme', {
        base: 'vs-dark',
        inherit: true,
        rules: [
          { token: 'leftParen', foreground: '#FFC0CB' },
          { token: 'rightParen', foreground: '#FFC0CB' },
          { token: 'leftBrace', foreground: '#ADD8E6' },
          { token: 'rightBrace', foreground: '#ADD8E6' },
          { token: 'pipe', foreground: '#98FB98' },
          { token: 'pipeStar', foreground: '#98FB98' },
          { token: 'colon', foreground: '#FFD700' },
          { token: 'identifier', foreground: '#7B68EE' },
          { token: 'stringLiteral', foreground: '#FFA07A' },
          { token: 'booleanLiteral', foreground: '#FF69B4' },
          { token: 'integerLiteral', foreground: '#FF4500' },
          { token: 'floatLiteral', foreground: '#DAA520' },
          { token: 'keyword', foreground: '#00CED1' },
        ],
        colors: {}
      });

      monaco.editor.setTheme('blockpipetheme');
    }
  }, [monaco]);

  return (
    <MantineProvider>
      <Container size="md" style={{ marginTop: '40px' }}>
        <Title order={1}>BlockPipe Editor</Title>

        <Group mt="md">
          <Button onClick={handleRunClick}>Play</Button>
        </Group>

        <Paper withBorder shadow="md" p="md" mt="md" style={{ height: '300px' }}>
          <Editor
            height="100%"
            defaultLanguage="blockpipe" // Change this to 'blockpipe' if you define a custom language
            value={code}
            onChange={handleEditorChange}
            theme="blockpipetheme"
            options = {{autoClosingBrackets: "always"}}
          />
        </Paper>

        <Title order={2} mt="md">Result</Title>

        <Paper withBorder shadow="md" p="md" mt="md" style={{ height: '300px' }}>
          <Editor
            height="100%"
            defaultLanguage="text" // Change this to your output language if needed
            value={result}
            theme="vs-dark"
            options={{ readOnly: true, autoClosingBrackets: "always" }}
          />
        </Paper>
      </Container>
    </MantineProvider>
  );
}

// Render the App component inside the root div
const root = ReactDOM.createRoot(document.getElementById('root') as HTMLElement);
root.render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
);
