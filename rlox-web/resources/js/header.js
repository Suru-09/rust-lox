import {runFile} from './output.js'
import { textEditor } from './text-editor.js'

export const headerID = 'header-id';

export const appendRunButton = async () => {
  const runButton = document.createElement(`button`);
  runButton.className = "run-button";
  var text = document.createTextNode(`Run this program`);
  runButton.appendChild(text);
  runButton.onclick = async () => {
    const file = textEditor.state.doc.toString();
    let output = await runFile(file);
    console.log(`Output: ${output}`);

    const outputDoc = document.getElementById("output-id");
    outputDoc.value = output;
  };

  document.getElementById(headerID).appendChild(runButton);
}
