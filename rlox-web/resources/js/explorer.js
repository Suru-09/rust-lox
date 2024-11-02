import { textEditor } from './text-editor.js'

const defaultFiles = [
  'linked_list.lox',
  'two_statements.lox'
];

const explorerDivID = 'explorer-id';

const getFileContents = async(filename) => {
  return fetch(`resources/lox_files/${filename}`)
    .then(response => response.text());
}

export const appendExplorerButtons = () => {
  defaultFiles.forEach((filename) => {
    const button = document.createElement(`button`);
    button.className = "explorer-button";
    var text = document.createTextNode(`${filename}`);
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

        // clear output on change of file.
        const outputDoc = document.getElementById("output-id");
        outputDoc.value = "";
      })
    };

    document.getElementById(explorerDivID).appendChild(button);
  });
}