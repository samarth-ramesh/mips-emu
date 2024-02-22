export default function RegisterDisplay(props: { reg: Object }) {
  if (props.reg === undefined) {
    return <></>;
  }
  let arr = Array<number>(32).fill(0);
  Object.keys(props.reg).forEach((k: string) => {
    //@ts-ignore
    arr[parseInt(k)] = props.reg[k];
  });
  return <div className="flex flex-col items-center">
    <h2 className="font-bold text-xl">Register File</h2>
    <table>
      <tbody>
        {
          arr.map((v: number, i: number) => {
            return <tr key={i}><td className="">r{i}</td><td>:</td><td>0x{v.toString(16)}</td></tr>;
          })
        }
      </tbody>
    </table>
  </div>
}
