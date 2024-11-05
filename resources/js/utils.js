export const getFileContents = async (filename) => {
  return fetch(`resources/lox_files/${filename}`)
    .then(response => response.text());
}
