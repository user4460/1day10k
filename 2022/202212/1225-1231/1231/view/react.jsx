//useStateとは、Reactの機能で、stateを使うためのもの
const [state, setState] = useState(initialState);
setState(newState);

function Counter({ initialCount }) {
  const [count, setCount] = useState(initialCount);
  return (
    <>
      Count: {count}
      <button onClick={() => setCount(initialCount)}>Reset</button>
      <button onClick={() => setCount((prevCount) => prevCount - 1)}>-</button>
      <button onClick={() => setCount((prevCount) => prevCount + 1)}>+</button>
    </>
  );
}

const [state, setState] = useState(() => {
  const initialState = someExpensiveComputation(props);
  return initialState;
});

//useEffectとは、Reactの機能で、副作用を使うためのもの
useEffect(didUpdate);

useEffect(() => {
  const subscription = props.source.subscribe();
  return () => {
    // Clean up the subscription
    subscription.unsubscribe();
  };
});

useEffect(() => {
  const subscription = props.source.subscribe();
  return () => {
    subscription.unsubscribe();
  };
}, [props.source]);

//useContextとは、Reactの機能で、コンテキストを使うためのもの
const value = useContext(MyContext);

//useReducerとは、Reactの機能で、reducerを使うためのもの
const [state, dispatch] = useReducer(reducer, initialArg, init);

const initialState = { count: 0 };

function reducer(state, action) {
  switch (action.type) {
    case "increment":
      return { count: state.count + 1 };
    case "decrement":
      return { count: state.count - 1 };
    default:
      throw new Error();
  }
}

function Counter() {
  const [state, dispatch] = useReducer(reducer, initialState);
  return (
    <>
      Count: {state.count}
      <button onClick={() => dispatch({ type: "decrement" })}>-</button>
      <button onClick={() => dispatch({ type: "increment" })}>+</button>
    </>
  );
}

//useCallbackとは、Reactの機能で、メモ化したコールバックを使うためのもの
const memoizedCallback = useCallback(() => {
  doSomething(a, b);
}, [a, b]);

//useMemoとは、Reactの機能で、メモ化した値を使うためのもの
const memoizedValue = useMemo(() => computeExpensiveValue(a, b), [a, b]);

//useRefとは、Reactの機能で、レンダー間で値を保持するためのもの
const refContainer = useRef(initialValue);

function TextInputWithFocusButton() {
  const inputEl = useRef(null);
  const onButtonClick = () => {
    // `current` points to the mounted text input element
    inputEl.current.focus();
  };
  return (
    <>
      <input ref={inputEl} type="text" />
      <button onClick={onButtonClick}>Focus the input</button>
    </>
  );
}

//useImperativeHandleとは、Reactの機能で、子コンポーネントに公開するインスタンス値をカスタマイズするためのもの
useImperativeHandle(ref, createHandle, [deps]);

//useLayoutEffectとは、Reactの機能で、レイアウトの変更を同期的に実行するためのもの
useLayoutEffect(didUpdate);

//useDebugValueとは、Reactの機能で、カスタムフックのデバッグ値を表示するためのもの
useDebugValue(date, date => date.toDateString()
)

//useDeferredValueとは、Reactの機能で、非同期に更新される値を表示するためのもの
const deferredValue = useDeferredValue(value);

