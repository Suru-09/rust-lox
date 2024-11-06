import {runFile} from './output.js'
import { textEditor } from './text-editor.js'
import { clearOutput } from './output.js'

export const headerID = 'header-id';

export const appendRunButton = () => {

  // button && text creation
  const runButton = document.createElement(`button`);
  runButton.className = "run-button";
  var text = document.createTextNode(`Run this program`);
  runButton.appendChild(text);

  // button logic
  runButton.onclick = () => {
    const file = textEditor.state.doc.toString();
    const outputDoc = document.getElementById("output-id");

    // clear output on change of file.
    clearOutput();

    let output = runFile(file);
    // console.log(`Output: ${output}`);

    outputDoc.value = output;
  };

  document.getElementById(headerID).appendChild(runButton);
}
