import init, { run_file } from '../../pkg/rlox_wasm.js'

export const run_file_at_path = (fileName) => {
  fetch(`resources/lox_files/${fileName}`)
    .then(response => response.text())
    .then((data) => {
      init().then(() => {
        const result = run_file(data);
        const output = result.get_output();
        const errors = result.get_errors();

        if (output)
          console.log(output);
        if (errors)
          console.log(errors);
      });
    });
};