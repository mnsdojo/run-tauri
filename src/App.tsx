import { useEffect, useState } from "react";

import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import "./App.css";

function App() {
  const [command, setCommand] = useState("");
  const [output, setOutput] = useState<any>([]);
  const [isRunning, setIsRunning] = useState(false);



 
  
  async function run() {
    try {
      setIsRunning(true);
      setOutput([]);
      await invoke("run_cmd", { cmd: command });
    } catch (error: any) {
      setOutput((prev: any) => [...prev, `Error: ${error}`]);
    } finally {
      setIsRunning(false);
    }
  }

  useEffect(() => {
    const unlisten1 = listen("cmd-output", (event) => {
      setOutput((prev: any) => [...prev, event.payload]);
    });

    // Listen for command completion
    const unlisten2 = listen("cmd-finished", (event) => {
      setIsRunning(false);
      setOutput((prev:any) => [
        ...prev,
        `Command finished with ${event.payload ? "success" : "error"}`,
      ]);
    });

    return () => {
      unlisten1.then((fn) => fn());
      unlisten2.then((fn) => fn());
    };
  }, []);
  return (
    <main className="container">
      <div className="">
        <input
          type="text"
          value={command}
          onChange={(e) => setCommand(e.target.value)}
        />
        <br />
        <button onClick={run} disabled={isRunning}>
          {isRunning ? "Wait..." : "Run"}
        </button>
      </div>
      {output && <pre>{JSON.stringify(output, null, 2)}</pre>}
    </main>
  );
}

export default App;
