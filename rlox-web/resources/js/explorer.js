import {CodeMirror } from '@codemirror';

const defaultFiles = [
  'linked_list.lox',
  'two_statements.lox'
];

const explorerDivID = 'explorer-id';

const getFileContents = async (filename) => {
  return fetch(`resources/lox_files/${filename}`)
    .then(response => response.text())
    .then((data) => {
      data
    });
}
export const appendExplorerButtons = () => {
  defaultFiles.forEach((filename) => {
    const button = document.createElement(`button`);
    button.className = "explorer-button";
    var text = document.createTextNode(`${filename}`);
    button.appendChild(text);

    button.onclick = async () => {
      let textArea = document.querySelector('#text-editor-id');
      var editor = CodeMirror.fromTextArea(textArea);
      let fileContents = await getFileContents(filename);
      editor.getDoc().setValue(fileContents);
    };

    document.getElementById(explorerDivID).appendChild(button);
  });
}