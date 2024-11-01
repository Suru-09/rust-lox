import {EditorView, basicSetup} from 'codemirror';

export let textEditor;

export const createTextEditor = () => {
  if (textEditor && textEditor !== undefined)
    return;

  textEditor = new EditorView({
    doc: "hello",
    extensions: [
      basicSetup
    ],
    parent: document.getElementById('text-editor-id')
  });
}