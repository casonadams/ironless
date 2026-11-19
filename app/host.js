// The page's side of the app's `backup.download` post: saves the backup text as a file the browser downloads.
function download({ name, text }) {
  const url = URL.createObjectURL(new Blob([text], { type: "application/json" }));
  const link = document.createElement("a");
  link.href = url;
  link.download = name;
  link.style.display = "none";
  document.body.append(link);
  link.click();
  link.remove();
  setTimeout(() => URL.revokeObjectURL(url), 0);
}

export function createHost() {
  return {
    call(name) {
      throw new Error(`no host function ${name}`);
    },
    post(name, data) {
      if (name === "backup.download") download(data);
    },
  };
}
