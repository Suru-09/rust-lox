import{ run_file } from '../../pkg/rlox_wasm.js'

export const runFile = async (src: string) => {
  console.log(`I am running: ${src}`);
  const result = run_file(src);
  const output = result.get_output();
  const errors = result.get_errors();

  if (output)
    console.log(output);
  if (errors)
    console.log(errors);

  console.log(`Finishing running file...`);

  return output + errors;
}

export const clearOutput = () => {
  const outputDoc = document.getElementById("output-id") as HTMLTextAreaElement;
  outputDoc.value = "";
};

