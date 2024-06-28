import { useEffect, useState } from "react";
import "./App.css";
import { invoke } from "@tauri-apps/api";

const readFile = async () => {
  try {
    const result = await invoke<string>("read_file", {});
    return result;
  } catch (error) {
    console.log(error);
    return "There was an error in getting the file content pls refresh";
  }
};

function App() {
  const [value, setValue] = useState("loading...");
  useEffect(() => {
    readFile().then((res) => setValue(res));
    console.log("iwas here..");
  }, []);

  const handleSave = (event: React.MouseEvent<HTMLButtonElement>) => {
    console.log(event, "event");
    event.preventDefault();
    invoke<string>("write_to_file", { content: value }).then((result) =>
      console.log(result)
    );
  };

  return (
    <>
      <textarea value={value} onChange={(e) => setValue(e.target.value)} />
      <br />
      <button type="button" onClick={(e) => handleSave(e)}>
        Save
      </button>
    </>
  );
}

export default App;
