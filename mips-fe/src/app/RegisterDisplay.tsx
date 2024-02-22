export default function RegisterDisplay(props: { reg: Object }) {
  if (props.reg === undefined) {
    return <></>;
  }
  let arr = Array<number>(32).fill(0);
  Object.keys(props.reg).forEach((k: string) => {
    //@ts-ignore
    arr[parseInt(k)] = props.reg[k];
  });
  return <>{JSON.stringify(arr)}</>;
}
