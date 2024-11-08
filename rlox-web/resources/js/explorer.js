import { textEditor } from './text-editor.js'
import { clearOutput } from './output.js'

import hello from '/lox_files/hello.lox?url'
import linked_list from '/lox_files/linked_list.lox?url'
import two_statements from '/lox_files/two_statements.lox?url'

const defaultFiles = [
  hello,
  linked_list,
  two_statements
];

const headerButtonsID = 'header-buttons-id';

const getFileContents = async(filename) => {
  return fetch(filename)
    .then(response => response.text());
}

const getFileNameFromPath = (path) => {
  var parts = path.split("/");
  return parts.pop();
}

export const appendExplorerButtons = () => {
  const explorerDropdown = document.createElement('div');
  explorerDropdown.className = "dropdown";

  const dropDownSpan = document.createElement(`SPAN`);
  dropDownSpan.className = "dropdown-span";
  dropDownSpan.id = "dropdown-span-id";
  dropDownSpan.innerText = getFileNameFromPath(defaultFiles[0]);
  explorerDropdown.appendChild(dropDownSpan);

  const dropDownContent = document.createElement('div');
  dropDownContent.className = "dropdown-content";
  explorerDropdown.appendChild(dropDownContent);

  defaultFiles.forEach((filePath) => {
    const button = document.createElement(`button`);
    button.className = "explorer-button";
    var text = document.createTextNode(getFileNameFromPath(filePath));
    button.appendChild(text);

    button.onclick = () => {
      getFileContents(filePath).then((contents) => {
        const transaction = textEditor.state.update({
          changes: {
            from: 0,
            to: textEditor.state.doc.length,
            insert: `${contents}`
          }
        });
        textEditor.dispatch(transaction)

        // update the dropdown
        document.getElementById("dropdown-span-id").innerText = getFileNameFromPath(filePath);

        // clear the old output when loading a new file to interpret.
        clearOutput();
      })
    };

    dropDownContent.appendChild(button);
  });

  document.getElementById(headerButtonsID).appendChild(explorerDropdown);
}