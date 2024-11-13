import {runFile} from './output'
import { textEditor } from './text-editor'
import { clearOutput } from './output'

export const headerButtonsID = 'header-buttons-id';

export const appendRunButton = async () => {

  // button && text creation
  const runButton = document.createElement(`button`);
  runButton.className = "run-button";
  var text = document.createTextNode(`Run this program`);
  runButton.appendChild(text);

  // button logic
  runButton.onclick = async () => {
    // clear output on change of file.
    clearOutput();
  
    const file = textEditor.state.doc.toString();
    const outputDoc = document.getElementById("output-id") as HTMLTextAreaElement;

    runFile(file).then((data) => {
      outputDoc.value = data;
    });
  };

  const headerButtons: any = document.getElementById(headerButtonsID);
  headerButtons.appendChild(runButton);
}
