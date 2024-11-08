import {EditorView, basicSetup} from 'codemirror';
import {javascript} from "@codemirror/lang-javascript";
import {tags, Tag} from "@lezer/highlight"
import {HighlightStyle} from "@codemirror/language"
import {syntaxHighlighting} from "@codemirror/language"
import hello from '/lox_files/hello.lox?url'
import { getFileContents } from './explorer.js';

const printTag = Tag.define('print');
const myHighlightStyle = HighlightStyle.define([
  {tag: tags.keyword, color: "#FF6868"},
  {tag: tags.comment, color: "#059212", fontStyle: "italic"},
  {tag: tags.variableName, color: "#FFFFFF"},
  {tag: tags.name, color: "#6499E9"},
  {tag: tags.string, color: "#FFAD60"},
  {tag: printTag, color: "#6499E9"},
])

export let textEditor;

export const createTextEditor = async () => {
  if (textEditor && textEditor !== undefined)
    return;

  textEditor = new EditorView({
    doc: await getFileContents(hello),
    extensions: [
      basicSetup,
      javascript(),
      syntaxHighlighting(myHighlightStyle)
    ],
    mode: "javascript",
    parent: document.getElementById('text-editor-id')
  });

  textEditor.requestMeasure();
}