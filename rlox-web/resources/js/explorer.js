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

const headerID = 'header-id';

const getFileContents = async(filename) => {
  return fetch(filename)
    .then(response => response.text());
}

export const appendExplorerButtons = () => {
  const explorerDropdown = document.createElement('div');
  explorerDropdown.className = "dropdown";

  const dropDownName = document.createElement("SPAN");
  dropDownName.className = "dropdown-span";
  dropDownName.innerText = "Choose file";
  explorerDropdown.appendChild(dropDownName);

  const dropDownContent = document.createElement('div');
  dropDownContent.className = "dropdown-content";
  explorerDropdown.appendChild(dropDownContent);

  defaultFiles.forEach((filename) => {
    const button = document.createElement(`button`);
    button.className = "explorer-button";
    var parts = filename.split("/");
    var filename_real = parts.pop();
    var text = document.createTextNode(`${filename_real}`);
    button.appendChild(text);

    button.onclick = () => {
      getFileContents(filename).then((contents) => {
        const transaction = textEditor.state.update({
          changes: {
            from: 0,
            to: textEditor.state.doc.length,
            insert: `${contents}`
          }
        });
        textEditor.dispatch(transaction)

        // clear the old output when loading a new file to interpret.
        clearOutput();
      })
    };

    dropDownContent.appendChild(button);
  });

  document.getElementById(headerID).appendChild(explorerDropdown);
}