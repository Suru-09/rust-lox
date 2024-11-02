import {runFile} from './output.js'
import { textEditor } from './text-editor.js'

export const explorerID = 'explorer-id';

export const appendRunButton = () => {

  // button && text creation
  const runButton = document.createElement(`button`);
  runButton.className = "run-button";
  var text = document.createTextNode(`Run this program`);
  runButton.appendChild(text);

  // button logic
  runButton.onclick = () => {
    const file = textEditor.state.doc.toString();
    let output = runFile(file);
    // console.log(`Output: ${output}`);

    const outputDoc = document.getElementById("output-id");
    outputDoc.value = output;
  };

  document.getElementById(explorerID).appendChild(runButton);
}