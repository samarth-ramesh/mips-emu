import { useEffect, useState } from "react";
import { run_prog } from "mips_be";
import { registerCallback, unregisterCallback } from "./regHandler";
import RegisterDisplay from "./RegisterDisplay";

export default function TextBox() {
  const [state, setState] = useState("");
  const [regFile, setRegFile] = useState<Object>({});
  useEffect(() => {
    const doUpdateRegFile = (nf: Array<number>) => {
      setRegFile(nf);
    };
    registerCallback(doUpdateRegFile);
    return () => {
      unregisterCallback(doUpdateRegFile);
    };
  }, []);
  const handleChange = (e: React.ChangeEvent<HTMLTextAreaElement>) => {
    setState(e.target.value);
  };

  const run = () => {
    run_prog(state);
  };

  return (
    <div className="flex-row flex gap-4 m-4 justify-start">
      <div className="flex flex-col gap-4 content-center items-center">
        <div>
          <h2 className="text-xl">Enter Code & Click Run</h2>
        </div>
        <div className="w-fit">
          <textarea className="border-gray-400 min-w-96 min-h-96 h-fit border-2 p-2" value={state} onChange={handleChange} />
        </div>
        <div>
          <button onClick={run} className="border-slate-500 p-2 rounded-lg bg-slate-200 hover:bg-slate-300 active:bg-slate-400 shadow-md shadow-slate-200 min-w-24">Run</button>
        </div>
      </div>
      <RegisterDisplay reg={regFile} />
    </div>
  );
}
