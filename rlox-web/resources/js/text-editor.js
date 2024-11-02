import {EditorView, basicSetup} from 'codemirror';
import {syntaxHighlighting} from "@codemirror/language"
import {tags} from "@lezer/highlight"
import {HighlightStyle} from "@codemirror/language"

const myHighlightStyle = HighlightStyle.define([
  {tag: tags.keyword, color: "#fc6"},
  {tag: tags.comment, color: "#f5d", fontStyle: "italic"}
])

export let textEditor;

const fixedHeightEditor = EditorView.theme({
  "&": {width: "300px"},
  ".cm-scroller": {overflow: "auto"}
})


export const createTextEditor = () => {
  if (textEditor && textEditor !== undefined)
    return;

  EditorView.theme({
    "&": {width: "200px"},
    ".cm-scroller": {overflow: "auto"}
  })

  textEditor = new EditorView({
    doc: "hello",
    extensions: [
      basicSetup,
      syntaxHighlighting(myHighlightStyle)
    ],
    fixedHeightEditor,
    parent: document.getElementById('text-editor-id')
  });

  textEditor.requestMeasure();
}