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

  const doParse = () => {
    run_prog(state);
  };

  return (
    <>
      <textarea value={state} onChange={handleChange} />
      <button onClick={doParse}>Parse</button>
      <RegisterDisplay regFile={regFile} />
    </>
  );
}
