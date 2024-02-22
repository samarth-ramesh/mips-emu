type Callback = (arr: Array<number>) => void;
let callbacks: Array<Callback> = [];

//@ts-ignore
function update_reg_file(arr: Array<number>) {
  console.log(arr);
  callbacks.forEach((cb) => cb(arr));
}

//@ts-ignore
window.update_reg_file = update_reg_file;

export function registerCallback(callback: Callback) {
  callbacks.push(callback);
}

export function unregisterCallback(callback: Callback) {
  callbacks = callbacks.filter((cb) => cb !== callback);
}
