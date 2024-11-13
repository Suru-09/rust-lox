import { textEditor } from './text-editor'
import { clearOutput } from './output'

const hello = new URL('/lox_files/hello.lox', import.meta.url);
const linked_list = new URL('/lox_files/linked_list.lox', import.meta.url);
const two_statements = new URL('/lox_files/two_statements.lox', import.meta.url);

const defaultFiles = [
  hello,
  linked_list,
  two_statements
];

const headerButtonsID = 'header-buttons-id';

export const getFileContents = async(filename: string) => {
  return fetch(filename)
    .then(response => response.text());
}

const getFileNameFromPath = (path: string) => {
  var parts = path.split("/");
  return parts.pop() as string;
}

export const appendExplorerButtons = () => {
  const explorerDropdown = document.createElement('div');
  explorerDropdown.className = "dropdown";

  // dropdown support
  const dropDownSpan = document.createElement(`SPAN`);
  dropDownSpan.className = "dropdown-span";
  dropDownSpan.id = "dropdown-span-id";

  // if
  dropDownSpan.innerText = getFileNameFromPath(defaultFiles[0].toString());
  explorerDropdown.appendChild(dropDownSpan);

  const dropDownContent = document.createElement('div');
  dropDownContent.className = "dropdown-content";
  explorerDropdown.appendChild(dropDownContent);

  defaultFiles.forEach((filePath) => {
    const button = document.createElement(`button`);
    button.className = "explorer-button";
    var text = document.createTextNode(getFileNameFromPath(filePath.toString()));
    button.appendChild(text);

    button.onclick = () => {
      getFileContents(filePath.toString()).then((contents) => {
        const transaction = textEditor.state.update({
          changes: {
            from: 0,
            to: textEditor.state.doc.length,
            insert: `${contents}`
          }
        });
        textEditor.dispatch(transaction)

        // update the dropdown
        const dropdown : HTMLSpanElement = document.getElementById("dropdown-span-id") as HTMLSpanElement;
        dropdown.innerText = getFileNameFromPath(filePath.toString());

        // clear the old output when loading a new file to interpret.
        clearOutput();
      })
    };

    dropDownContent.appendChild(button);
  });

  // add the header buttons to the header...
  const header : HTMLElement = document.getElementById(headerButtonsID) as HTMLElement;
  header.appendChild(explorerDropdown);
}