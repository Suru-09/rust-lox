export const getFileContents = async (filename: string) => {
  return fetch(`resources/lox_files/${filename}`)
    .then(response => response.text());
}
