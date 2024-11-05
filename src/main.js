import { appendExplorerButtons } from '../resources/js/explorer.js'
import { createTextEditor } from '../resources/js/text-editor.js'
import { appendRunButton } from '../resources/js/header.js'
import init, {init_interpreter} from '../pkg/rlox_wasm.js'

const clearOutput = () => {
  const outputDoc = document.getElementById("output-id");
  outputDoc.value = "";
};

window.onbeforeunload = clearOutput;

init().then(() => {
  init_interpreter();
  createTextEditor();
  appendRunButton();
  appendExplorerButtons();
});
