import { useState } from "react";
import { run_prog } from "mips_be";
export default function TextBox() {
  const [state, setState] = useState("");

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
    </>
  );
}
