import {runFile} from './output.js'
import { textEditor } from './text-editor.js'
import { clearOutput } from './output.js'

export const headerButtonsID = 'header-buttons-id';

export const appendRunButton = () => {

  // button && text creation
  const runButton = document.createElement(`button`);
  runButton.className = "run-button";
  var text = document.createTextNode(`Run this program`);
  runButton.appendChild(text);

  // button logic
  runButton.onclick = async () => {
    const file = textEditor.state.doc.toString();
    const outputDoc = document.getElementById("output-id");

    // clear output on change of file.
    clearOutput();

    let output = runFile(file).then(() => {
      outputDoc.value = output;
    });
  };

  document.getElementById(headerButtonsID).appendChild(runButton);
}
